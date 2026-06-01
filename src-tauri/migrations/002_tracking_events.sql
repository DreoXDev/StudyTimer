CREATE TABLE IF NOT EXISTS tracking_events (
  id TEXT PRIMARY KEY,
  event_type TEXT NOT NULL,
  started_at TEXT NOT NULL,
  ended_at TEXT,
  duration_seconds INTEGER,
  value REAL,
  unit TEXT,
  source TEXT NOT NULL DEFAULT 'manual',
  note TEXT,
  metadata_json TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT,
  synced_at TEXT,
  sync_status TEXT NOT NULL DEFAULT 'pending'
);

CREATE INDEX IF NOT EXISTS idx_tracking_events_type_started
ON tracking_events(event_type, started_at);

CREATE INDEX IF NOT EXISTS idx_tracking_events_started
ON tracking_events(started_at);

CREATE INDEX IF NOT EXISTS idx_tracking_events_sync_status
ON tracking_events(sync_status);
