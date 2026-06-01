use tauri::State;
use uuid::Uuid;
use chrono::Utc;
use crate::AppState;
use crate::models::habit::{Habit, HabitEntry, CreateHabitPayload, UpdateHabitPayload, UpsertHabitEntryPayload};

#[tauri::command]
pub async fn list_habits(state: State<'_, AppState>) -> Result<Vec<Habit>, String> {
    sqlx::query_as::<_, Habit>(
        "SELECT * FROM habits WHERE deleted_at IS NULL ORDER BY sort_order ASC, created_at ASC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_habit(
    state: State<'_, AppState>,
    payload: CreateHabitPayload,
) -> Result<Habit, String> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO habits (id, name, description, icon, color, tracking_type, unit, daily_goal, direction, is_archived, sort_order, created_at, updated_at, sync_status)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 0, 0, ?10, ?11, 'pending')"
    )
    .bind(&id)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(&payload.icon)
    .bind(&payload.color)
    .bind(&payload.tracking_type)
    .bind(&payload.unit)
    .bind(payload.daily_goal)
    .bind(&payload.direction)
    .bind(&now)
    .bind(&now)
    .execute(&state.db)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query_as::<_, Habit>("SELECT * FROM habits WHERE id = ?1")
        .bind(&id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_habit(
    state: State<'_, AppState>,
    payload: UpdateHabitPayload,
) -> Result<Habit, String> {
    let now = Utc::now().to_rfc3339();

    if let Some(name) = &payload.name {
        sqlx::query("UPDATE habits SET name = ?1, updated_at = ?2, sync_status = 'pending' WHERE id = ?3")
            .bind(name)
            .bind(&now)
            .bind(&payload.id)
            .execute(&state.db)
            .await
            .map_err(|e| e.to_string())?;
    }
    if payload.description.is_some() {
        sqlx::query("UPDATE habits SET description = ?1, updated_at = ?2, sync_status = 'pending' WHERE id = ?3")
            .bind(&payload.description)
            .bind(&now)
            .bind(&payload.id)
            .execute(&state.db)
            .await
            .map_err(|e| e.to_string())?;
    }
    if let Some(color) = &payload.color {
        sqlx::query("UPDATE habits SET color = ?1, updated_at = ?2, sync_status = 'pending' WHERE id = ?3")
            .bind(color)
            .bind(&now)
            .bind(&payload.id)
            .execute(&state.db)
            .await
            .map_err(|e| e.to_string())?;
    }
    if payload.unit.is_some() {
        sqlx::query("UPDATE habits SET unit = ?1, updated_at = ?2, sync_status = 'pending' WHERE id = ?3")
            .bind(&payload.unit)
            .bind(&now)
            .bind(&payload.id)
            .execute(&state.db)
            .await
            .map_err(|e| e.to_string())?;
    }
    if let Some(goal) = payload.daily_goal {
        sqlx::query("UPDATE habits SET daily_goal = ?1, updated_at = ?2, sync_status = 'pending' WHERE id = ?3")
            .bind(goal)
            .bind(&now)
            .bind(&payload.id)
            .execute(&state.db)
            .await
            .map_err(|e| e.to_string())?;
    }
    if let Some(direction) = &payload.direction {
        sqlx::query("UPDATE habits SET direction = ?1, updated_at = ?2, sync_status = 'pending' WHERE id = ?3")
            .bind(direction)
            .bind(&now)
            .bind(&payload.id)
            .execute(&state.db)
            .await
            .map_err(|e| e.to_string())?;
    }
    if let Some(archived) = payload.is_archived {
        sqlx::query("UPDATE habits SET is_archived = ?1, updated_at = ?2, sync_status = 'pending' WHERE id = ?3")
            .bind(archived as i64)
            .bind(&now)
            .bind(&payload.id)
            .execute(&state.db)
            .await
            .map_err(|e| e.to_string())?;
    }
    if let Some(order) = payload.sort_order {
        sqlx::query("UPDATE habits SET sort_order = ?1, updated_at = ?2, sync_status = 'pending' WHERE id = ?3")
            .bind(order)
            .bind(&now)
            .bind(&payload.id)
            .execute(&state.db)
            .await
            .map_err(|e| e.to_string())?;
    }

    sqlx::query_as::<_, Habit>("SELECT * FROM habits WHERE id = ?1")
        .bind(&payload.id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_habit(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let now = Utc::now().to_rfc3339();
    sqlx::query("UPDATE habits SET deleted_at = ?1, updated_at = ?2, sync_status = 'pending' WHERE id = ?3")
        .bind(&now)
        .bind(&now)
        .bind(&id)
        .execute(&state.db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn upsert_habit_entry(
    state: State<'_, AppState>,
    payload: UpsertHabitEntryPayload,
) -> Result<HabitEntry, String> {
    let now = Utc::now().to_rfc3339();
    let existing = sqlx::query_as::<_, HabitEntry>(
        "SELECT * FROM habit_entries WHERE habit_id = ?1 AND entry_date = ?2 AND deleted_at IS NULL"
    )
    .bind(&payload.habit_id)
    .bind(&payload.entry_date)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| e.to_string())?;

    if let Some(entry) = existing {
        sqlx::query(
            "UPDATE habit_entries SET value = ?1, note = ?2, updated_at = ?3, sync_status = 'pending' WHERE id = ?4"
        )
        .bind(payload.value)
        .bind(&payload.note)
        .bind(&now)
        .bind(&entry.id)
        .execute(&state.db)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query_as::<_, HabitEntry>("SELECT * FROM habit_entries WHERE id = ?1")
            .bind(&entry.id)
            .fetch_one(&state.db)
            .await
            .map_err(|e| e.to_string())
    } else {
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO habit_entries (id, habit_id, entry_date, value, note, created_at, updated_at, sync_status)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'pending')"
        )
        .bind(&id)
        .bind(&payload.habit_id)
        .bind(&payload.entry_date)
        .bind(payload.value)
        .bind(&payload.note)
        .bind(&now)
        .bind(&now)
        .execute(&state.db)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query_as::<_, HabitEntry>("SELECT * FROM habit_entries WHERE id = ?1")
            .bind(&id)
            .fetch_one(&state.db)
            .await
            .map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn increment_habit_entry(
    state: State<'_, AppState>,
    habit_id: String,
    entry_date: String,
    delta: f64,
) -> Result<HabitEntry, String> {
    let now = Utc::now().to_rfc3339();
    let existing = sqlx::query_as::<_, HabitEntry>(
        "SELECT * FROM habit_entries WHERE habit_id = ?1 AND entry_date = ?2 AND deleted_at IS NULL"
    )
    .bind(&habit_id)
    .bind(&entry_date)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| e.to_string())?;

    if let Some(entry) = existing {
        let new_val = (entry.value + delta).max(0.0);
        sqlx::query(
            "UPDATE habit_entries SET value = ?1, updated_at = ?2, sync_status = 'pending' WHERE id = ?3"
        )
        .bind(new_val)
        .bind(&now)
        .bind(&entry.id)
        .execute(&state.db)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query_as::<_, HabitEntry>("SELECT * FROM habit_entries WHERE id = ?1")
            .bind(&entry.id)
            .fetch_one(&state.db)
            .await
            .map_err(|e| e.to_string())
    } else {
        let new_val = delta.max(0.0);
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO habit_entries (id, habit_id, entry_date, value, created_at, updated_at, sync_status)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'pending')"
        )
        .bind(&id)
        .bind(&habit_id)
        .bind(&entry_date)
        .bind(new_val)
        .bind(&now)
        .bind(&now)
        .execute(&state.db)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query_as::<_, HabitEntry>("SELECT * FROM habit_entries WHERE id = ?1")
            .bind(&id)
            .fetch_one(&state.db)
            .await
            .map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn get_habit_entries_range(
    state: State<'_, AppState>,
    habit_id: String,
    start_date: String,
    end_date: String,
) -> Result<Vec<HabitEntry>, String> {
    sqlx::query_as::<_, HabitEntry>(
        "SELECT * FROM habit_entries WHERE habit_id = ?1 AND entry_date >= ?2 AND entry_date <= ?3 AND deleted_at IS NULL ORDER BY entry_date ASC"
    )
    .bind(&habit_id)
    .bind(&start_date)
    .bind(&end_date)
    .fetch_all(&state.db)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_habit_entries_range(
    state: State<'_, AppState>,
    start_date: String,
    end_date: String,
) -> Result<Vec<HabitEntry>, String> {
    sqlx::query_as::<_, HabitEntry>(
        "SELECT * FROM habit_entries WHERE entry_date >= ?1 AND entry_date <= ?2 AND deleted_at IS NULL ORDER BY entry_date ASC"
    )
    .bind(&start_date)
    .bind(&end_date)
    .fetch_all(&state.db)
    .await
    .map_err(|e| e.to_string())
}

/// Seeds the default "Smoking" habit if no habit with that name exists
pub async fn seed_default_habits(db: &sqlx::SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM habits WHERE name = 'Smoking' AND deleted_at IS NULL")
        .fetch_one(db)
        .await?;

    if count == 0 {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO habits (id, name, description, icon, color, tracking_type, unit, daily_goal, direction, is_archived, sort_order, created_at, updated_at, sync_status)
             VALUES (?1, 'Smoking', 'Il fumo uccide', '🚬', 'red', 'counter', 'cigarettes', NULL, 'limit', 0, 0, ?2, ?3, 'pending')"
        )
        .bind(&id)
        .bind(&now)
        .bind(&now)
        .execute(db)
        .await?;
    }
    Ok(())
}
