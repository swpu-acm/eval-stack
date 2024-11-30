use std::{path::PathBuf, time::Duration};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct JudgeOptions {
    pub time_limit: Duration,
    pub memory_limit: u64,
    pub fast_fail: bool,
}

pub struct TestCase<I, O>
where
    I: Into<PathBuf>,
    O: Into<PathBuf>,
{
    pub input_file: I,
    pub expected_output_file: O,
}
