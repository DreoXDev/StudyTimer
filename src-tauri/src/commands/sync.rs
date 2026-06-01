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
    Ok(())
}
