use std::{path::PathBuf, time::Duration};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct JudgeOptions {
    /// Maximum time limit in seconds.
    pub time_limit: Duration,
    /// Maximum memory usage in bytes.
    pub memory_limit: u64,
    /// Stop running tests after the first failure.
    ///
    /// Enable this option for ICPC mode contests.
    /// Defaults to `true`.
    pub fail_fast: bool,
    /// Disable setting `RLIMIT_AS` and `seccomp` filter.
    pub no_startup_limits: bool,
    /// Run without kernel-level sand-boxing.
    pub unsafe_mode: bool,
}

impl Default for JudgeOptions {
    fn default() -> Self {
        Self {
            time_limit: Duration::from_secs(1),
            memory_limit: 128 * 1024 * 1024,
            fail_fast: true,
            no_startup_limits: false,
            unsafe_mode: false,
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
