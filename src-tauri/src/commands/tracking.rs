use tauri::State;
use crate::AppState;
use crate::AppError;
use crate::models::tracking::{TrackingEvent, CreateTrackingEventPayload};
use uuid::Uuid;
use chrono::Utc;

#[tauri::command]
pub async fn create_tracking_event(
    payload: CreateTrackingEventPayload,
    state: State<'_, AppState>,
) -> Result<TrackingEvent, AppError> {
    let now = Utc::now().to_rfc3339();
    let source = payload.source.unwrap_or_else(|| "manual".to_string());
    
    sqlx::query(
        "INSERT INTO tracking_events (id, event_type, started_at, ended_at, duration_seconds, value, unit, source, note, metadata_json, created_at, updated_at, sync_status)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'pending')"
    )
    .bind(&payload.id)
    .bind(&payload.event_type)
    .bind(&payload.started_at)
    .bind(&payload.ended_at)
    .bind(payload.duration_seconds)
    .bind(payload.value)
    .bind(&payload.unit)
    .bind(&source)
    .bind(&payload.note)
    .bind(&payload.metadata_json)
    .bind(&now)
    .bind(&now)
    .execute(&state.db)
    .await?;

    Ok(TrackingEvent {
        id: payload.id,
        event_type: payload.event_type,
        started_at: payload.started_at,
        ended_at: payload.ended_at,
        duration_seconds: payload.duration_seconds,
        value: payload.value,
        unit: payload.unit,
        source,
        note: payload.note,
        metadata_json: payload.metadata_json,
        created_at: now.clone(),
        updated_at: now,
        deleted_at: None,
        synced_at: None,
        sync_status: "pending".to_string(),
    })
}

#[tauri::command]
pub async fn list_tracking_events(
    event_type: Option<String>,
    start: Option<String>,
    end: Option<String>,
    limit: Option<i64>,
    state: State<'_, AppState>,
) -> Result<Vec<TrackingEvent>, AppError> {
    let limit_val = limit.unwrap_or(100);
    
    let mut query = "SELECT id, event_type, started_at, ended_at, duration_seconds, value, unit, source, note, metadata_json, created_at, updated_at, deleted_at, synced_at, sync_status 
                     FROM tracking_events 
                     WHERE deleted_at IS NULL".to_string();
    
    if event_type.is_some() {
        query.push_str(" AND event_type = ?");
    }
    if start.is_some() {
        query.push_str(" AND started_at >= ?");
    }
    if end.is_some() {
        query.push_str(" AND started_at <= ?");
    }
    
    query.push_str(" ORDER BY started_at DESC LIMIT ?");
    
    let mut q = sqlx::query_as::<_, TrackingEvent>(&query);
    
    if let Some(ref et) = event_type {
        q = q.bind(et);
    }
    if let Some(ref s) = start {
        q = q.bind(s);
    }
    if let Some(ref e) = end {
        q = q.bind(e);
    }
    q = q.bind(limit_val);
    
    let events = q.fetch_all(&state.db).await?;
    Ok(events)
}

#[tauri::command]
pub async fn delete_tracking_event(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let now = Utc::now().to_rfc3339();
    sqlx::query("UPDATE tracking_events SET deleted_at = ?, updated_at = ?, sync_status = 'pending' WHERE id = ?")
        .bind(&now)
        .bind(&now)
        .bind(&id)
        .execute(&state.db)
        .await?;

    // Also delete from study_sessions (since we only keep study_sessions now)
    sqlx::query("DELETE FROM study_sessions WHERE id = ?")
        .bind(&id)
        .execute(&state.db)
        .await?;

    Ok(())
}

#[tauri::command]
pub async fn get_smoking_today_count(
    state: State<'_, AppState>,
) -> Result<i64, AppError> {
    let today_start = Utc::now().date_naive().and_hms_opt(0, 0, 0).unwrap().and_local_timezone(Utc).unwrap().to_rfc3339();
    
    let res: (Option<i64>,) = sqlx::query_as(
        "SELECT COUNT(id) FROM tracking_events WHERE event_type = 'cigarette_smoked' AND started_at >= ? AND deleted_at IS NULL"
    )
    .bind(today_start)
    .fetch_one(&state.db)
    .await?;
    
    Ok(res.0.unwrap_or(0))
}

#[tauri::command]
pub async fn add_cigarette(
    state: State<'_, AppState>,
) -> Result<TrackingEvent, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        "INSERT INTO tracking_events (id, event_type, started_at, ended_at, duration_seconds, value, unit, source, note, metadata_json, created_at, updated_at, sync_status)
         VALUES (?, 'cigarette_smoked', ?, ?, NULL, 1.0, 'cigarette', 'manual', NULL, NULL, ?, ?, 'pending')"
    )
    .bind(&id)
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .execute(&state.db)
    .await?;
    
    Ok(TrackingEvent {
        id,
        event_type: "cigarette_smoked".to_string(),
        started_at: now.clone(),
        ended_at: Some(now.clone()),
        duration_seconds: None,
        value: Some(1.0),
        unit: Some("cigarette".to_string()),
        source: "manual".to_string(),
        note: None,
        metadata_json: None,
        created_at: now.clone(),
        updated_at: now,
        deleted_at: None,
        synced_at: None,
        sync_status: "pending".to_string(),
    })
}

