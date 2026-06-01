use tauri::State;
use uuid::Uuid;
use chrono::Utc;
use crate::AppState;
use crate::AppError;
use crate::models::task::Task;

#[tauri::command]
pub async fn create_task(
    title: String,
    state: State<'_, AppState>,
) -> Result<Task, AppError> {
    let id = Uuid::new_v4().to_string();
    let created_at = Utc::now().to_rfc3339();

    // Find maximum sort_order to append to bottom
    let max_order: (Option<i64>,) = sqlx::query_as("SELECT MAX(sort_order) FROM tasks")
        .fetch_one(&state.db)
        .await?;
    let sort_order = max_order.0.unwrap_or(0) + 1;

    sqlx::query(
        "INSERT INTO tasks (id, title, completed, created_at, completed_at, sort_order)
         VALUES (?, ?, 0, ?, NULL, ?)"
    )
    .bind(&id)
    .bind(&title)
    .bind(&created_at)
    .bind(sort_order)
    .execute(&state.db)
    .await?;

    Ok(Task {
        id,
        title,
        completed: false,
        created_at,
        completed_at: None,
        sort_order,
    })
}

#[tauri::command]
pub async fn list_tasks(
    state: State<'_, AppState>,
) -> Result<Vec<Task>, AppError> {
    let tasks = sqlx::query_as::<_, Task>(
        "SELECT id, title, completed, created_at, completed_at, sort_order
         FROM tasks
         ORDER BY completed ASC, sort_order ASC, created_at DESC"
    )
    .fetch_all(&state.db)
    .await?;
    Ok(tasks)
}

#[tauri::command]
pub async fn update_task_completed(
    id: String,
    completed: bool,
    state: State<'_, AppState>,
) -> Result<Task, AppError> {
    let completed_at = if completed {
        Some(Utc::now().to_rfc3339())
    } else {
        None
    };

    sqlx::query("UPDATE tasks SET completed = ?, completed_at = ? WHERE id = ?")
        .bind(completed)
        .bind(&completed_at)
        .bind(&id)
        .execute(&state.db)
        .await?;

    let task = sqlx::query_as::<_, Task>(
        "SELECT id, title, completed, created_at, completed_at, sort_order FROM tasks WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&state.db)
    .await?;

    Ok(task)
}

#[tauri::command]
pub async fn delete_task(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn reorder_tasks(
    ids: Vec<String>,
    state: State<'_, AppState>,
) -> Result<Vec<Task>, AppError> {
    let mut tx = state.db.begin().await?;

    for (index, id) in ids.iter().enumerate() {
        sqlx::query("UPDATE tasks SET sort_order = ? WHERE id = ?")
            .bind(index as i64)
            .bind(id)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;

    let tasks = sqlx::query_as::<_, Task>(
        "SELECT id, title, completed, created_at, completed_at, sort_order
         FROM tasks
         ORDER BY completed ASC, sort_order ASC, created_at DESC"
    )
    .fetch_all(&state.db)
    .await?;

    Ok(tasks)
}
