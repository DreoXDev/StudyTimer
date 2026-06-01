<script lang="ts" setup>
import { onMounted, onUnmounted } from 'vue'
import AppTitleBar from '../components/window/AppTitleBar.vue'
import SessionsSidebar from '../components/sessions/SessionsSidebar.vue'
import TasksSidebar from '../components/tasks/TasksSidebar.vue'
import CurrentClock from '../components/clock/CurrentClock.vue'
import StudyTimer from '../components/timer/StudyTimer.vue'
import NowPlayingCompact from '../components/spotify/NowPlayingCompact.vue'
import { useUiStore } from '@/stores/ui.store'
import { useSessionStore } from '@/stores/session.store'
import { useTaskStore } from '@/stores/task.store'
import { useMediaStore } from '@/stores/media.store'

const uiStore = useUiStore()
const sessionStore = useSessionStore()
const taskStore = useTaskStore()
const mediaStore = useMediaStore()

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
  ])
  mediaStore.refreshNowPlaying()
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown)
})
</script>

<template>
  <div class="h-screen w-screen overflow-hidden bg-background text-foreground flex flex-col relative select-none">
    
    <!-- Custom Windows Titlebar -->
    <AppTitleBar class="shrink-0" />

    <!-- Sidebars Overlay Components -->
    <SessionsSidebar />
    <TasksSidebar />

    <!-- Focus Environment Main Viewport -->
    <div class="flex-1 w-full relative flex flex-col justify-between items-center py-6 px-8 h-[calc(100vh-40px)]">
      
      <!-- Top Ambient Gradient Background Effects -->
      <div class="absolute -top-40 left-1/4 right-1/4 h-80 rounded-full bg-primary/3 blur-[120px] pointer-events-none"></div>
      <div class="absolute -bottom-40 left-5 h-80 w-80 rounded-full bg-primary/2 blur-[100px] pointer-events-none"></div>

      <!-- Header Row: Digital Clock (Centered) -->
      <div class="w-full flex items-center justify-center shrink-0 h-14 z-10">
        <CurrentClock />
      </div>

      <!-- Main Center Section: Focus Timer -->
      <main class="flex-1 flex items-center justify-center w-full z-10 max-h-[500px]">
        <StudyTimer />
      </main>

      <!-- Bottom Layout Section: Left NowPlaying, Center/Right empty (clean look) -->
      <div class="w-full flex items-end justify-between shrink-0 h-20 z-10">
        
        <!-- Bottom Left: System Media controller -->
        <NowPlayingCompact />

        <!-- Bottom Right is empty for a cleaner look -->
        <div class="w-10"></div>

      </div>

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
