use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub completed: i64, // 0/1 in SQLite
    pub created_at: String,
    pub completed_at: Option<String>,
    pub sort_order: i64,
}