#[tauri::command]
pub async fn remove_last_cigarette_today(
    state: State<'_, AppState>,
) -> Result<bool, AppError> {
    let today_start = Utc::now().date_naive().and_hms_opt(0, 0, 0).unwrap().and_local_timezone(Utc).unwrap().to_rfc3339();
    let now = Utc::now().to_rfc3339();

    // Find the latest cigarette smoked today
    let last_event: Option<(String,)> = sqlx::query_as(
        "SELECT id FROM tracking_events 
         WHERE event_type = 'cigarette_smoked' AND started_at >= ? AND deleted_at IS NULL 
         ORDER BY started_at DESC LIMIT 1"
    )
    .bind(today_start)
    .fetch_optional(&state.db)
    .await?;

    if let Some((id,)) = last_event {
        sqlx::query("UPDATE tracking_events SET deleted_at = ?, updated_at = ?, sync_status = 'pending' WHERE id = ?")
            .bind(&now)
            .bind(&now)
            .bind(id)
            .execute(&state.db)
            .await?;
        Ok(true)
    } else {
        Ok(false)
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BucketValue {
    pub bucket: String,
    pub seconds: f64,
    pub count: i64,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StudySummary {
    pub total_seconds: i64,
    pub completed_sessions: i64,
    pub interrupted_sessions: i64,
    pub average_session_seconds: f64,
    pub by_bucket: Vec<BucketValue>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SmokingSummary {
    pub total_cigarettes: i64,
    pub by_bucket: Vec<BucketValue>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingSummary {
    pub range_start: String,
    pub range_end: String,
    pub study: StudySummary,
    pub smoking: SmokingSummary,
}

#[tauri::command]
pub async fn get_tracking_summary(
    start: String,
    end: String,
    granularity: String,
    state: State<'_, AppState>,
) -> Result<TrackingSummary, AppError> {
    let events = sqlx::query_as::<_, TrackingEvent>(
        "SELECT id, event_type, started_at, ended_at, duration_seconds, value, unit, source, note, metadata_json, created_at, updated_at, deleted_at, synced_at, sync_status 
         FROM tracking_events 
         WHERE deleted_at IS NULL AND started_at >= ? AND started_at <= ?"
    )
    .bind(&start)
    .bind(&end)
    .fetch_all(&state.db)
    .await?;

    let mut total_study_seconds = 0;
    let mut completed_sessions = 0;
    let mut interrupted_sessions = 0;
    let mut total_cigarettes = 0;

    use std::collections::HashMap;
    let mut study_buckets: HashMap<String, (f64, i64)> = HashMap::new();
    let mut smoking_buckets: HashMap<String, i64> = HashMap::new();

    for ev in events {
        let bucket_key = if ev.started_at.len() >= 10 {
            match granularity.as_str() {
                "hour" => {
                    if ev.started_at.len() >= 13 {
                        ev.started_at[0..13].to_string()
                    } else {
                        ev.started_at[0..10].to_string()
                    }
                }
                "day" => ev.started_at[0..10].to_string(),
                "month" => ev.started_at[0..7].to_string(),
                _ => ev.started_at[0..10].to_string(),
            }
        } else {
            ev.started_at.clone()
        };

        if ev.event_type == "study_session" {
            let dur = ev.duration_seconds.unwrap_or(0);
            total_study_seconds += dur;
            
            let is_completed = if let Some(ref meta) = ev.metadata_json {
                meta.contains("\"completed\":true")
            } else {
                ev.ended_at.is_some()
            };

            if is_completed {
                completed_sessions += 1;
            } else {
                interrupted_sessions += 1;
            }

            let entry = study_buckets.entry(bucket_key).or_insert((0.0, 0));
            entry.0 += dur as f64;
            entry.1 += 1;
        } else if ev.event_type == "cigarette_smoked" {
            total_cigarettes += 1;
            let entry = smoking_buckets.entry(bucket_key).or_insert(0);
            *entry += 1;
        }
    }

    let average_session_seconds = if (completed_sessions + interrupted_sessions) > 0 {
        total_study_seconds as f64 / (completed_sessions + interrupted_sessions) as f64
    } else {
        0.0
    };

    let mut study_by_bucket: Vec<BucketValue> = study_buckets
        .into_iter()
        .map(|(k, v)| BucketValue {
            bucket: k,
            seconds: v.0,
            count: v.1,
        })
        .collect();
    study_by_bucket.sort_by(|a, b| a.bucket.cmp(&b.bucket));

    let mut smoking_by_bucket: Vec<BucketValue> = smoking_buckets
        .into_iter()
        .map(|(k, v)| BucketValue {
            bucket: k,
            seconds: 0.0,
            count: v,
        })
        .collect();
    smoking_by_bucket.sort_by(|a, b| a.bucket.cmp(&b.bucket));

    Ok(TrackingSummary {
        range_start: start,
        range_end: end,
        study: StudySummary {
            total_seconds: total_study_seconds,
            completed_sessions,
            interrupted_sessions,
            average_session_seconds,
            by_bucket: study_by_bucket,
        },
        smoking: SmokingSummary {
            total_cigarettes,
            by_bucket: smoking_by_bucket,
        },
    })
}
