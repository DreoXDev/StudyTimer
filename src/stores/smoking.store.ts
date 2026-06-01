import { defineStore } from 'pinia'
import { ref } from 'vue'
import { api } from '@/lib/tauri'
import { useSyncStore } from './sync.store'

export const useSmokingStore = defineStore('smoking', () => {
  const countToday = ref(0)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function loadSmokingTodayCount() {
    loading.value = true
    error.value = null
    try {
      countToday.value = await api.tracking.getSmokingTodayCount()
    } catch (e: any) {
      console.error('Errore durante il caricamento delle sigarette oggi:', e)
      error.value = e.toString()
    } finally {
      loading.value = false
    }
  }

  async function addCigarette() {
    loading.value = true
    error.value = null
    try {
      await api.tracking.addCigarette()
      countToday.value += 1

      const syncStore = useSyncStore()
      if (syncStore.isAuthenticated) {
        syncStore.sync().catch(err => console.error('Errore sync in background:', err))
      }
    } catch (e: any) {
      console.error('Errore aggiunta sigaretta:', e)
      error.value = e.toString()
    } finally {
      loading.value = false
    }
  }

  async function removeLastCigaretteToday() {
    loading.value = true
    error.value = null
    try {
      const removed = await api.tracking.removeLastCigaretteToday()
      if (removed && countToday.value > 0) {
        countToday.value -= 1

        const syncStore = useSyncStore()
        if (syncStore.isAuthenticated) {
          syncStore.sync().catch(err => console.error('Errore sync in background:', err))
        }
      }
      return removed
    } catch (e: any) {
      console.error('Errore rimozione sigaretta:', e)
      error.value = e.toString()
      return false
    } finally {
      loading.value = false
    }
  }

  return {
    countToday,
    loading,
    error,
    loadSmokingTodayCount,
    addCigarette,
    removeLastCigaretteToday,
  }
})
