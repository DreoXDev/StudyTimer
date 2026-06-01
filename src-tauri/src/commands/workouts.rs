use tauri::State;
use uuid::Uuid;
use chrono::Utc;
use crate::AppState;
use crate::models::workout::{WorkoutTemplate, WorkoutLog, CreateWorkoutTemplatePayload, CreateWorkoutLogPayload};

#[tauri::command]
pub async fn list_workout_templates(state: State<'_, AppState>) -> Result<Vec<WorkoutTemplate>, String> {
    sqlx::query_as::<_, WorkoutTemplate>(
        "SELECT * FROM workout_templates WHERE deleted_at IS NULL ORDER BY created_at DESC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_workout_template(
    state: State<'_, AppState>,
    payload: CreateWorkoutTemplatePayload,
) -> Result<WorkoutTemplate, String> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO workout_templates (id, name, description, category, exercises_json, created_at, updated_at, sync_status)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'pending')"
    )
    .bind(&id)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(&payload.category)
    .bind(&payload.exercises_json)
    .bind(&now)
    .bind(&now)
    .execute(&state.db)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query_as::<_, WorkoutTemplate>("SELECT * FROM workout_templates WHERE id = ?1")
        .bind(&id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_workout_template(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let now = Utc::now().to_rfc3339();
    sqlx::query("UPDATE workout_templates SET deleted_at = ?1, updated_at = ?2, sync_status = 'pending' WHERE id = ?3")
        .bind(&now)
        .bind(&now)
        .bind(&id)
        .execute(&state.db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn list_workout_logs(
    state: State<'_, AppState>,
    limit: Option<i64>,
) -> Result<Vec<WorkoutLog>, String> {
    let limit = limit.unwrap_or(50);
    sqlx::query_as::<_, WorkoutLog>(
        "SELECT * FROM workout_logs WHERE deleted_at IS NULL ORDER BY performed_at DESC LIMIT ?1"
    )
    .bind(limit)
    .fetch_all(&state.db)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_workout_logs_range(
    state: State<'_, AppState>,
    start: String,
    end: String,
) -> Result<Vec<WorkoutLog>, String> {
    sqlx::query_as::<_, WorkoutLog>(
        "SELECT * FROM workout_logs WHERE deleted_at IS NULL AND performed_at >= ?1 AND performed_at <= ?2 ORDER BY performed_at DESC"
    )
    .bind(&start)
    .bind(&end)
    .fetch_all(&state.db)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_workout_log(
    state: State<'_, AppState>,
    payload: CreateWorkoutLogPayload,
) -> Result<WorkoutLog, String> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO workout_logs (id, template_id, title, performed_at, duration_minutes, calories, exercises_json, notes, created_at, updated_at, sync_status)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'pending')"
    )
    .bind(&id)
    .bind(&payload.template_id)
    .bind(&payload.title)
    .bind(&payload.performed_at)
    .bind(payload.duration_minutes)
    .bind(payload.calories)
    .bind(&payload.exercises_json)
    .bind(&payload.notes)
    .bind(&now)
    .bind(&now)
    .execute(&state.db)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query_as::<_, WorkoutLog>("SELECT * FROM workout_logs WHERE id = ?1")
        .bind(&id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_workout_log(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let now = Utc::now().to_rfc3339();
    sqlx::query("UPDATE workout_logs SET deleted_at = ?1, updated_at = ?2, sync_status = 'pending' WHERE id = ?3")
        .bind(&now)
        .bind(&now)
        .bind(&id)
        .execute(&state.db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
