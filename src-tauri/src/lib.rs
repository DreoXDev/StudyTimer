use tauri::Manager;

pub mod db;
pub mod models;
pub mod commands;
pub mod media;

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

            // System Tray Setup
            use tauri::menu::{Menu, MenuItem};
            use tauri::tray::{TrayIconBuilder, TrayIconEvent};

            let quit_i = MenuItem::with_id(app, "quit", "Esci", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Apri StudyTimer", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            let mut tray_builder = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "quit" => {
                            app.exit(0);
                        }
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                });

            if let Some(icon) = app.default_window_icon() {
                tray_builder = tray_builder.icon(icon.clone());
            }

            let _tray = tray_builder.build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
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
            // Media/System controls
            media::get_now_playing,
            media::media_play_pause,
            media::media_next,
            media::media_previous,
            // Tracking events commands
            commands::tracking::create_tracking_event,
            commands::tracking::list_tracking_events,
            commands::tracking::delete_tracking_event,
            commands::tracking::get_tracking_summary,
            commands::tracking::get_smoking_today_count,
            commands::tracking::add_cigarette,
            commands::tracking::remove_last_cigarette_today,
            // Export commands
            commands::export::export_tracking_data,
            // Sync commands
            commands::sync::update_event_sync_status,
            commands::sync::get_unsynced_events,
            commands::sync::upsert_synced_event,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
