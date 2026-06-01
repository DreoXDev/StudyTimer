use std::fs;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tauri::Manager;

pub async fn init_db<R: tauri::Runtime>(app: &tauri::App<R>) -> Result<SqlitePool, Box<dyn std::error::Error>> {
    let app_dir = app.path().app_local_data_dir()?;
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)?;
    }
    
    let db_path = app_dir.join("study_timer.db");
    
    // Create empty database file if it doesn't exist
    if !db_path.exists() {
        fs::File::create(&db_path)?;
    }
    
    let db_url = format!("sqlite:{}", db_path.to_string_lossy());
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    
    // Run SQL migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    Ok(pool)
}
