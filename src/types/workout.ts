export interface Exercise {
  name: string
  sets?: Array<{ reps?: number; weight?: number; duration_seconds?: number }>
  distance_m?: number
  notes?: string
}

export interface WorkoutTemplate {
  id: string
  name: string
  description: string | null
  category: string | null
  exercises_json: string
  created_at: string
  updated_at: string
  deleted_at: string | null
  sync_status: string
}

export interface WorkoutLog {
  id: string
  template_id: string | null
  title: string
  performed_at: string
  duration_minutes: number | null
  calories: number | null
  exercises_json: string
  notes: string | null
  created_at: string
  updated_at: string
  deleted_at: string | null
  sync_status: string
}

export interface CreateWorkoutTemplatePayload {
  name: string
  description?: string | null
  category?: string | null
  exercises_json: string
}

export interface CreateWorkoutLogPayload {
  template_id?: string | null
  title: string
  performed_at: string
  duration_minutes?: number | null
  calories?: number | null
  exercises_json: string
  notes?: string | null
}
