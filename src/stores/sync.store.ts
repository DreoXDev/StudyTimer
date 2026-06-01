import { defineStore } from 'pinia'
import { ref } from 'vue'
import { supabase, isSupabaseConfigured } from '@/lib/supabase'
import { api } from '@/lib/tauri'

export const useSyncStore = defineStore('sync', () => {
  const isConfigured = ref(isSupabaseConfigured)
  const isAuthenticated = ref(false)
  const userEmail = ref<string | null>(null)
  const syncing = ref(false)
  const error = ref<string | null>(null)
  const lastSyncedAt = ref<string | null>(localStorage.getItem('study_timer_last_synced_at'))

  async function init() {
    if (!isSupabaseConfigured || !supabase) return

    try {
      const { data } = await supabase.auth.getSession()
      isAuthenticated.value = !!data.session
      userEmail.value = data.session?.user?.email || null

      // Set up auth state change listener
      supabase.auth.onAuthStateChange((_event, session) => {
        isAuthenticated.value = !!session
        userEmail.value = session?.user?.email || null
        if (!session) {
          // Clear sync status if logged out
          lastSyncedAt.value = null
          localStorage.removeItem('study_timer_last_synced_at')
        }
      })
    } catch (e: any) {
      console.error('Errore durante init Supabase auth:', e)
    }
  }

  async function login(email: string, password: string) {
    if (!isSupabaseConfigured || !supabase) throw new Error('Supabase non configurato')
    error.value = null
    const { data, error: authError } = await supabase.auth.signInWithPassword({
      email,
      password,
    })
    if (authError) {
      error.value = authError.message
      throw authError
    }
    isAuthenticated.value = !!data.session
    userEmail.value = data.session?.user?.email || null
    return data
  }

  async function signup(email: string, password: string) {
    if (!isSupabaseConfigured || !supabase) throw new Error('Supabase non configurato')
    error.value = null
    const { data, error: authError } = await supabase.auth.signUp({
      email,
      password,
    })
    if (authError) {
      error.value = authError.message
      throw authError
    }
    return data
  }

  async function logout() {
    if (!isSupabaseConfigured || !supabase) return
    await supabase.auth.signOut()
    isAuthenticated.value = false
    userEmail.value = null
    lastSyncedAt.value = null
    localStorage.removeItem('study_timer_last_synced_at')
  }

  async function sync() {
    if (!isSupabaseConfigured || !supabase || !isAuthenticated.value) return false
    
    syncing.value = true
    error.value = null

    try {
      const userRes = await supabase.auth.getUser()
      const userId = userRes.data.user?.id
      if (!userId) {
        throw new Error('Utente non autenticato o ID non valido')
      }

      // Step 1: Upload unsynced local events to Supabase
      const unsyncedEvents = await api.sync.getUnsyncedEvents()
      for (const event of unsyncedEvents) {
        let metaObj = null
        if (event.metadataJson) {
          try {
            metaObj = JSON.parse(event.metadataJson)
          } catch (e) {
            console.error('Errore nel parsing metadata_json:', e)
          }
        }

        const payload = {
          id: event.id,
          user_id: userId,
          event_type: event.eventType,
          started_at: event.startedAt,
          ended_at: event.endedAt || null,
          duration_seconds: event.durationSeconds || null,
          value: event.value || null,
          unit: event.unit || null,
          source: event.source,
          note: event.note || null,
          metadata_json: metaObj,
          created_at: event.createdAt,
          updated_at: event.updatedAt,
          deleted_at: event.deletedAt || null
        }

        const { error: upsertError } = await supabase
          .from('tracking_events')
          .upsert(payload, { onConflict: 'id' })

        if (!upsertError) {
          await api.sync.updateEventSyncStatus(event.id, 'synced', new Date().toISOString())
        } else {
          console.error(`Errore nel caricamento cloud dell'evento ${event.id}:`, upsertError)
          await api.sync.updateEventSyncStatus(event.id, 'conflict')
        }
      }

      // Step 2: Download remote events updated since last sync from Supabase
      let query = supabase
        .from('tracking_events')
        .select('*')

      if (lastSyncedAt.value) {
        query = query.gt('updated_at', lastSyncedAt.value)
      }

      const { data: remoteEvents, error: pullError } = await query
      if (pullError) {
        throw pullError
      }

      if (remoteEvents && remoteEvents.length > 0) {
        for (const cloudEvent of remoteEvents) {
          const localEvent = {
            id: cloudEvent.id,
            eventType: cloudEvent.event_type,
            startedAt: cloudEvent.started_at,
            endedAt: cloudEvent.ended_at || undefined,
            durationSeconds: cloudEvent.duration_seconds || undefined,
            value: cloudEvent.value ? parseFloat(cloudEvent.value.toString()) : undefined,
            unit: cloudEvent.unit || undefined,
            source: cloudEvent.source,
            note: cloudEvent.note || undefined,
            metadataJson: cloudEvent.metadata_json ? JSON.stringify(cloudEvent.metadata_json) : undefined,
            createdAt: cloudEvent.created_at,
            updatedAt: cloudEvent.updated_at,
            deletedAt: cloudEvent.deleted_at || undefined,
            syncedAt: new Date().toISOString(),
            syncStatus: 'synced'
          }

          await api.sync.upsertSyncedEvent(localEvent)
        }
      }

      // Step 3: Update sync metadata
      const nowStr = new Date().toISOString()
      lastSyncedAt.value = nowStr
      localStorage.setItem('study_timer_last_synced_at', nowStr)
      return true
    } catch (e: any) {
      console.error('Errore durante la sincronizzazione:', e)
      error.value = e.message || e.toString()
      return false
    } finally {
      syncing.value = false
    }
  }

  return {
    isConfigured,
    isAuthenticated,
    userEmail,
    syncing,
    error,
    lastSyncedAt,
    init,
    login,
    signup,
    logout,
    sync,
  }
})
