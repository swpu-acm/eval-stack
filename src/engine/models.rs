use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::compile::Language;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    InQueue,
    Judging,
    Ready,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCase {
    pub input: PathBuf,
    pub output: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Submission {
    pub id: Thing,

    pub lang: Language,
    pub test_cases: Vec<TestCase>,

    pub code: String,
    pub status: Status,
    pub creator: Thing,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
