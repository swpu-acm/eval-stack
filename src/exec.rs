use std::{
    fs,
    os::unix::process::CommandExt,
    path::PathBuf,
    process::{Command, Stdio},
    time::Duration,
};

use anyhow::Result;

use crate::{
    config::{JudgeOptions, TestCase},
    judge::{Judge, JudgeResult},
};

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
        .stderr(Stdio::null());

    let memory_limit = options.memory_limit;
    unsafe {
        command.pre_exec(move || {
            use libc::{rlimit, setrlimit, RLIMIT_AS};

            for fd in 3..1024 {
                libc::close(fd);
            }
            if libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0 {
                panic!(
                    "Failed to disable grant of additional privileges: {}",
                    std::io::Error::last_os_error()
                )
            }
            if libc::unshare(libc::CLONE_NEWNS) != 0 {
                panic!(
                    "Failed to unshare namespace: {}",
                    std::io::Error::last_os_error()
                )
            }
            let limit = rlimit {
                rlim_cur: memory_limit,
                rlim_max: memory_limit,
            };
            if setrlimit(RLIMIT_AS, &limit) != 0 {
                panic!(
                    "Failed to set memory limit: {}",
                    std::io::Error::last_os_error()
                )
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
