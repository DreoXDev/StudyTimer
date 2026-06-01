use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Habit {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: String,
    pub tracking_type: String,
    pub unit: Option<String>,
    pub daily_goal: Option<f64>,
    pub direction: String,
    pub is_archived: i64,
    pub sort_order: i64,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub sync_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct HabitEntry {
    pub id: String,
    pub habit_id: String,
    pub entry_date: String,
    pub value: f64,
    pub note: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub sync_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateHabitPayload {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: String,
    pub tracking_type: String,
    pub unit: Option<String>,
    pub daily_goal: Option<f64>,
    pub direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateHabitPayload {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub unit: Option<String>,
    pub daily_goal: Option<f64>,
    pub direction: Option<String>,
    pub is_archived: Option<bool>,
    pub sort_order: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertHabitEntryPayload {
    pub habit_id: String,
    pub entry_date: String,
    pub value: f64,
    pub note: Option<String>,
}
