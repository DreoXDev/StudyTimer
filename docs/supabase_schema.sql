-- Schema di database per Supabase Sync (StudyTimer)

-- Creazione della tabella per gli eventi di tracciamento generali
CREATE TABLE IF NOT EXISTS public.tracking_events (
  id uuid PRIMARY KEY,
  user_id uuid NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
  event_type text NOT NULL,
  started_at timestamptz NOT NULL,
  ended_at timestamptz,
  duration_seconds integer,
  value numeric,
  unit text,
  source text NOT NULL DEFAULT 'manual',
  note text,
  metadata_json jsonb,
  created_at timestamptz NOT NULL,
  updated_at timestamptz NOT NULL,
  deleted_at timestamptz
);

-- Indici per velocizzare filtri e aggregazioni
CREATE INDEX IF NOT EXISTS idx_tracking_events_user_type_started ON public.tracking_events(user_id, event_type, started_at);
CREATE INDEX IF NOT EXISTS idx_tracking_events_user_updated ON public.tracking_events(user_id, updated_at);

-- Abilitazione della Row Level Security (RLS)
ALTER TABLE public.tracking_events ENABLE ROW LEVEL SECURITY;

-- Politiche di sicurezza RLS (Ciascun utente può accedere esclusivamente ai propri dati)

CREATE POLICY "Users can read own tracking events"
ON public.tracking_events
FOR SELECT
USING (auth.uid() = user_id);

CREATE POLICY "Users can insert own tracking events"
ON public.tracking_events
FOR INSERT
WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can update own tracking events"
ON public.tracking_events
FOR UPDATE
USING (auth.uid() = user_id)
WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can delete own tracking events"
ON public.tracking_events
FOR DELETE
USING (auth.uid() = user_id);
