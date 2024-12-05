use std::{path::PathBuf, time::Duration};

use anyhow::Result;
use tokio::fs::{create_dir_all, remove_dir_all};
use which::which;

use crate::{
    compile::{compile, Language},
    config::{JudgeOptions, TestCase},
    exec::execute,
    judge::{JudgeResult, JudgeStatus},
};

pub async fn run_test_cases<B, C>(
    language: Language,
    workspace: B,
    source_file_path: B,
    options: JudgeOptions,
    test_cases: Vec<(C, C)>,
    clean: bool,
) -> Result<Vec<JudgeResult>>
where
    B: Into<PathBuf>,
    C: Into<PathBuf>,
{
    let workspace: PathBuf = workspace.into();

    if !workspace.exists() {
        create_dir_all(&workspace).await?;
    }

    let source_file_path = Into::<PathBuf>::into(source_file_path)
        .to_string_lossy()
        .to_string();
    let exec_path = match &language {
        Language::Python => which("python")?,
        Language::NodeJs => which("deno")?,
        Language::Java => which("java")?,
        _ => workspace.join("out"),
    };

    if let Err(e) = compile(
        language,
        workspace.clone(),
        &source_file_path,
        exec_path.to_string_lossy(),
    )
    .await
    {
        if clean {
            if let Err(e) = remove_dir_all(workspace).await {
                anyhow::bail!("Failed to remove workspace: {}", e);
            }
        }
        return Ok(vec![JudgeResult {
            status: JudgeStatus::CompileError {
                message: e.to_string(),
            },
            time_used: Duration::default(),
            memory_used: 0,
        }]);
    };

    let py_args = vec![source_file_path.as_str()];
    let deno_args = vec![
        "run",
        format!("--v8-flags=--max-old-space-size={}", options.memory_limit).leak(),
        "--deny-read=*",
        "--deny-write=*",
        "--deny-env=*",
        "--deny-run=*",
        "--deny-ffi=*",
        source_file_path.as_str(),
    ];
    let java_args = vec!["Main"];
    let args = match language {
        Language::Python => Some(&py_args),
        Language::NodeJs => Some(&deno_args),
        Language::Java => Some(&java_args),
        _ => None,
    }
    .map(|v| &**v);

    let mut results = vec![];
    for (input_file, expected_output_file) in test_cases {
        let result = execute(
            &workspace,
            exec_path.to_string_lossy(),
            args,
            &options,
            TestCase {
                input_file: input_file.into(),
                expected_output_file: expected_output_file.into(),
            },
            workspace.join("test.out"),
        )
        .await?;
        if options.fail_fast && !matches!(result.status, JudgeStatus::Accepted) {
            results.push(result);
            break;
        }
        results.push(result);
    }

    if clean {
        if let Err(e) = remove_dir_all(workspace).await {
            anyhow::bail!("Failed to remove workspace: {}", e);
        }
    }
    Ok(results)
}
