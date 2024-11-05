use std::{
    fs, future::Future, io::Read, os::unix::process::ExitStatusExt, path::PathBuf, task::Poll,
    time::Duration,
};

use anyhow::Result;

use crate::utils::get_memory_usage;

#[derive(Debug)]
pub enum JudgeStatus {
    Accepted,
    WrongAnswer,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    RuntimeError,
    CompileError { message: String },
    SystemError { code: i32 },
    SegmentFault,
}

#[derive(Debug)]
pub struct JudgeResult {
    pub status: JudgeStatus,
    pub time_used: Duration,
    pub memory_used: u64,
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
                    let mut stdout = fs::File::open(&self.stdout_file)?;
                    let mut expected_out = fs::File::open(&self.expected_output_file)?;

                    let mut output = String::new();
                    let mut expected_output = String::new();
                    stdout.read_to_string(&mut output)?;
                    expected_out.read_to_string(&mut expected_output)?;

                    if output.trim_end_matches(|c: char| c.is_whitespace() || c == '\n')
                        == expected_output
                            .trim_end_matches(|c: char| c.is_whitespace() || c == '\n')
                    {
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
                    match status.signal() {
                        Some(libc::SIGSEGV) | Some(libc::SIGBUS) | Some(libc::SIGILL) => {
                            Poll::Ready(Ok(JudgeResult {
                                status: JudgeStatus::SegmentFault,
                                time_used: self.time_used,
                                memory_used: self.memory_used,
                            }))
                        }
                        Some(code) => Poll::Ready(Ok(JudgeResult {
                            status: JudgeStatus::SystemError { code },
                            time_used: self.time_used,
                            memory_used: self.memory_used,
                        })),
                        None => Poll::Ready(Ok(JudgeResult {
                            status: JudgeStatus::RuntimeError,
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
