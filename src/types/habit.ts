export type HabitTrackingType = 'boolean' | 'counter' | 'duration' | 'quantity' | 'rating'
export type HabitDirection = 'build' | 'limit'

export interface Habit {
  id: string
  name: string
  description: string | null
  icon: string | null
  color: string
  tracking_type: HabitTrackingType
  unit: string | null
  daily_goal: number | null
  direction: HabitDirection
  is_archived: number  // 0 | 1
  sort_order: number
  created_at: string
  updated_at: string
  deleted_at: string | null
  sync_status: string
}

export interface HabitEntry {
  id: string
  habit_id: string
  entry_date: string  // YYYY-MM-DD
  value: number
  note: string | null
  created_at: string
  updated_at: string
  deleted_at: string | null
  sync_status: string
}

export interface CreateHabitPayload {
  name: string
  description?: string | null
  icon?: string | null
  color: string
  tracking_type: HabitTrackingType
  unit?: string | null
  daily_goal?: number | null
  direction: HabitDirection
}

export interface UpdateHabitPayload {
  id: string
  name?: string | null
  description?: string | null
  icon?: string | null
  color?: string | null
  unit?: string | null
  daily_goal?: number | null
  direction?: string | null
  is_archived?: boolean | null
  sort_order?: number | null
}

export interface UpsertHabitEntryPayload {
  habit_id: string
  entry_date: string
  value: number
  note?: string | null
}
