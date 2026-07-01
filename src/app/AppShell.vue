<script lang="ts" setup>
import { onMounted, onUnmounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import AppTitleBar from '../components/window/AppTitleBar.vue'
import SessionsSidebar from '../components/sessions/SessionsSidebar.vue'
import TasksSidebar from '../components/tasks/TasksSidebar.vue'
import { useUiStore } from '@/stores/ui.store'
import { useSessionStore } from '@/stores/session.store'
import { useTaskStore } from '@/stores/task.store'
import { useMediaStore } from '@/stores/media.store'
import { useSyncStore } from '@/stores/sync.store'
import { useTimerStore } from '@/stores/timer.store'
import { useSettingsStore } from '@/stores/settings.store'
import { listen } from '@tauri-apps/api/event'

const router = useRouter()
const route = useRoute()
const uiStore = useUiStore()
const sessionStore = useSessionStore()
const taskStore = useTaskStore()
const mediaStore = useMediaStore()
const syncStore = useSyncStore()
const settingsStore = useSettingsStore()

let syncIntervalId: number | null = null
let unlistenToggle: (() => void) | null = null

/** Returns true if user is currently typing in an input/textarea/contenteditable */
function isTyping(e: KeyboardEvent): boolean {
  const target = e.target as HTMLElement | null
  if (!target) return false
  const tag = target.tagName.toLowerCase()
  return tag === 'input' || tag === 'textarea' || target.isContentEditable
}

const handleKeyDown = (e: KeyboardEvent) => {
  // F11 – fullscreen (always)
  if (e.key === 'F11') {
    e.preventDefault()
    uiStore.toggleFullscreen()
    return
  }
  if (e.key === 'Escape' && uiStore.isFullscreen) {
    uiStore.setFullscreen(false)
    return
  }

  // Navigation shortcuts (not while typing)
  if (!isTyping(e) && !e.ctrlKey && !e.altKey && !e.metaKey) {
    switch (e.key.toLowerCase()) {
      case 's':
        e.preventDefault()
        router.push('/stats')
        break
      case ',':
        e.preventDefault()
        router.push('/settings')
        break
      case 'f':
        e.preventDefault()
        router.push('/')
        break
    }

    // Space — start/pause timer (only on focus page, if enabled in settings)
    if (e.code === 'Space' && route.path === '/' && settingsStore.spaceStartStop) {
      e.preventDefault()
      const timerStore = useTimerStore()
      if (timerStore.status === 'running') {
        timerStore.pause()
      } else if (timerStore.status === 'idle' || timerStore.status === 'paused') {
        timerStore.start()
      }
    }

    // R — reset timer (only on focus page, if enabled in settings)
    if (e.key.toLowerCase() === 'r' && route.path === '/' && settingsStore.rReset) {
      e.preventDefault()
      const timerStore = useTimerStore()
      timerStore.reset()
    }
  }

  // Ctrl+, → settings
  if (e.key === ',' && e.ctrlKey) {
    e.preventDefault()
    router.push('/settings')
  }
}

onMounted(async () => {
  settingsStore.init()
  window.addEventListener('keydown', handleKeyDown)
  await Promise.all([
    sessionStore.loadSessions(),
    sessionStore.loadStats(),
    taskStore.loadTasks(),
    syncStore.init()
  ])
  
  // Perform initial sync if user is logged in and autoSync is enabled
  if (syncStore.isAuthenticated && settingsStore.autoSync) {
    try {
      const synced = await syncStore.sync()
      if (synced) {
        await Promise.all([
          sessionStore.loadSessions(),
          sessionStore.loadStats()
        ])
      }
    } catch (e) {
      console.error('Errore durante sync iniziale all\'avvio:', e)
    }
  }

  // Set up periodic background sync every 5 minutes (if autoSync is enabled)
  syncIntervalId = window.setInterval(async () => {
    if (syncStore.isAuthenticated && !syncStore.syncing && settingsStore.autoSync) {
      const synced = await syncStore.sync()
      if (synced) {
        await Promise.all([
          sessionStore.loadSessions(),
          sessionStore.loadStats()
        ])
      }
    }
  }, 5 * 60 * 1000)

  // Listen for tray timer toggle events
  unlistenToggle = await listen('tray-timer-toggle', () => {
    const timerStore = useTimerStore()
    if (timerStore.status === 'running') {
      timerStore.pause()
    } else {
      timerStore.start()
    }
  })

  mediaStore.refreshNowPlaying()
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown)
  if (syncIntervalId !== null) {
    clearInterval(syncIntervalId)
  }
  if (unlistenToggle) {
    unlistenToggle()
  }
})
</script>

<template>
  <div class="h-screen w-screen overflow-hidden bg-background text-foreground flex flex-col relative select-none">
    
    <!-- Custom Windows Titlebar -->
    <AppTitleBar class="shrink-0" />

    <!-- Sidebars Overlay Components (always active) -->
    <SessionsSidebar />
    <TasksSidebar />

    <!-- Main Viewport (Router) -->
    <div class="flex-1 w-full relative overflow-hidden">
      <RouterView v-slot="{ Component }">
        <Transition name="page" mode="out-in">
          <component :is="Component" />
        </Transition>
      </RouterView>
    </div>
  </div>
</template>

<style>
/* Global scrollbar adjustments for a premium look */
::-webkit-scrollbar {
  width: 5px;
  height: 5px;
}
::-webkit-scrollbar-track {
  background: transparent;
}
::-webkit-scrollbar-thumb {
  background: rgba(239, 68, 68, 0.1);
  border-radius: 99px;
}
::-webkit-scrollbar-thumb:hover {
  background: rgba(239, 68, 68, 0.25);
}
</style>
