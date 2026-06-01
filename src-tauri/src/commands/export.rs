use tauri::State;
use crate::AppState;
use crate::AppError;
use crate::models::tracking::TrackingEvent;

#[tauri::command]
pub async fn export_tracking_data(
    format: String,
    range_start: String,
    range_end: String,
    event_types: Vec<String>,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    if event_types.is_empty() {
        return Ok("".to_string());
    }

    let placeholders = event_types.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query_str = format!(
        "SELECT id, event_type, started_at, ended_at, duration_seconds, value, unit, source, note, metadata_json, created_at, updated_at, deleted_at, synced_at, sync_status 
         FROM tracking_events 
         WHERE deleted_at IS NULL AND started_at >= ? AND started_at <= ? AND event_type IN ({})
         ORDER BY started_at ASC",
        placeholders
    );

    let mut query = sqlx::query_as::<_, TrackingEvent>(&query_str)
        .bind(&range_start)
        .bind(&range_end);

    for et in &event_types {
        query = query.bind(et);
    }

    let events = query.fetch_all(&state.db).await?;

    match format.to_lowercase().as_str() {
        "json" => {
            let serialized = serde_json::to_string_pretty(&events)
                .map_err(|e| AppError::InvalidInput(e.to_string()))?;
            Ok(serialized)
        }
        "csv" => {
            let mut csv_str = "id,event_type,started_at,ended_at,duration_seconds,value,unit,source,note,metadata_json,created_at,updated_at,sync_status\n".to_string();
            for ev in events {
                csv_str.push_str(&format!(
                    "\"{}\",\"{}\",\"{}\",\"{}\",{},{},\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                    ev.id,
                    ev.event_type,
                    ev.started_at,
                    ev.ended_at.unwrap_or_default(),
                    ev.duration_seconds.map(|d| d.to_string()).unwrap_or_default(),
                    ev.value.map(|v| v.to_string()).unwrap_or_default(),
                    ev.unit.unwrap_or_default(),
                    ev.source,
                    ev.note.unwrap_or_default().replace("\"", "\"\""),
                    ev.metadata_json.unwrap_or_default().replace("\"", "\"\""),
                    ev.created_at,
                    ev.updated_at,
                    ev.sync_status
                ));
            }
            Ok(csv_str)
        }
        "markdown" | "md" => {
            let mut md_str = format!(
                "# Esportazione StudyTimer\n\n- **Periodo**: {} - {}\n- **Eventi inclusi**: {}\n\n",
                range_start,
                range_end,
                event_types.join(", ")
            );

            let total_study_seconds: i64 = events.iter()
                .filter(|e| e.event_type == "study_session")
                .map(|e| e.duration_seconds.unwrap_or(0))
                .sum();
            
            let completed_sessions = events.iter()
                .filter(|e| e.event_type == "study_session" && e.metadata_json.as_ref().map_or(e.ended_at.is_some(), |m| m.contains("\"completed\":true")))
                .count();

            let interrupted_sessions = events.iter()
                .filter(|e| e.event_type == "study_session" && !e.metadata_json.as_ref().map_or(e.ended_at.is_some(), |m| m.contains("\"completed\":true")))
                .count();

            let cigarettes_count = events.iter()
                .filter(|e| e.event_type == "cigarette_smoked")
                .count();

            md_str.push_str("## Riepilogo Metriche\n\n");
            md_str.push_str(&format!("- **Tempo di studio totale**: {} minuti ({} ore)\n", total_study_seconds / 60, total_study_seconds / 3600));
            md_str.push_str(&format!("- **Sessioni completate**: {}\n", completed_sessions));
            md_str.push_str(&format!("- **Sessioni interrotte**: {}\n", interrupted_sessions));
            md_str.push_str(&format!("- **Sigarette fumate**: {}\n\n", cigarettes_count));

            md_str.push_str("## Registro Eventi Dettagliato\n\n");
            md_str.push_str("| Orario | Tipo Evento | Valore | Durata (min) | Note |\n");
            md_str.push_str("| --- | --- | ---: | ---: | --- |\n");

            for ev in events {
                let time_str = if ev.started_at.len() >= 16 {
                    ev.started_at[11..16].to_string()
                } else {
                    ev.started_at.clone()
                };
                let date_str = if ev.started_at.len() >= 10 {
                    ev.started_at[0..10].to_string()
                } else {
                    "".to_string()
                };

                let event_label = match ev.event_type.as_str() {
                    "study_session" => "Sessione Studio",
                    "cigarette_smoked" => "Sigaretta Fumata",
                    "timer_interrupted" => "Timer Interrotto",
                    _ => &ev.event_type,
                };

                let duration_min = ev.duration_seconds.map(|d| format!("{:.1}", d as f64 / 60.0)).unwrap_or_else(|| "-".to_string());
                let value_str = ev.value.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string());

                md_str.push_str(&format!(
                    "| {} {} | {} | {} | {} | {} |\n",
                    date_str,
                    time_str,
                    event_label,
                    value_str,
                    duration_min,
                    ev.note.unwrap_or_else(|| "-".to_string())
                ));
            }

            Ok(md_str)
        }
        _ => Err(AppError::InvalidInput("Formato di esportazione non supportato.".to_string())),
    }
}
