use std::{path::PathBuf, time::Duration};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct JudgeOptions {
    pub time_limit: Duration,
    pub memory_limit: u64,
    pub fail_fast: bool,
    pub no_startup_limits: bool,
}

impl Default for JudgeOptions {
    fn default() -> Self {
        Self {
            time_limit: Duration::from_secs(1),
            memory_limit: 128 * 1024 * 1024,
            fail_fast: false,
            no_startup_limits: false,
        }
    }
}

impl JudgeOptions {
    pub fn fail_fast(mut self, fail_fast: bool) -> Self {
        self.fail_fast = fail_fast;
        self
    }

    pub fn no_fail_fast(self) -> Self {
        self.fail_fast(false)
    }

    pub fn no_startup_limits(mut self, no_startup_limits: bool) -> Self {
        self.no_startup_limits = no_startup_limits;
        self
    }
}

pub struct TestCase<I, O>
where
    I: Into<PathBuf>,
    O: Into<PathBuf>,
{
    pub input_file: I,
    pub expected_output_file: O,
}
