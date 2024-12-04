use std::fs::create_dir_all;
use std::sync::LazyLock;
use std::time::Duration;

use anyhow::Result;
use futures::StreamExt;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::{engine::remote::ws::Client, Surreal};
use surrealdb::{Action, Notification};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::case::run_test_cases;
use crate::compile::Language;
use crate::config::JudgeOptions;
use crate::engine::models::Status;
use crate::judge::{JudgeResult, JudgeStatus};

use super::models::Submission;

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

const LIVE_QUERY: &str = r#"
LIVE SELECT *, problem.test_cases.{ input: input.path, output: output.path } AS test_cases
FROM submission
WHERE status = "in_queue"
"#;
pub async fn listen_for_submissions() -> Result<()> {
    DB.connect::<Ws>("127.0.0.1:5177").await?;
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    DB.use_ns("main").use_db("acm").await?;

    println!("Listening for submissions...");
    let mut stream = DB
        .query(LIVE_QUERY)
        .await?
        .stream::<Notification<Submission>>(0)?;

    while let Some(submission) = stream.next().await {
        tokio::spawn(handle_submission(submission));
    }

    Ok(())
}

pub async fn handle_submission(
    submission: surrealdb::Result<Notification<Submission>>,
) -> Result<()> {
    let submission = submission?;

    if !matches!(submission.action, Action::Create) {
        return Ok(());
    }

    let submission = submission.data;

    DB.query("UPDATE $submission SET status = $status")
        .bind(("submission", submission.id.clone()))
        .bind(("status", Status::Judging))
        .await?;

    let base_path = std::env::current_dir().unwrap();
    let workspace = base_path
        .join("workspaces")
        .join(submission.id.id.to_string());
    if !workspace.exists() {
        create_dir_all(&workspace)?;
    }

    let source_file_path = workspace.join(match submission.lang {
        Language::C => "main.c",
        Language::CPP => "main.cpp",
        Language::Java => "Main.java",
        Language::Python => "main.py",
        Language::Rust => "main.rs",
        Language::NodeJs => "main.js",
        Language::Golang => "main.go",
    });
    let mut file = File::create(&source_file_path).await?;
    file.write_all(submission.code.as_bytes()).await?;

    let results = run_test_cases(
        submission.lang,
        workspace,
        source_file_path,
        JudgeOptions {
            time_limit: Duration::from_secs(1),
            memory_limit: 128 * 1024 * 1024,
            fail_fast: true,
            no_startup_limits: false,
            unsafe_mode: true,
        },
        submission
            .test_cases
            .into_iter()
            .map(|_tc| ("tests/1.in", "tests/1.out"))
            .collect(),
        true,
    )
    .await?;

    let mut result = JudgeResult::default();
    for res in &results {
        result.memory_used = result.memory_used.max(res.memory_used);
        result.time_used = result.time_used.max(res.time_used);
        if !matches!(res.status, JudgeStatus::Accepted) {
            result = res.clone();
            break;
        };
    }

    DB.query(
        "UPDATE $submission SET status = $status, judge_details = $results, judge_result = $result",
    )
    .bind(("submission", submission.id.clone()))
    .bind(("status", Status::Ready))
    .bind(("results", results))
    .bind(("result", result))
    .await?;

    Ok(())
}
