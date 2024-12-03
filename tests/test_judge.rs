use std::time::Duration;

use anyhow::Result;
use eval_stack::{case::run_test_cases, compile::Language, config::JudgeOptions};

#[tokio::test]
async fn test_rust_judge() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let workspace_path = current_dir.join("rust_workspace");
    let tests_path = current_dir.join("tests");

    let results = run_test_cases(
        Language::Rust,
        &workspace_path,
        &tests_path.join("test.rs"),
        Default::default(),
        vec![
            (tests_path.join("1.in"), tests_path.join("1.out")),
            (tests_path.join("2.in"), tests_path.join("2.out")),
        ],
        true,
    )
    .await?;

    for result in results {
        println!("{:?}", result);
        assert!(result.is_accepted())
    }

    let results = run_test_cases(
        Language::Rust,
        &workspace_path,
        &tests_path.join("test.rs"),
        JudgeOptions {
            time_limit: Duration::from_secs(1),
            memory_limit: 128 * 1024 * 1024,
            fail_fast: true,
            no_startup_limits: false,
            unsafe_mode: false,
        },
        vec![
            (tests_path.join("any.in"), tests_path.join("any.out")),
            (tests_path.join("1.out"), tests_path.join("1.in")),
        ],
        true,
    )
    .await?;

    for result in results {
        println!("{:?}", result);
        assert!(!result.is_accepted())
    }

    Ok(())
}

#[tokio::test]
async fn test_cpp_judge() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let workspace_path = current_dir.join("cpp_workspace");
    let tests_path = current_dir.join("tests");

    let results = run_test_cases(
        Language::CPP,
        &workspace_path,
        &tests_path.join("test.cpp"),
        Default::default(),
        vec![
            (tests_path.join("1.in"), tests_path.join("1.out")),
            (tests_path.join("2.in"), tests_path.join("2.out")),
        ],
        true,
    )
    .await?;

    for result in results {
        println!("{:?}", result);
        assert!(result.is_accepted())
    }

    Ok(())
}

#[tokio::test]
async fn test_c_judge() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let workspace_path = current_dir.join("c_workspace");
    let tests_path = current_dir.join("tests");

    let results = run_test_cases(
        Language::C,
        &workspace_path,
        &tests_path.join("test.c"),
        Default::default(),
        vec![
            (tests_path.join("1.in"), tests_path.join("1.out")),
            (tests_path.join("2.in"), tests_path.join("2.out")),
        ],
        true,
    )
    .await?;

    for result in results {
        println!("{:?}", result);
        assert!(result.is_accepted())
    }

    Ok(())
}

#[tokio::test]
async fn test_python_judge() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let workspace_path = current_dir.join("python_workspace");
    let tests_path = current_dir.join("tests");

    let results = run_test_cases(
        Language::Python,
        &workspace_path,
        &tests_path.join("test.py"),
        Default::default(),
        vec![
            (tests_path.join("1.in"), tests_path.join("1.out")),
            (tests_path.join("2.in"), tests_path.join("2.out")),
        ],
        true,
    )
    .await?;

    for result in results {
        println!("{:?}", result);
        assert!(result.is_accepted())
    }

    Ok(())
}

#[tokio::test]
async fn test_nodejs_judge() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let workspace_path = current_dir.join("nodejs_workspace");
    let tests_path = current_dir.join("tests");

    let results = run_test_cases(
        Language::NodeJs,
        &workspace_path,
        &tests_path.join("test.mjs"),
        JudgeOptions::default().no_startup_limits(true),
        vec![
            (tests_path.join("1.in"), tests_path.join("1.out")),
            (tests_path.join("2.in"), tests_path.join("2.out")),
        ],
        true,
    )
    .await?;

    for result in results {
        println!("{:?}", result);
        assert!(result.is_accepted())
    }

    Ok(())
}

#[tokio::test]
async fn test_golang_judge() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let workspace_path = current_dir.join("golang_workspace");
    let tests_path = current_dir.join("tests");

    let results = run_test_cases(
        Language::Golang,
        &workspace_path,
        &tests_path.join("test.go"),
        JudgeOptions::default(),
        vec![
            (tests_path.join("1.in"), tests_path.join("1.out")),
            (tests_path.join("2.in"), tests_path.join("2.out")),
        ],
        true,
    )
    .await?;

    for result in results {
        println!("{:?}", result);
        assert!(result.is_accepted())
    }

    Ok(())
}

#[tokio::test]
async fn test_java_judge() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let workspace_path = current_dir.join("java_workspace");
    let tests_path = current_dir.join("tests");

    let results = run_test_cases(
        Language::Java,
        &workspace_path,
        &tests_path.join("test.java"),
        JudgeOptions::default().no_startup_limits(true),
        vec![
            (tests_path.join("1.in"), tests_path.join("1.out")),
            (tests_path.join("2.in"), tests_path.join("2.out")),
        ],
        true,
    )
    .await?;

    for result in results {
        println!("{:?}", result);
        assert!(result.is_accepted())
    }

    Ok(())
}
