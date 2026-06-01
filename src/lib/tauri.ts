import { invoke } from '@tauri-apps/api/core'
import type { StudySession, CreateSessionPayload, StudyStats } from '@/types/session'
import type { Task } from '@/types/task'

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
}
