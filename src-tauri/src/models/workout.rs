use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WorkoutTemplate {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub exercises_json: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub sync_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WorkoutLog {
    pub id: String,
    pub template_id: Option<String>,
    pub title: String,
    pub performed_at: String,
    pub duration_minutes: Option<i64>,
    pub calories: Option<i64>,
    pub exercises_json: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub sync_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkoutTemplatePayload {
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub exercises_json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkoutLogPayload {
    pub template_id: Option<String>,
    pub title: String,
    pub performed_at: String,
    pub duration_minutes: Option<i64>,
    pub calories: Option<i64>,
    pub exercises_json: String,
    pub notes: Option<String>,
}
