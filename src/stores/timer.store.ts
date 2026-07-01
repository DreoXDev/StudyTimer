import { defineStore } from 'pinia'
import { ref, computed, onUnmounted, watch } from 'vue'
import { useSessionStore } from './session.store'
import { toast } from 'vue-sonner'
import { useSyncStore } from './sync.store'
import { useSettingsStore } from './settings.store'

export const useTimerStore = defineStore('timer', () => {
  const settingsStore = useSettingsStore()
  const status = ref<'idle' | 'running' | 'paused' | 'completed'>('idle')
  const mode = ref<'focus' | 'break' | 'deep'>('focus')
  const plannedDurationSeconds = ref(settingsStore.defaultFocusDuration)
  const remainingSeconds = ref(settingsStore.defaultFocusDuration)

  const startedAt = ref<number | null>(null)
  const pausedAt = ref<number | null>(null)
  const accumulatedPausedMs = ref(0)

  let tickIntervalId: number | null = null

  const progress = computed(() => {
    if (plannedDurationSeconds.value === 0) return 100
    return (remainingSeconds.value / plannedDurationSeconds.value) * 100
  })

  const elapsedSeconds = computed(() => {
    if (startedAt.value === null) return 0
    if (status.value === 'completed') return plannedDurationSeconds.value

    const endTimestamp = status.value === 'paused' && pausedAt.value !== null ? pausedAt.value : Date.now()
    const elapsedMs = endTimestamp - startedAt.value - accumulatedPausedMs.value
    return Math.min(plannedDurationSeconds.value, Math.max(0, Math.floor(elapsedMs / 1000)))
  })

  // Start the timer
  function start() {
    if (status.value === 'idle' || status.value === 'completed') {
      status.value = 'running'
      startedAt.value = Date.now()
      pausedAt.value = null
      accumulatedPausedMs.value = 0
      remainingSeconds.value = plannedDurationSeconds.value
      startTicking()
    }
  }

  // Pause the timer
  function pause() {
    if (status.value === 'running') {
      status.value = 'paused'
      pausedAt.value = Date.now()
      stopTicking()
    }
  }

  // Resume the timer
  function resume() {
    if (status.value === 'paused' && startedAt.value !== null && pausedAt.value !== null) {
      status.value = 'running'
      accumulatedPausedMs.value += Date.now() - pausedAt.value
      pausedAt.value = null
      startTicking()
    }
  }

  // Reset the timer
  function reset() {
    if ((status.value === 'running' || status.value === 'paused') && startedAt.value !== null) {
      const elapsed = elapsedSeconds.value
      if (elapsed >= 60) {
        const sessionStore = useSessionStore()
        const completedStartedAt = startedAt.value
        const completedEndedAt = Date.now()
        const completedPlannedSeconds = plannedDurationSeconds.value
        const completedMode = mode.value

        const sessionId = crypto.randomUUID()
        sessionStore.createSession({
          id: sessionId,
          startedAt: new Date(completedStartedAt).toISOString(),
          endedAt: new Date(completedEndedAt).toISOString(),
          plannedDurationSeconds: completedPlannedSeconds,
          actualDurationSeconds: elapsed,
          completed: false,
          mode: completedMode,
          note: 'Interrotta manualmente',
        })
        .then(() => {
          toast.info('Sessione interrotta salvata nel registro.')
          const syncStore = useSyncStore()
          if (syncStore.isAuthenticated) {
            syncStore.sync().catch(err => console.error('Errore sync in background:', err))
          }
        })
        .catch(err => console.error('Errore nel salvataggio della sessione interrotta:', err))
      }
    }

    status.value = 'idle'
    startedAt.value = null
    pausedAt.value = null
    accumulatedPausedMs.value = 0
    remainingSeconds.value = plannedDurationSeconds.value
    stopTicking()
  }

  // Tick calculation to avoid drift
  function tick() {
    if (status.value !== 'running' || startedAt.value === null) return

    const now = Date.now()
    const elapsedMs = now - startedAt.value - accumulatedPausedMs.value
    const elapsedSecs = Math.floor(elapsedMs / 1000)

    const nextRemaining = plannedDurationSeconds.value - elapsedSecs

    if (nextRemaining <= 0) {
      void completeTimer()
    } else {
      remainingSeconds.value = nextRemaining
    }
  }

  function startTicking() {
    stopTicking()
    tick() // initial tick
    tickIntervalId = window.setInterval(tick, 200) // tick frequently for accuracy
  }

  function stopTicking() {
    if (tickIntervalId !== null) {
      clearInterval(tickIntervalId)
      tickIntervalId = null
    }
  }

  const canEditDuration = computed(() => status.value === 'idle' || status.value === 'completed')

  const onCompletedCallbacks: Array<() => void> = []
  function onCompleted(callback: () => void) {
    onCompletedCallbacks.push(callback)
  }

  async function completeTimer() {
    if (status.value === 'completed') return
    if (startedAt.value === null) return

    const completedStartedAt = startedAt.value
    const completedEndedAt = Date.now()
    const completedPlannedSeconds = plannedDurationSeconds.value
    const completedMode = mode.value

    status.value = 'completed'
    remainingSeconds.value = 0
    stopTicking()

    // Play sound notification
    try {
      const audio = new Audio('https://assets.mixkit.co/active_storage/sfx/2869/2869-84.wav')
      audio.volume = 0.3
      audio.play()
    } catch (e) {
      console.log('Audio playback not supported or blocked')
    }

    try {
      const sessionStore = useSessionStore()
      const sessionId = crypto.randomUUID()
      await sessionStore.createSession({
        id: sessionId,
        startedAt: new Date(completedStartedAt).toISOString(),
        endedAt: new Date(completedEndedAt).toISOString(),
        plannedDurationSeconds: completedPlannedSeconds,
        actualDurationSeconds: completedPlannedSeconds,
        completed: true,
        mode: completedMode,
        note: `Sessione ${completedMode} completata`,
      })

      toast.success('Sessione completata e salvata nel registro!')

      // Trigger background sync
      const syncStore = useSyncStore()
      if (syncStore.isAuthenticated) {
        syncStore.sync().catch(err => console.error('Errore sync in background:', err))
      }
    } catch (error) {
      console.error('Failed to save completed study session:', error)
      toast.error('Timer completato, ma il salvataggio della sessione è fallito.')
    }

    for (const cb of onCompletedCallbacks) {
      cb()
    }

    // Auto-start next mode if enabled
    if (settingsStore.autoStart) {
      setTimeout(() => {
        if (completedMode === 'focus' || completedMode === 'deep') {
          // Switch to break preset (5m = 300s)
          setPreset(300, 'break')
        } else {
          // Switch to focus preset (defaultFocusDuration)
          setPreset(settingsStore.defaultFocusDuration, 'focus')
        }
      }, 1000) // Small delay to let the toast show up
    }
  }

  function setPreset(seconds: number, newMode: 'focus' | 'break' | 'deep') {
    if (!canEditDuration.value) return
    plannedDurationSeconds.value = seconds
    remainingSeconds.value = seconds
    mode.value = newMode
    reset()
    if (settingsStore.autoStart) {
      start()
    }
  }

  function setCustomDuration(minutes: number) {
    if (!canEditDuration.value) return
    const safeMinutes = Math.min(240, Math.max(1, Math.floor(minutes)))
    plannedDurationSeconds.value = safeMinutes * 60
    remainingSeconds.value = safeMinutes * 60
  }

  watch(() => settingsStore.defaultFocusDuration, (newVal) => {
    if (status.value === 'idle') {
      plannedDurationSeconds.value = newVal
      remainingSeconds.value = newVal
    }
  })

  onUnmounted(() => {
    stopTicking()
  })

  return {
    status,
    mode,
    plannedDurationSeconds,
    remainingSeconds,
    progress,
    elapsedSeconds,
    startedAt,
    pausedAt,
    accumulatedPausedMs,
    canEditDuration,
    start,
    pause,
    resume,
    reset,
    setPreset,
    setCustomDuration,
    onCompleted,
  }
})
