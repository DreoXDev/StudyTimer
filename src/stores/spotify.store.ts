import { defineStore } from 'pinia'
import { ref, onUnmounted } from 'vue'
import type { NowPlayingTrack } from '@/types/spotify'

export const useSpotifyStore = defineStore('spotify', () => {
  const connected = ref(false)
  const loading = ref(false)
  const error = ref<string | null>(null)

  const track = ref<NowPlayingTrack>({
    title: 'Focus & Concentration',
    artist: 'Study Music Academy',
    album: 'Deep Work Chillout',
    isPlaying: false,
    progressMs: 45000,
    durationMs: 180000,
  })

  let progressIntervalId: number | null = null

  function connect() {
    loading.value = true
    // Simulate loading/OAuth handshake response
    setTimeout(() => {
      connected.value = true
      loading.value = false
      startProgressTicker()
    }, 800)
  }

  function disconnect() {
    connected.value = false
    stopProgressTicker()
  }

  function togglePlay() {
    if (!connected.value) return
    track.value.isPlaying = !track.value.isPlaying
    if (track.value.isPlaying) {
      startProgressTicker()
    } else {
      stopProgressTicker()
    }
  }

  function startProgressTicker() {
    stopProgressTicker()
    progressIntervalId = window.setInterval(() => {
      if (
        connected.value &&
        track.value.isPlaying &&
        track.value.progressMs !== undefined &&
        track.value.durationMs !== undefined
      ) {
        if (track.value.progressMs >= track.value.durationMs) {
          track.value.progressMs = 0 // loop mock track
        } else {
          track.value.progressMs += 1000
        }
      }
    }, 1000)
  }

  function stopProgressTicker() {
    if (progressIntervalId !== null) {
      clearInterval(progressIntervalId)
      progressIntervalId = null
    }
  }

  onUnmounted(() => {
    stopProgressTicker()
  })

  return {
    connected,
    loading,
    error,
    track,
    connect,
    disconnect,
    togglePlay,
  }
})
