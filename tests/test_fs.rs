use std::time::Duration;

use anyhow::Result;
use eval_stack::{case::run_test_cases, compile::Language, config::JudgeOptions};

#[tokio::test]
async fn test_fs() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let workspace_path = current_dir.join("fs_workspace");
    let tests_path = current_dir.join("tests");

    let results = run_test_cases(
        Language::Python,
        &workspace_path,
        &tests_path.join("fail_to_write.py"),
        JudgeOptions {
            time_limit: Duration::from_secs(1),
            memory_limit: 128 * 1024 * 1024,
            fail_fast: true,
            no_startup_limits: false,
        },
        vec![
            (tests_path.join("1.in"), tests_path.join("1.out")),
            (tests_path.join("2.in"), tests_path.join("2.out")),
        ],
        true,
    )
    .await?;

    for result in results {
        println!("{:?}", result);
        assert!(!result.is_accepted());
        assert!(matches!(
            result.status,
            eval_stack::judge::JudgeStatus::SystemError {
                code: -1,
                stderr: _stderr,
                signal: 31
            }
        ))
    }

    let results = run_test_cases(
        Language::NodeJs,
        &workspace_path,
        &tests_path.join("fail_to_write.mjs"),
        JudgeOptions {
            time_limit: Duration::from_secs(1),
            memory_limit: 128 * 1024 * 1024,
            fail_fast: true,
            no_startup_limits: true,
        },
        vec![
            (tests_path.join("1.in"), tests_path.join("1.out")),
            (tests_path.join("2.in"), tests_path.join("2.out")),
        ],
        true,
    )
    .await?;

    for result in results {
        println!("{:?}", result);
        assert!(!result.is_accepted());
        assert!(matches!(
            result.status,
            eval_stack::judge::JudgeStatus::RuntimeError {
                code: 1,
                stderr: _stderr
            }
        ))
    }

    Ok(())
}
