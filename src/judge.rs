use std::{
    fs,
    future::Future,
    io::{BufRead, BufReader, Read},
    os::unix::process::ExitStatusExt,
    path::PathBuf,
    task::Poll,
    time::Duration,
};

use anyhow::Result;

use crate::utils::get_memory_usage;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum JudgeStatus {
    Accepted,
    WrongAnswer,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    RuntimeError {
        code: i32,
        stderr: String,
    },
    CompileError {
        message: String,
    },
    SystemError {
        code: i32,
        stderr: String,
        signal: i32,
    },
    SegmentFault {
        code: i32,
        stderr: String,
    },
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct JudgeResult {
    pub status: JudgeStatus,
    pub time_used: Duration,
    pub memory_used: u64,
}

impl Default for JudgeResult {
    fn default() -> Self {
        Self {
            status: JudgeStatus::Accepted,
            time_used: Duration::from_secs(0),
            memory_used: 0,
        }
    }
}

impl JudgeResult {
    pub fn is_accepted(&self) -> bool {
        matches!(self.status, JudgeStatus::Accepted)
    }

    pub fn is_wrong_answer(&self) -> bool {
        !self.is_accepted()
    }
}

pub struct Judge {
    pub child: std::process::Child,
    pub id: u32,
    pub time_limit: Duration,
    pub memory_limit: u64,
    pub instant: tokio::time::Instant,
    pub memory_used: u64,
    pub time_used: Duration,
    pub stdout_file: PathBuf,
    pub expected_output_file: PathBuf,
}

impl Future for Judge {
    type Output = Result<JudgeResult>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match self.child.try_wait()? {
            Some(status) => {
                self.time_used = self.instant.elapsed();
                drop(self.child.stdin.take());
                drop(self.child.stdout.take());
                if status.success() {
                    let stdout = BufReader::new(fs::File::open(&self.stdout_file)?);
                    let expected_out = BufReader::new(fs::File::open(&self.expected_output_file)?);

                    let mut stdout_lines = stdout.lines();
                    let mut expected_out_lines = expected_out.lines();

                    let matched = loop {
                        match (stdout_lines.next(), expected_out_lines.next()) {
                            (None, None) => break true,
                            (Some(output), None) => {
                                if output?
                                    .trim_end_matches(|c: char| c.is_whitespace() || c == '\n')
                                    != ""
                                {
                                    break false;
                                }
                            }
                            (None, Some(expected_output)) => {
                                if expected_output?
                                    .trim_end_matches(|c: char| c.is_whitespace() || c == '\n')
                                    != ""
                                {
                                    break false;
                                }
                            }
                            (Some(output), Some(expected_output)) => {
                                if output?
                                    .trim_end_matches(|c: char| c.is_whitespace() || c == '\n')
                                    != expected_output?
                                        .trim_end_matches(|c: char| c.is_whitespace() || c == '\n')
                                {
                                    break false;
                                }
                            }
                        }
                    };

                    if matched {
                        Poll::Ready(Ok(JudgeResult {
                            status: JudgeStatus::Accepted,
                            time_used: self.time_used,
                            memory_used: self.memory_used,
                        }))
                    } else {
                        Poll::Ready(Ok(JudgeResult {
                            status: JudgeStatus::WrongAnswer,
                            time_used: self.time_used,
                            memory_used: self.memory_used,
                        }))
                    }
                } else {
                    let mut stderr = String::new();
                    let _ = self
                        .child
                        .stderr
                        .take()
                        .unwrap()
                        .read_to_string(&mut stderr);
                    let code = status.code().unwrap_or(-1);
                    match status.signal() {
                        Some(libc::SIGSEGV) | Some(libc::SIGBUS) | Some(libc::SIGILL) => {
                            Poll::Ready(Ok(JudgeResult {
                                status: JudgeStatus::SegmentFault { code, stderr },
                                time_used: self.time_used,
                                memory_used: self.memory_used,
                            }))
                        }
                        Some(signal) => Poll::Ready(Ok(JudgeResult {
                            status: JudgeStatus::SystemError {
                                code,
                                signal,
                                stderr,
                            },
                            time_used: self.time_used,
                            memory_used: self.memory_used,
                        })),
                        None => Poll::Ready(Ok(JudgeResult {
                            status: JudgeStatus::RuntimeError { code, stderr },
                            time_used: self.time_used,
                            memory_used: self.memory_used,
                        })),
                    }
                }
            }
            None => {
                if let Some(memory_used) = get_memory_usage(self.id) {
                    self.memory_used = memory_used.max(self.memory_used);
                };
                if self.memory_used > self.memory_limit {
                    self.child.kill()?;
                    self.time_used = self.instant.elapsed();
                    return Poll::Ready(Ok(JudgeResult {
                        status: JudgeStatus::MemoryLimitExceeded,
                        time_used: self.time_used,
                        memory_used: self.memory_used,
                    }));
                }
                if self.instant.elapsed() > self.time_limit {
                    self.child.kill()?;
                    self.time_used = self.instant.elapsed();
                    return Poll::Ready(Ok(JudgeResult {
                        status: JudgeStatus::TimeLimitExceeded,
                        time_used: self.time_used,
                        memory_used: self.memory_used,
                    }));
                }
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}
