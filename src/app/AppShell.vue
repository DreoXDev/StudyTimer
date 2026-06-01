<script lang="ts" setup>
import { onMounted } from 'vue'
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
import { Calendar, CheckSquare } from 'lucide-vue-next'

const uiStore = useUiStore()
const sessionStore = useSessionStore()
const taskStore = useTaskStore()
const mediaStore = useMediaStore()

onMounted(async () => {
  await Promise.all([
    sessionStore.loadSessions(),
    sessionStore.loadStats(),
    taskStore.loadTasks(),
  ])
  // Fetch initial now playing state
  mediaStore.refreshNowPlaying()
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

      <!-- Header Row: Floating Sidebar Buttons & Digital Clock -->
      <div class="w-full flex items-center justify-between shrink-0 h-14 z-10">
        
        <!-- Left: Open Sessions -->
        <button
          class="h-9 px-4 flex items-center gap-2 rounded-xl border border-border/50 bg-card/45 hover:bg-muted text-xs font-semibold text-muted-foreground hover:text-foreground cursor-pointer shadow-sm active:scale-95 transition-all"
          @click="uiStore.toggleSessionsSidebar()"
        >
          <Calendar class="h-4 w-4 text-primary" />
          <span>Registro</span>
        </button>

        <!-- Center: Current Date & Clock -->
        <CurrentClock />

        <!-- Right: Open Tasks -->
        <button
          class="h-9 px-4 flex items-center gap-2 rounded-xl border border-border/50 bg-card/45 hover:bg-muted text-xs font-semibold text-muted-foreground hover:text-foreground cursor-pointer shadow-sm active:scale-95 transition-all"
          @click="uiStore.toggleTasksSidebar()"
        >
          <CheckSquare class="h-4 w-4 text-primary" />
          <span>Task List</span>
        </button>

      </div>

      <!-- Main Center Section: Focus Timer -->
      <main class="flex-1 flex items-center justify-center w-full z-10 max-h-[500px]">
        <StudyTimer />
      </main>

      <!-- Bottom Layout Section: Left NowPlaying, Center/Right empty (clean look) -->
      <div class="w-full flex items-end justify-between shrink-0 h-20 z-10">
        
        <!-- Bottom Left: System Media controller -->
        <NowPlayingCompact />

        <!-- Bottom Right (Clean Placeholder or light indicator) -->
        <div class="text-[9px] font-bold text-muted-foreground/30 uppercase tracking-widest cursor-default select-none pr-1">
          Focus Mode Active
        </div>

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
