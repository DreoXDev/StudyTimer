export interface StudySession {
  id: string
  startedAt: string
  endedAt: string
  plannedDurationSeconds: number
  actualDurationSeconds: number
  completed: boolean
  mode: 'focus' | 'break' | 'deep'
  note?: string
}

export interface CreateSessionPayload {
  id: string
  startedAt: string
  endedAt: string
  plannedDurationSeconds: number
  actualDurationSeconds: number
  completed: boolean
  mode: 'focus' | 'break' | 'deep'
  note?: string
}

export interface StudyStats {
  todayMinutes: number
  todaySessionsCount: number
  weekMinutes: number
}
