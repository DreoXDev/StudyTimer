import { defineStore } from 'pinia'
import { ref } from 'vue'
import { api } from '@/lib/tauri'
import type { StudySession, CreateSessionPayload, StudyStats } from '@/types/session'

export const useSessionStore = defineStore('session', () => {
  const sessions = ref<StudySession[]>([])
  const stats = ref<StudyStats>({
    todayMinutes: 0,
    todaySessionsCount: 0,
    weekMinutes: 0,
  })
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function loadSessions() {
    loading.value = true
    error.value = null
    try {
      sessions.value = await api.sessions.list(20)
    } catch (e: any) {
      error.value = e.toString()
    } finally {
      loading.value = false
    }
  }

  async function loadStats() {
    const today = new Date()
    today.setHours(0, 0, 0, 0)
    const todayStart = today.toISOString()

    const weekAgo = new Date()
    weekAgo.setDate(weekAgo.getDate() - 7)
    weekAgo.setHours(0, 0, 0, 0)
    const weekStart = weekAgo.toISOString()

    try {
      stats.value = await api.sessions.getStats(todayStart, weekStart)
    } catch (e: any) {
      console.error('Errore durante il caricamento delle statistiche:', e)
    }
  }

  async function createSession(payload: CreateSessionPayload) {
    loading.value = true
    error.value = null
    try {
      const newSession = await api.sessions.create(payload)
      sessions.value.unshift(newSession)
      await loadStats() // reload stats
    } catch (e: any) {
      error.value = e.toString()
      throw e
    } finally {
      loading.value = false
    }
  }

  async function deleteSession(id: string) {
    loading.value = true
    error.value = null
    try {
      await api.sessions.delete(id)
      sessions.value = sessions.value.filter(s => s.id !== id)
      await loadStats() // reload stats
    } catch (e: any) {
      error.value = e.toString()
    } finally {
      loading.value = false
    }
  }

  return {
    sessions,
    stats,
    loading,
    error,
    loadSessions,
    loadStats,
    createSession,
    deleteSession,
  }
})
