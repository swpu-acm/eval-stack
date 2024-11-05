use std::time::Duration;

use acmer_oj::{
    case::run_test_cases, compile::Language, config::JudgeOptions, utils::rerun_if_not_root,
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    rerun_if_not_root()?;

    let current_dir = std::env::current_dir()?;
    let workspace_path = current_dir.join("workspace");
    let tests_path = current_dir.join("tests");

    let results = run_test_cases(
        Language::CPP,
        &workspace_path,
        &tests_path.join("test.cpp"),
        JudgeOptions {
            time_limit: Duration::from_secs(1),
            memory_limit: 128 * 1024 * 1024,
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
    }

    Ok(())
}
