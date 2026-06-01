use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct StudySession {
    pub id: String,
    pub started_at: String,
    pub ended_at: String,
    pub planned_duration_seconds: i64,
    pub actual_duration_seconds: i64,
    pub completed: bool,
    pub mode: String,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSessionPayload {
    pub id: String,
    pub started_at: String,
    pub ended_at: String,
    pub planned_duration_seconds: i64,
    pub actual_duration_seconds: i64,
    pub completed: bool,
    pub mode: String,
    pub note: Option<String>,
}
