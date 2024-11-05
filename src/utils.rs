use std::{
    env, fs,
    process::{self, Command},
};

use anyhow::Result;

pub fn get_memory_usage(pid: u32) -> Option<u64> {
    let statm_path = format!("/proc/{}/statm", pid);
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) as u64 };
    if let Ok(contents) = fs::read_to_string(statm_path) {
        let resident = contents.split_whitespace().nth(1)?;
        let memory = resident.parse::<u64>().ok()? * page_size;
        return Some(memory);
    }
    None
}

pub fn rerun_if_not_root() -> Result<()> {
    if !nix::unistd::getuid().is_root() {
        let args: Vec<String> = env::args().collect();
        let mut command = Command::new("sudo");
        command.args(&args);
        let status = command.status().expect("failed to execute process");
        if status.success() {
            process::exit(0);
        } else {
            process::exit(1);
        }
    };
    Ok(())
}
