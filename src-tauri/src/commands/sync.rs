use tauri::State;
use crate::AppState;
use crate::AppError;
use crate::models::tracking::TrackingEvent;

#[tauri::command]
pub async fn update_event_sync_status(
    id: String,
    status: String,
    synced_at: Option<String>,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    sqlx::query("UPDATE tracking_events SET sync_status = ?, synced_at = ? WHERE id = ?")
        .bind(status)
        .bind(synced_at)
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn get_unsynced_events(
    state: State<'_, AppState>,
) -> Result<Vec<TrackingEvent>, AppError> {
    let events = sqlx::query_as::<_, TrackingEvent>(
        "SELECT id, event_type, started_at, ended_at, duration_seconds, value, unit, source, note, metadata_json, created_at, updated_at, deleted_at, synced_at, sync_status 
         FROM tracking_events 
         WHERE sync_status != 'synced'"
    )
    .fetch_all(&state.db)
    .await?;
    Ok(events)
}

#[tauri::command]
pub async fn upsert_synced_event(
    event: TrackingEvent,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    // 1. Upsert into tracking_events
    sqlx::query(
        "INSERT INTO tracking_events (id, event_type, started_at, ended_at, duration_seconds, value, unit, source, note, metadata_json, created_at, updated_at, deleted_at, synced_at, sync_status)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'synced')
         ON CONFLICT(id) DO UPDATE SET
            event_type = excluded.event_type,
            started_at = excluded.started_at,
            ended_at = excluded.ended_at,
            duration_seconds = excluded.duration_seconds,
            value = excluded.value,
            unit = excluded.unit,
            source = excluded.source,
            note = excluded.note,
            metadata_json = excluded.metadata_json,
            created_at = excluded.created_at,
            updated_at = excluded.updated_at,
            deleted_at = excluded.deleted_at,
            synced_at = excluded.synced_at,
            sync_status = 'synced'"
    )
    .bind(&event.id)
    .bind(&event.event_type)
    .bind(&event.started_at)
    .bind(&event.ended_at)
    .bind(event.duration_seconds)
    .bind(event.value)
    .bind(&event.unit)
    .bind(&event.source)
    .bind(&event.note)
    .bind(&event.metadata_json)
    .bind(&event.created_at)
    .bind(&event.updated_at)
    .bind(&event.deleted_at)
    .bind(&event.synced_at)
    .execute(&state.db)
    .await?;

    // 2. If it's a study session, keep the study_sessions table in sync
    if event.event_type == "study_session" {
        if event.deleted_at.is_some() {
            sqlx::query("DELETE FROM study_sessions WHERE id = ?")
                .bind(&event.id)
                .execute(&state.db)
                .await?;
        } else {
            let completed = event.metadata_json.as_ref().map(|m| m.contains("\"completed\":true")).unwrap_or(true);
            let mode = if event.metadata_json.as_ref().map(|m| m.contains("\"mode\":\"deep\"")).unwrap_or(false) { "deep" }
                       else if event.metadata_json.as_ref().map(|m| m.contains("\"mode\":\"break\"")).unwrap_or(false) { "break" }
                       else { "focus" };
            let dur = event.duration_seconds.unwrap_or(0);
            let ended_at_val = event.ended_at.as_ref().unwrap_or(&event.started_at);

            sqlx::query(
                "INSERT INTO study_sessions (id, started_at, ended_at, planned_duration_seconds, actual_duration_seconds, completed, mode, note)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                 ON CONFLICT(id) DO UPDATE SET
                    started_at = excluded.started_at,
                    ended_at = excluded.ended_at,
                    planned_duration_seconds = excluded.planned_duration_seconds,
                    actual_duration_seconds = excluded.actual_duration_seconds,
                    completed = excluded.completed,
                    mode = excluded.mode,
                    note = excluded.note"
            )
            .bind(&event.id)
            .bind(&event.started_at)
            .bind(ended_at_val)
            .bind(dur)
            .bind(dur)
            .bind(completed)
            .bind(mode)
            .bind(&event.note)
            .execute(&state.db)
            .await?;
        }
    }

    Ok(())
}
