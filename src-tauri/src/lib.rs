use tauri::Manager;

pub mod db;
pub mod models;
pub mod commands;

pub struct AppState {
    pub db: sqlx::SqlitePool,
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize database synchronously on setup
            let pool = tauri::async_runtime::block_on(async {
                db::init_db(app).await
            }).expect("failed to initialize SQLite database");

            app.manage(AppState { db: pool });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Sessions commands
            commands::sessions::create_session,
            commands::sessions::list_sessions,
            commands::sessions::delete_session,
            commands::sessions::get_stats,
            // Tasks commands
            commands::tasks::create_task,
            commands::tasks::list_tasks,
            commands::tasks::update_task_completed,
            commands::tasks::delete_task,
            commands::tasks::reorder_tasks,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
