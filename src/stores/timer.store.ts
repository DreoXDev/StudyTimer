import { defineStore } from 'pinia'
import { ref, computed, onUnmounted } from 'vue'
import { useSessionStore } from './session.store'
import { toast } from 'vue-sonner'

export const useTimerStore = defineStore('timer', () => {
  const status = ref<'idle' | 'running' | 'paused' | 'completed'>('idle')
  const mode = ref<'focus' | 'break' | 'deep'>('focus')
  const plannedDurationSeconds = ref(25 * 60)
  const remainingSeconds = ref(25 * 60)

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
        sessionStore.createSession({
          id: Date.now().toString(),
          startedAt: new Date(startedAt.value).toISOString(),
          endedAt: new Date().toISOString(),
          plannedDurationSeconds: plannedDurationSeconds.value,
          actualDurationSeconds: elapsed,
          completed: false,
          mode: mode.value,
          note: 'Interrotta manualmente',
        })
        .then(() => toast.info('Sessione interrotta salvata nel registro.'))
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
      remainingSeconds.value = 0
      status.value = 'completed'
      stopTicking()
      onTimerCompleted()
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

  // This will be overridden or extended in Phase E to call Tauri commands
  const onCompletedCallbacks: Array<() => void> = []
  function onCompleted(callback: () => void) {
    onCompletedCallbacks.push(callback)
  }

  function onTimerCompleted() {
    // Play sound notification
    try {
      const audio = new Audio('https://assets.mixkit.co/active_storage/sfx/2869/2869-84.wav')
      audio.volume = 0.3
      audio.play()
    } catch (e) {
      console.log('Audio playback not supported or blocked')
    }

    // Save completed session to SQLite
    if (startedAt.value !== null) {
      const sessionStore = useSessionStore()
      sessionStore.createSession({
        id: Date.now().toString(),
        startedAt: new Date(startedAt.value).toISOString(),
        endedAt: new Date().toISOString(),
        plannedDurationSeconds: plannedDurationSeconds.value,
        actualDurationSeconds: plannedDurationSeconds.value,
        completed: true,
        mode: mode.value,
      })
      .then(() => toast.success('Sessione completata e salvata nel registro!'))
      .catch(err => console.error('Errore nel salvataggio della sessione completata:', err))
    }

    // Trigger registered callbacks
    for (const cb of onCompletedCallbacks) {
      cb()
    }
  }

  function setPreset(seconds: number, newMode: 'focus' | 'break' | 'deep') {
    plannedDurationSeconds.value = seconds
    remainingSeconds.value = seconds
    mode.value = newMode
    reset()
  }

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
    start,
    pause,
    resume,
    reset,
    setPreset,
    onCompleted,
  }
})
