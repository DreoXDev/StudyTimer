import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { api } from '@/lib/tauri'
import type { WorkoutTemplate, WorkoutLog, CreateWorkoutTemplatePayload, CreateWorkoutLogPayload } from '@/types/workout'

export const useWorkoutStore = defineStore('workouts', () => {
  const templates = ref<WorkoutTemplate[]>([])
  const logs = ref<WorkoutLog[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  const recentLogs = computed(() => logs.value.slice(0, 10))

  /** Logs this week */
  const logsThisWeek = computed(() => {
    const now = new Date()
    const dayOfWeek = now.getDay()
    const weekStart = new Date(now)
    weekStart.setDate(now.getDate() - (dayOfWeek === 0 ? 6 : dayOfWeek - 1))
    weekStart.setHours(0, 0, 0, 0)
    return logs.value.filter(l => new Date(l.performed_at) >= weekStart && !l.deleted_at)
  })

  const totalMinutesThisWeek = computed(() =>
    logsThisWeek.value.reduce((sum, l) => sum + (l.duration_minutes ?? 0), 0)
  )

  const totalCaloriesThisWeek = computed(() =>
    logsThisWeek.value.reduce((sum, l) => sum + (l.calories ?? 0), 0)
  )

  async function loadTemplates() {
    try {
      templates.value = await api.workouts.listTemplates()
    } catch (e: any) {
      console.error('Error loading workout templates:', e)
    }
  }

  async function loadLogs(limit = 50) {
    loading.value = true
    error.value = null
    try {
      logs.value = await api.workouts.listLogs(limit)
    } catch (e: any) {
      error.value = e.toString()
      console.error('Error loading workout logs:', e)
    } finally {
      loading.value = false
    }
  }

  async function init() {
    await Promise.all([loadTemplates(), loadLogs()])
  }

  async function createTemplate(payload: CreateWorkoutTemplatePayload): Promise<WorkoutTemplate> {
    const tmpl = await api.workouts.createTemplate(payload)
    templates.value.unshift(tmpl)
    return tmpl
  }

  async function deleteTemplate(id: string) {
    await api.workouts.deleteTemplate(id)
    const idx = templates.value.findIndex(t => t.id === id)
    if (idx !== -1) templates.value.splice(idx, 1)
  }

  async function createLog(payload: CreateWorkoutLogPayload): Promise<WorkoutLog> {
    const log = await api.workouts.createLog(payload)
    logs.value.unshift(log)
    return log
  }

  async function deleteLog(id: string) {
    await api.workouts.deleteLog(id)
    const idx = logs.value.findIndex(l => l.id === id)
    if (idx !== -1) logs.value.splice(idx, 1)
  }

  return {
    templates,
    logs,
    loading,
    error,
    recentLogs,
    logsThisWeek,
    totalMinutesThisWeek,
    totalCaloriesThisWeek,
    loadTemplates,
    loadLogs,
    init,
    createTemplate,
    deleteTemplate,
    createLog,
    deleteLog,
  }
})
