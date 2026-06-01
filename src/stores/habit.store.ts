import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { api } from '@/lib/tauri'
import type { Habit, HabitEntry, CreateHabitPayload, UpdateHabitPayload, UpsertHabitEntryPayload } from '@/types/habit'

/** Returns today's date as YYYY-MM-DD in local time */
function todayLocal(): string {
  const d = new Date()
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`
}

/** Returns a date string YYYY-MM-DD for N days ago */
function daysAgo(n: number): string {
  const d = new Date()
  d.setDate(d.getDate() - n)
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`
}

export const useHabitStore = defineStore('habits', () => {
  const habits = ref<Habit[]>([])
  const entries = ref<HabitEntry[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  /** Map: habitId -> entryDate -> HabitEntry */
  const entryMap = computed(() => {
    const map: Record<string, Record<string, HabitEntry>> = {}
    for (const e of entries.value) {
      if (!map[e.habit_id]) map[e.habit_id] = {}
      map[e.habit_id][e.entry_date] = e
    }
    return map
  })

  const activeHabits = computed(() => habits.value.filter(h => h.is_archived === 0 && !h.deleted_at))

  function getTodayValue(habitId: string): number {
    return entryMap.value[habitId]?.[todayLocal()]?.value ?? 0
  }

  function getEntryForDate(habitId: string, date: string): HabitEntry | null {
    return entryMap.value[habitId]?.[date] ?? null
  }

  async function loadHabits() {
    loading.value = true
    error.value = null
    try {
      habits.value = await api.habits.list()
    } catch (e: any) {
      error.value = e.toString()
      console.error('Error loading habits:', e)
    } finally {
      loading.value = false
    }
  }

  async function loadEntriesRange(startDate: string, endDate: string) {
    try {
      const fetched = await api.habits.getAllEntriesRange(startDate, endDate)
      // Merge fetched into entries (replace range)
      const outside = entries.value.filter(e => e.entry_date < startDate || e.entry_date > endDate)
      entries.value = [...outside, ...fetched]
    } catch (e: any) {
      console.error('Error loading habit entries:', e)
    }
  }

  /** Load habits and the last 90 days of entries */
  async function init() {
    await loadHabits()
    await loadEntriesRange(daysAgo(89), todayLocal())
  }

  async function createHabit(payload: CreateHabitPayload): Promise<Habit> {
    const habit = await api.habits.create(payload)
    habits.value.unshift(habit)
    return habit
  }

  async function updateHabit(payload: UpdateHabitPayload): Promise<Habit> {
    const updated = await api.habits.update(payload)
    const idx = habits.value.findIndex(h => h.id === updated.id)
    if (idx !== -1) habits.value[idx] = updated
    return updated
  }

  async function deleteHabit(id: string) {
    await api.habits.delete(id)
    const idx = habits.value.findIndex(h => h.id === id)
    if (idx !== -1) habits.value.splice(idx, 1)
  }

  async function upsertEntry(payload: UpsertHabitEntryPayload): Promise<HabitEntry> {
    const entry = await api.habits.upsertEntry(payload)
    // Update local entries
    const existing = entries.value.findIndex(e => e.habit_id === entry.habit_id && e.entry_date === entry.entry_date)
    if (existing !== -1) {
      entries.value[existing] = entry
    } else {
      entries.value.push(entry)
    }
    return entry
  }

  async function increment(habitId: string, delta = 1): Promise<HabitEntry> {
    const date = todayLocal()
    const entry = await api.habits.incrementEntry(habitId, date, delta)
    const existing = entries.value.findIndex(e => e.habit_id === entry.habit_id && e.entry_date === entry.entry_date)
    if (existing !== -1) {
      entries.value[existing] = entry
    } else {
      entries.value.push(entry)
    }
    return entry
  }

  async function decrement(habitId: string): Promise<HabitEntry> {
    return increment(habitId, -1)
  }

  return {
    habits,
    entries,
    loading,
    error,
    entryMap,
    activeHabits,
    todayLocal,
    getTodayValue,
    getEntryForDate,
    loadHabits,
    loadEntriesRange,
    init,
    createHabit,
    updateHabit,
    deleteHabit,
    upsertEntry,
    increment,
    decrement,
  }
})
