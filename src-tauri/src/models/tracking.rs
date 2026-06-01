use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TrackingEvent {
    pub id: String,
    pub event_type: String,
    pub started_at: String,
    pub ended_at: Option<String>,
    pub duration_seconds: Option<i64>,
    pub value: Option<f64>,
    pub unit: Option<String>,
    pub source: String,
    pub note: Option<String>,
    pub metadata_json: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub synced_at: Option<String>,
    pub sync_status: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTrackingEventPayload {
    pub id: String,
    pub event_type: String,
    pub started_at: String,
    pub ended_at: Option<String>,
    pub duration_seconds: Option<i64>,
    pub value: Option<f64>,
    pub unit: Option<String>,
    pub source: Option<String>,
    pub note: Option<String>,
    pub metadata_json: Option<String>,
}
