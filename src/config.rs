use std::{path::PathBuf, time::Duration};

#[derive(Debug, Clone)]
pub struct JudgeOptions {
    pub time_limit: Duration,
    pub memory_limit: u64,
}

pub struct TestCase<I, O>
where
    I: Into<PathBuf>,
    O: Into<PathBuf>,
{
    pub input_file: I,
    pub expected_output_file: O,
}
