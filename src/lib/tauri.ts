import { invoke } from '@tauri-apps/api/core'
import type { StudySession, CreateSessionPayload, StudyStats } from '@/types/session'
import type { Task } from '@/types/task'
import type { TrackingEvent, CreateTrackingEventPayload, TrackingSummary } from '@/types/tracking'
import type { Habit, HabitEntry, CreateHabitPayload, UpdateHabitPayload, UpsertHabitEntryPayload } from '@/types/habit'
import type { WorkoutTemplate, WorkoutLog, CreateWorkoutTemplatePayload, CreateWorkoutLogPayload } from '@/types/workout'

export const api = {
  sessions: {
    list: (limit = 20) => invoke<StudySession[]>('list_sessions', { limit }),
    create: (payload: CreateSessionPayload) => invoke<StudySession>('create_session', { payload }),
    delete: (id: string) => invoke<void>('delete_session', { id }),
    getStats: (todayStart: string, weekStart: string) =>
      invoke<StudyStats>('get_stats', { todayStart, weekStart }),
  },
  tasks: {
    list: () => invoke<Task[]>('list_tasks'),
    create: (title: string) => invoke<Task>('create_task', { title }),
    updateCompleted: (id: string, completed: boolean) =>
      invoke<Task>('update_task_completed', { id, completed }),
    delete: (id: string) => invoke<void>('delete_task', { id }),
    reorder: (ids: string[]) => invoke<Task[]>('reorder_tasks', { ids }),
  },
  tracking: {
    createEvent: (payload: CreateTrackingEventPayload) =>
      invoke<TrackingEvent>('create_tracking_event', { payload }),
    listEvents: (eventType?: string, start?: string, end?: string, limit?: number) =>
      invoke<TrackingEvent[]>('list_tracking_events', { eventType, start, end, limit }),
    deleteEvent: (id: string) => invoke<void>('delete_tracking_event', { id }),
    getSummary: (start: string, end: string, granularity: string) =>
      invoke<TrackingSummary>('get_tracking_summary', { start, end, granularity }),
    getSmokingTodayCount: () => invoke<number>('get_smoking_today_count'),
    addCigarette: () => invoke<TrackingEvent>('add_cigarette'),
    removeLastCigaretteToday: () => invoke<boolean>('remove_last_cigarette_today'),
  },
  habits: {
    list: () => invoke<Habit[]>('list_habits'),
    create: (payload: CreateHabitPayload) => invoke<Habit>('create_habit', { payload }),
    update: (payload: UpdateHabitPayload) => invoke<Habit>('update_habit', { payload }),
    delete: (id: string) => invoke<void>('delete_habit', { id }),
    upsertEntry: (payload: UpsertHabitEntryPayload) => invoke<HabitEntry>('upsert_habit_entry', { payload }),
    incrementEntry: (habitId: string, entryDate: string, delta: number) =>
      invoke<HabitEntry>('increment_habit_entry', { habitId, entryDate, delta }),
    getEntriesRange: (habitId: string, startDate: string, endDate: string) =>
      invoke<HabitEntry[]>('get_habit_entries_range', { habitId, startDate, endDate }),
    getAllEntriesRange: (startDate: string, endDate: string) =>
      invoke<HabitEntry[]>('get_all_habit_entries_range', { startDate, endDate }),
  },
  workouts: {
    listTemplates: () => invoke<WorkoutTemplate[]>('list_workout_templates'),
    createTemplate: (payload: CreateWorkoutTemplatePayload) =>
      invoke<WorkoutTemplate>('create_workout_template', { payload }),
    deleteTemplate: (id: string) => invoke<void>('delete_workout_template', { id }),
    listLogs: (limit?: number) => invoke<WorkoutLog[]>('list_workout_logs', { limit }),
    getLogsRange: (start: string, end: string) =>
      invoke<WorkoutLog[]>('get_workout_logs_range', { start, end }),
    createLog: (payload: CreateWorkoutLogPayload) =>
      invoke<WorkoutLog>('create_workout_log', { payload }),
    deleteLog: (id: string) => invoke<void>('delete_workout_log', { id }),
  },
  export: {
    exportData: (format: string, rangeStart: string, rangeEnd: string, eventTypes: string[]) =>
      invoke<string>('export_tracking_data', { format, rangeStart, rangeEnd, eventTypes }),
  },
  sync: {
    updateEventSyncStatus: (id: string, syncStatus: string, syncedAt?: string) =>
      invoke<void>('update_event_sync_status', { id, status: syncStatus, syncedAt }),
    getUnsyncedEvents: () => invoke<TrackingEvent[]>('get_unsynced_events'),
    upsertSyncedEvent: (event: TrackingEvent) => invoke<void>('upsert_synced_event', { event }),
  },
}
