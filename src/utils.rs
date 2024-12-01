use std::fs;

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
