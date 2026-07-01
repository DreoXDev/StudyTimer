use tauri::State;
use crate::AppState;
use crate::AppError;
use crate::models::session::{StudySession, CreateSessionPayload};

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StudyStats {
    pub today_minutes: i64,
    pub today_sessions_count: i64,
    pub week_minutes: i64,
}

#[tauri::command]
pub async fn create_session(
    payload: CreateSessionPayload,
    state: State<'_, AppState>,
) -> Result<StudySession, AppError> {
    // 1. Insert into study_sessions
    sqlx::query(
        "INSERT INTO study_sessions (id, started_at, ended_at, planned_duration_seconds, actual_duration_seconds, completed, mode, note)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&payload.id)
    .bind(&payload.started_at)
    .bind(&payload.ended_at)
    .bind(payload.planned_duration_seconds)
    .bind(payload.actual_duration_seconds)
    .bind(payload.completed)
    .bind(&payload.mode)
    .bind(&payload.note)
    .execute(&state.db)
    .await?;

    // 2. Insert into tracking_events
    let now = chrono::Utc::now().to_rfc3339();
    let metadata_json = format!("{{\"completed\":{},\"mode\":\"{}\"}}", payload.completed, payload.mode);
    sqlx::query(
        "INSERT INTO tracking_events (id, event_type, started_at, ended_at, duration_seconds, value, unit, source, note, metadata_json, created_at, updated_at, sync_status)
         VALUES (?, 'study_session', ?, ?, ?, ?, 'seconds', 'manual', ?, ?, ?, ?, 'pending')
         ON CONFLICT(id) DO UPDATE SET
            event_type = 'study_session',
            started_at = excluded.started_at,
            ended_at = excluded.ended_at,
            duration_seconds = excluded.duration_seconds,
            value = excluded.value,
            unit = excluded.unit,
            source = excluded.source,
            note = excluded.note,
            metadata_json = excluded.metadata_json,
            updated_at = excluded.updated_at,
            sync_status = 'pending'"
    )
    .bind(&payload.id)
    .bind(&payload.started_at)
    .bind(&payload.ended_at)
    .bind(payload.actual_duration_seconds)
    .bind(payload.actual_duration_seconds as f64)
    .bind(&payload.note)
    .bind(&metadata_json)
    .bind(&now)
    .bind(&now)
    .execute(&state.db)
    .await?;

    Ok(StudySession {
        id: payload.id,
        started_at: payload.started_at,
        ended_at: payload.ended_at,
        planned_duration_seconds: payload.planned_duration_seconds,
        actual_duration_seconds: payload.actual_duration_seconds,
        completed: payload.completed,
        mode: payload.mode,
        note: payload.note,
    })
}

#[tauri::command]
pub async fn list_sessions(
    limit: Option<i64>,
    state: State<'_, AppState>,
) -> Result<Vec<StudySession>, AppError> {
    let limit_val = limit.unwrap_or(20);

    let sessions = sqlx::query_as::<_, StudySession>(
        "SELECT id, started_at, ended_at, planned_duration_seconds, actual_duration_seconds, completed, mode, note
         FROM study_sessions
         ORDER BY started_at DESC
         LIMIT ?"
    )
    .bind(limit_val)
    .fetch_all(&state.db)
    .await?;

    Ok(sessions)
}

#[tauri::command]
pub async fn delete_session(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    // 1. Delete from study_sessions
    sqlx::query("DELETE FROM study_sessions WHERE id = ?")
        .bind(&id)
        .execute(&state.db)
        .await?;

    // 2. Soft-delete in tracking_events
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query("UPDATE tracking_events SET deleted_at = ?, updated_at = ?, sync_status = 'pending' WHERE id = ?")
        .bind(&now)
        .bind(&now)
        .bind(&id)
        .execute(&state.db)
        .await?;

    Ok(())
}

#[tauri::command]
pub async fn get_stats(
    today_start: String,
    week_start: String,
    state: State<'_, AppState>,
) -> Result<StudyStats, AppError> {
    // Query today stats
    let today_res: (Option<i64>, Option<i64>) = sqlx::query_as(
        "SELECT SUM(actual_duration_seconds), COUNT(id) FROM study_sessions WHERE started_at >= ? AND completed = 1"
    )
    .bind(&today_start)
    .fetch_one(&state.db)
    .await?;

    // Query week stats
    let week_res: (Option<i64>,) = sqlx::query_as(
        "SELECT SUM(actual_duration_seconds) FROM study_sessions WHERE started_at >= ? AND completed = 1"
    )
    .bind(&week_start)
    .fetch_one(&state.db)
    .await?;

    let today_seconds = today_res.0.unwrap_or(0);
    let today_sessions_count = today_res.1.unwrap_or(0);
    let week_seconds = week_res.0.unwrap_or(0);

    Ok(StudyStats {
        today_minutes: today_seconds / 60,
        today_sessions_count,
        week_minutes: week_seconds / 60,
    })
}
