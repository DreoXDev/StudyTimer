<script lang="ts" setup>
import { onMounted, onUnmounted } from 'vue'
import AppTitleBar from '../components/window/AppTitleBar.vue'
import SessionsSidebar from '../components/sessions/SessionsSidebar.vue'
import TasksSidebar from '../components/tasks/TasksSidebar.vue'
import HomeView from '../views/HomeView.vue'
import StatsView from '../views/StatsView.vue'
import { useUiStore } from '@/stores/ui.store'
import { useSessionStore } from '@/stores/session.store'
import { useTaskStore } from '@/stores/task.store'
import { useMediaStore } from '@/stores/media.store'
import { useSmokingStore } from '@/stores/smoking.store'
import { useSyncStore } from '@/stores/sync.store'

const uiStore = useUiStore()
const sessionStore = useSessionStore()
const taskStore = useTaskStore()
const mediaStore = useMediaStore()
const smokingStore = useSmokingStore()
const syncStore = useSyncStore()

let syncIntervalId: number | null = null

const handleKeyDown = (e: KeyboardEvent) => {
  if (e.key === 'F11') {
    e.preventDefault()
    uiStore.toggleFullscreen()
  } else if (e.key === 'Escape') {
    if (uiStore.isFullscreen) {
      uiStore.setFullscreen(false)
    }
  }
}

onMounted(async () => {
  window.addEventListener('keydown', handleKeyDown)
  await Promise.all([
    sessionStore.loadSessions(),
    sessionStore.loadStats(),
    taskStore.loadTasks(),
    smokingStore.loadSmokingTodayCount(),
    syncStore.init()
  ])
  
  // Perform initial sync if user is logged in
  if (syncStore.isAuthenticated) {
    try {
      const synced = await syncStore.sync()
      if (synced) {
        // Refresh local data to reflect cloud updates
        await Promise.all([
          sessionStore.loadSessions(),
          sessionStore.loadStats(),
          smokingStore.loadSmokingTodayCount()
        ])
      }
    } catch (e) {
      console.error('Errore durante sync iniziale all\'avvio:', e)
    }
  }

  // Set up periodic background sync every 5 minutes
  syncIntervalId = window.setInterval(async () => {
    if (syncStore.isAuthenticated && !syncStore.syncing) {
      const synced = await syncStore.sync()
      if (synced) {
        await Promise.all([
          sessionStore.loadSessions(),
          sessionStore.loadStats(),
          smokingStore.loadSmokingTodayCount()
        ])
      }
    }
  }, 5 * 60 * 1000)

  mediaStore.refreshNowPlaying()
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown)
  if (syncIntervalId !== null) {
    clearInterval(syncIntervalId)
  }
})
</script>

<template>
  <div class="h-screen w-screen overflow-hidden bg-background text-foreground flex flex-col relative select-none">
    
    <!-- Custom Windows Titlebar -->
    <AppTitleBar class="shrink-0" />

    <!-- Sidebars Overlay Components (only active in Home/Focus view) -->
    <template v-if="uiStore.currentView === 'home'">
      <SessionsSidebar />
      <TasksSidebar />
    </template>

    <!-- Main Viewport -->
    <div class="flex-1 w-full relative">
      <HomeView v-if="uiStore.currentView === 'home'" />
      <StatsView v-else />
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

