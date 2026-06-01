-- Habits system: generic habit tracking replacing standalone smoking table

CREATE TABLE IF NOT EXISTS habits (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT,
  icon TEXT,
  color TEXT NOT NULL DEFAULT 'red',
  tracking_type TEXT NOT NULL, -- 'boolean' | 'counter' | 'duration' | 'quantity' | 'rating'
  unit TEXT,
  daily_goal REAL,
  direction TEXT NOT NULL DEFAULT 'build', -- 'build' | 'limit'
  is_archived INTEGER NOT NULL DEFAULT 0,
  sort_order INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT,
  sync_status TEXT NOT NULL DEFAULT 'pending'
);

CREATE TABLE IF NOT EXISTS habit_entries (
  id TEXT PRIMARY KEY,
  habit_id TEXT NOT NULL,
  entry_date TEXT NOT NULL, -- YYYY-MM-DD local date
  value REAL NOT NULL DEFAULT 0,
  note TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT,
  sync_status TEXT NOT NULL DEFAULT 'pending',
  FOREIGN KEY (habit_id) REFERENCES habits(id)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_habit_entries_unique_day
ON habit_entries(habit_id, entry_date)
WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_habit_entries_date
ON habit_entries(entry_date);

CREATE INDEX IF NOT EXISTS idx_habit_entries_habit
ON habit_entries(habit_id);
