-- Workouts: templates and logs

CREATE TABLE IF NOT EXISTS workout_templates (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT,
  category TEXT, -- 'gym' | 'swimming' | 'running' | 'home' | 'cycling' | etc.
  exercises_json TEXT NOT NULL DEFAULT '[]',
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT,
  sync_status TEXT NOT NULL DEFAULT 'pending'
);

CREATE TABLE IF NOT EXISTS workout_logs (
  id TEXT PRIMARY KEY,
  template_id TEXT,
  title TEXT NOT NULL,
  performed_at TEXT NOT NULL, -- ISO datetime UTC
  duration_minutes INTEGER,
  calories INTEGER,
  exercises_json TEXT NOT NULL DEFAULT '[]',
  notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT,
  sync_status TEXT NOT NULL DEFAULT 'pending',
  FOREIGN KEY (template_id) REFERENCES workout_templates(id)
);

CREATE INDEX IF NOT EXISTS idx_workout_logs_performed
ON workout_logs(performed_at);
