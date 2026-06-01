import { defineStore } from 'pinia'
import { ref, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface NowPlaying {
  available: boolean
  source?: string
  title?: string
  artist?: string
  album?: string
  isPlaying: boolean
  progressMs?: number
  durationMs?: number
}

export const useMediaStore = defineStore('media', () => {
  const current = ref<NowPlaying | null>({
    available: true,
    source: 'Spotify',
    title: 'Focus & Concentration',
    artist: 'Study Music Academy',
    album: 'Deep Work Chillout',
    isPlaying: false,
    progressMs: 45000,
    durationMs: 180000,
  })
  const loading = ref(false)
  const error = ref<string | null>(null)

  let progressIntervalId: number | null = null
  let pollingIntervalId: number | null = null

  function isWindows(): boolean {
    return /win/i.test(navigator.userAgent || navigator.platform || '')
  }

  async function refreshNowPlaying() {
    try {
      const res = await invoke<{
        available: boolean
        source: string | null
        title: string | null
        artist: string | null
        album: string | null
        is_playing: boolean
        progress_ms: number | null
        duration_ms: number | null
      }>('get_now_playing')

      if (res && res.available) {
        current.value = {
          available: true,
          source: res.source ?? undefined,
          title: res.title ?? undefined,
          artist: res.artist ?? undefined,
          album: res.album ?? undefined,
          isPlaying: res.is_playing,
          progressMs: res.progress_ms ?? 0,
          durationMs: res.duration_ms ?? 0,
        }
        error.value = null
        
        if (current.value.isPlaying) {
          startProgressTicker()
        } else {
          stopProgressTicker()
        }
      } else {
        if (isWindows()) {
          current.value = {
            available: false,
            isPlaying: false
          }
          stopProgressTicker()
        } else {
          if (!current.value || !current.value.available || current.value.source === undefined) {
            current.value = {
              available: true,
              source: 'Spotify (Mock)',
              title: 'Focus & Concentration',
              artist: 'Study Music Academy',
              album: 'Deep Work Chillout',
              isPlaying: false,
              progressMs: 45000,
              durationMs: 180000,
            }
          }
          tickProgress()
        }
      }
    } catch (e: any) {
      console.error('Failed to get now playing from Rust:', e)
      error.value = e.toString()
      
      if (isWindows()) {
        current.value = {
          available: false,
          isPlaying: false
        }
        stopProgressTicker()
      } else {
        if (!current.value || !current.value.available || current.value.source === undefined) {
          current.value = {
            available: true,
            source: 'Spotify (Mock)',
            title: 'Focus & Concentration',
            artist: 'Study Music Academy',
            album: 'Deep Work Chillout',
            isPlaying: false,
            progressMs: 45000,
            durationMs: 180000,
          }
        }
        tickProgress()
      }
    }
  }

  async function playPause() {
    if (!current.value) return
    
    if (isWindows() && current.value.available && current.value.source !== 'Spotify (Mock)') {
      try {
        await invoke('media_play_pause')
        current.value.isPlaying = !current.value.isPlaying
        await refreshNowPlaying()
      } catch (e) {
        console.error('Failed play_pause:', e)
      }
    } else {
      current.value.isPlaying = !current.value.isPlaying
      if (current.value.isPlaying) {
        startProgressTicker()
      } else {
        stopProgressTicker()
      }
    }
  }

  async function next() {
    if (!current.value) return
    
    if (isWindows() && current.value.available && current.value.source !== 'Spotify (Mock)') {
      try {
        await invoke('media_next')
        await refreshNowPlaying()
      } catch (e) {
        console.error('Failed next:', e)
      }
    } else {
      current.value.title = 'Ambient Noise Generator'
      current.value.artist = 'Calm Waves Collective'
      current.value.progressMs = 0
    }
  }

  async function previous() {
    if (!current.value) return
    
    if (isWindows() && current.value.available && current.value.source !== 'Spotify (Mock)') {
      try {
        await invoke('media_previous')
        await refreshNowPlaying()
      } catch (e) {
        console.error('Failed previous:', e)
      }
    } else {
      current.value.title = 'Focus & Concentration'
      current.value.artist = 'Study Music Academy'
      current.value.progressMs = 0
    }
  }

  function tickProgress() {
    if (current.value && current.value.isPlaying && current.value.progressMs !== undefined && current.value.durationMs !== undefined) {
      if (current.value.progressMs >= current.value.durationMs) {
        current.value.progressMs = 0
      } else {
        current.value.progressMs += 1000
      }
    }
  }

  function startProgressTicker() {
    stopProgressTicker()
    progressIntervalId = window.setInterval(tickProgress, 1000)
  }

  function stopProgressTicker() {
    if (progressIntervalId !== null) {
      clearInterval(progressIntervalId)
      progressIntervalId = null
    }
  }

  function startPolling() {
    stopPolling()
    pollingIntervalId = window.setInterval(refreshNowPlaying, 2500)
  }

  function stopPolling() {
    if (pollingIntervalId !== null) {
      clearInterval(pollingIntervalId)
      pollingIntervalId = null
    }
  }

  startPolling()

  onUnmounted(() => {
    stopProgressTicker()
    stopPolling()
  })

  return {
    current,
    loading,
    error,
    refreshNowPlaying,
    playPause,
    next,
    previous,
  }
})

