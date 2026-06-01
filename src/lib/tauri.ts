import { invoke } from '@tauri-apps/api/core'
import type { StudySession, CreateSessionPayload, StudyStats } from '@/types/session'
import type { Task } from '@/types/task'
import type { TrackingEvent, CreateTrackingEventPayload, TrackingSummary } from '@/types/tracking'

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

