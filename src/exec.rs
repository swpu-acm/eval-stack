use std::{
    fs,
    os::unix::process::CommandExt,
    path::PathBuf,
    process::{Command, Stdio},
    time::Duration,
};

use anyhow::Result;
use seccompiler::{
    BpfProgram, SeccompCmpArgLen, SeccompCmpOp, SeccompCondition, SeccompFilter, SeccompRule,
};

use crate::{
    config::{JudgeOptions, TestCase},
    judge::{Judge, JudgeResult},
};

pub fn seccomp_filter() -> anyhow::Result<BpfProgram> {
    Ok(SeccompFilter::new(
        vec![(
            libc::SYS_write,
            vec![SeccompRule::new(vec![
                SeccompCondition::new(0, SeccompCmpArgLen::Dword, SeccompCmpOp::Ne, 1)?,
                SeccompCondition::new(0, SeccompCmpArgLen::Dword, SeccompCmpOp::Ne, 2)?,
            ])?],
        )]
        .into_iter()
        .collect(),
        seccompiler::SeccompAction::Allow,
        seccompiler::SeccompAction::KillProcess,
        seccompiler::TargetArch::x86_64,
    )?
    .try_into()?)
}

pub async fn execute<'a, B, E, I, O>(
    base: B,
    exec_path: E,
    args: Option<&'a [&'a str]>,
    options: &'a JudgeOptions,
    case: TestCase<I, O>,
    output_file: O,
) -> Result<JudgeResult>
where
    B: Into<PathBuf>,
    E: AsRef<str>,
    I: Into<PathBuf>,
    O: Into<PathBuf>,
{
    let base_path = base.into();
    let input_file = case.input_file.into();
    let output_file = output_file.into();
    let expected_output_file = case.expected_output_file.into();

    let mut command = Command::new(exec_path.as_ref());
    if let Some(args) = args {
        command.args(args);
    }
    command
        .env_clear()
        .current_dir(base_path)
        .stdin(Stdio::from(fs::File::open(&input_file)?))
        .stdout(Stdio::from(fs::File::create(&output_file)?))
        .stderr(Stdio::piped());

    let no_sys_as_limits = options.no_startup_limits;
    let memory_limit = options.memory_limit;
    let time_limit = options.time_limit.as_secs();
    unsafe {
        command.pre_exec(move || {
            use libc::{rlimit, setrlimit};
            // Close all file descriptors except for stdin, stdout, and stderr
            for fd in 3..1024 {
                libc::close(fd);
            }
            // Prevent child from gaining new privileges
            if libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0 {
                panic!(
                    "Failed to disable grant of additional privileges: {}",
                    std::io::Error::last_os_error()
                )
            }
            // Unshare the mount namespace to prevent child from gaining new mounts
            if libc::unshare(libc::CLONE_NEWNS) != 0 {
                panic!(
                    "Failed to unshare namespace: {}",
                    std::io::Error::last_os_error()
                )
            }
            // Set memory limit
            if !no_sys_as_limits {
                let limit = rlimit {
                    rlim_cur: memory_limit,
                    rlim_max: memory_limit,
                };
                if setrlimit(libc::RLIMIT_AS, &limit) != 0 {
                    panic!(
                        "Failed to set memory limit: {}",
                        std::io::Error::last_os_error()
                    )
                }
                let filter = seccomp_filter().unwrap();
                seccompiler::apply_filter(&filter).unwrap();
            }
            // Set process limit
            let proc_limit = rlimit {
                rlim_cur: 0,
                rlim_max: 0,
            };
            if setrlimit(libc::RLIMIT_NPROC, &proc_limit) != 0 {
                return Err(std::io::Error::last_os_error());
            }
            // Set CPU time limit
            let cpu_limit = rlimit {
                rlim_cur: time_limit,
                rlim_max: time_limit,
            };
            if setrlimit(libc::RLIMIT_CPU, &cpu_limit) != 0 {
                return Err(std::io::Error::last_os_error());
            }
            // Disable core dumps
            if setrlimit(
                libc::RLIMIT_CORE,
                &rlimit {
                    rlim_cur: 0,
                    rlim_max: 0,
                },
            ) != 0
            {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        })
    };

    let instant = tokio::time::Instant::now();
    let child = command.spawn()?;

    let id = child.id();

    Judge {
        child,
        id,
        time_limit: options.time_limit,
        memory_limit: options.memory_limit,
        instant,
        memory_used: 0,
        time_used: Duration::from_secs(0),
        stdout_file: output_file,
        expected_output_file,
    }
    .await
}
