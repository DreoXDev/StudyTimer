use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct NowPlaying {
    pub available: bool,
    pub source: Option<String>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub is_playing: bool,
    pub progress_ms: Option<i64>,
    pub duration_ms: Option<i64>,
}

#[cfg(target_os = "windows")]
mod win_media {
    use super::NowPlaying;
    use windows::Media::Control::{
        GlobalSystemMediaTransportControlsSessionManager,
        GlobalSystemMediaTransportControlsSessionPlaybackStatus,
    };

    pub async fn get_now_playing() -> Result<NowPlaying, String> {
        let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
            .map_err(|e| e.to_string())?
            .get()
            .map_err(|e| e.to_string())?;

        let session = match manager.GetCurrentSession() {
            Ok(s) => s,
            Err(_) => {
                return Ok(NowPlaying {
                    available: false,
                    source: None,
                    title: None,
                    artist: None,
                    album: None,
                    is_playing: false,
                    progress_ms: None,
                    duration_ms: None,
                });
            }
        };

        // Source ID (App name like spotify.exe)
        let raw_source = session.SourceAppUserModelId().ok().map(|s| s.to_string());
        // Clean source name
        let source = raw_source.map(|s| {
            let lower = s.to_lowercase();
            if lower.contains("spotify") {
                "Spotify".to_string()
            } else if lower.contains("chrome") {
                "Chrome (YouTube)".to_string()
            } else if lower.contains("edge") {
                "Edge".to_string()
            } else {
                s.split('.').next().unwrap_or(&s).to_string()
            }
        });

        let mut title = None;
        let mut artist = None;
        let mut album = None;

        if let Ok(op) = session.TryGetMediaPropertiesAsync() {
            if let Ok(props) = op.get() {
                title = props.Title().ok().map(|s| s.to_string());
                artist = props.Artist().ok().map(|s| s.to_string());
                album = props.AlbumTitle().ok().map(|s| s.to_string());
            }
        }

        let mut is_playing = false;
        if let Ok(info) = session.GetPlaybackInfo() {
            if let Ok(status) = info.PlaybackStatus() {
                is_playing = status == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing;
            }
        }

        let mut progress_ms = None;
        let mut duration_ms = None;
        if let Ok(timeline) = session.GetTimelineProperties() {
            if let Ok(pos) = timeline.Position() {
                progress_ms = Some(pos.Duration / 10000); // 100ns intervals to ms
            }
            if let Ok(end) = timeline.EndTime() {
                duration_ms = Some(end.Duration / 10000);
            }
        }

        Ok(NowPlaying {
            available: true,
            source,
            title,
            artist,
            album,
            is_playing,
            progress_ms,
            duration_ms,
        })
    }

    pub async fn play_pause() -> Result<(), String> {
        let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
            .map_err(|e| e.to_string())?.get().map_err(|e| e.to_string())?;
        if let Ok(session) = manager.GetCurrentSession() {
            if let Ok(info) = session.GetPlaybackInfo() {
                if let Ok(status) = info.PlaybackStatus() {
                    if status == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing {
                        let _ = session.TryPauseAsync().map_err(|e| e.to_string())?.get();
                    } else {
                        let _ = session.TryPlayAsync().map_err(|e| e.to_string())?.get();
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn next() -> Result<(), String> {
        let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
            .map_err(|e| e.to_string())?.get().map_err(|e| e.to_string())?;
        if let Ok(session) = manager.GetCurrentSession() {
            let _ = session.TrySkipNextAsync().map_err(|e| e.to_string())?.get();
        }
        Ok(())
    }

    pub async fn previous() -> Result<(), String> {
        let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
            .map_err(|e| e.to_string())?.get().map_err(|e| e.to_string())?;
        if let Ok(session) = manager.GetCurrentSession() {
            let _ = session.TrySkipPreviousAsync().map_err(|e| e.to_string())?.get();
        }
        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
mod win_media {
    use super::NowPlaying;

    pub async fn get_now_playing() -> Result<NowPlaying, String> {
        Ok(NowPlaying {
            available: false,
            source: None,
            title: None,
            artist: None,
            album: None,
            is_playing: false,
            progress_ms: None,
            duration_ms: None,
        })
    }

    pub async fn play_pause() -> Result<(), String> {
        Ok(())
    }

    pub async fn next() -> Result<(), String> {
        Ok(())
    }

    pub async fn previous() -> Result<(), String> {
        Ok(())
    }
}

// Exposed Tauri Commands
#[tauri::command]
pub async fn get_now_playing() -> Result<NowPlaying, String> {
    win_media::get_now_playing().await
}

#[tauri::command]
pub async fn media_play_pause() -> Result<(), String> {
    win_media::play_pause().await
}

#[tauri::command]
pub async fn media_next() -> Result<(), String> {
    win_media::next().await
}

#[tauri::command]
pub async fn media_previous() -> Result<(), String> {
    win_media::previous().await
}
