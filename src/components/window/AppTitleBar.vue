<script lang="ts" setup>
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useRoute } from 'vue-router'
import { Minimize2, Maximize2, Minus, Square, X, Calendar, CheckSquare } from 'lucide-vue-next'
import { useUiStore } from '@/stores/ui.store'
import { storeToRefs } from 'pinia'
import SmokingCounterDropdown from '@/components/tracking/SmokingCounterDropdown.vue'

const appWindow = getCurrentWindow()
const uiStore = useUiStore()
const route = useRoute()
const { sessionsSidebarOpen, tasksSidebarOpen, isFullscreen } = storeToRefs(uiStore)

const minimizeWindow = async () => {
  await appWindow.minimize()
}

const toggleMaximizeWindow = async () => {
  await appWindow.toggleMaximize()
}

const closeWindow = async () => {
  await appWindow.close()
}

const navItems = [
  { label: 'Focus', path: '/' },
  { label: 'Stats', path: '/stats' },
  { label: 'Habits', path: '/habits' },
  { label: 'Workouts', path: '/workouts' },
  { label: 'Settings', path: '/settings' },
]
</script>

<template>
  <div
    class="h-10 w-full flex items-center justify-between border-b border-border/50 bg-background/85 backdrop-blur-md select-none relative z-50 text-foreground"
    data-tauri-drag-region
  >
    <!-- Left Section: Title, Icon & Smoking Counter -->
    <div class="flex items-center gap-3 pl-4 h-full" data-tauri-drag-region>
      <div class="flex items-center gap-2" data-tauri-drag-region>
        <div class="h-2 w-2 rounded-full bg-primary animate-pulse"></div>
        <span class="text-xs font-bold uppercase tracking-wider text-muted-foreground cursor-default font-sans" data-tauri-drag-region>
          Study Timer
        </span>
      </div>
      <SmokingCounterDropdown />
    </div>

    <!-- Center Section: Navigation & Sidebar Toggles -->
    <div class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 flex items-center justify-center z-40">
      <!-- Left Sidebar Toggle (Sessioni) — only on Focus page -->
      <div class="w-24 flex justify-end mr-2">
        <button
          v-if="route.path === '/'"
          class="h-7 px-3 flex items-center gap-1.5 rounded-lg text-xs font-semibold cursor-pointer transition-all duration-200"
          :class="
            sessionsSidebarOpen
              ? 'bg-primary/10 text-primary border border-primary/20'
              : 'hover:bg-muted text-muted-foreground hover:text-foreground border border-transparent'
          "
          @click="uiStore.toggleSessionsSidebar()"
        >
          <Calendar class="h-3.5 w-3.5" />
          <span>Sessioni</span>
        </button>
      </div>

      <!-- Main Nav Toggle -->
      <div class="flex items-center bg-muted/40 p-0.5 rounded-lg border border-border/30">
        <RouterLink
          v-for="item in navItems"
          :key="item.path"
          :to="item.path"
          class="h-7 px-3 rounded-md text-xs font-semibold transition-all cursor-pointer flex items-center justify-center font-sans no-underline"
          :class="
            route.path === item.path
              ? 'bg-background text-foreground shadow-sm border border-border/30'
              : 'text-muted-foreground hover:text-foreground'
          "
        >
          {{ item.label }}
        </RouterLink>
      </div>

      <!-- Right Sidebar Toggle (Task) — only on Focus page -->
      <div class="w-24 flex justify-start ml-2">
        <button
          v-if="route.path === '/'"
          class="h-7 px-3 flex items-center gap-1.5 rounded-lg text-xs font-semibold cursor-pointer transition-all duration-200"
          :class="
            tasksSidebarOpen
              ? 'bg-primary/10 text-primary border border-primary/20'
              : 'hover:bg-muted text-muted-foreground hover:text-foreground border border-transparent'
          "
          @click="uiStore.toggleTasksSidebar()"
        >
          <CheckSquare class="h-3.5 w-3.5" />
          <span>Task</span>
        </button>
      </div>
    </div>

    <!-- Right Section: Fullscreen, Minimize, Maximize, Close -->
    <div class="flex items-center h-full">
      <!-- Full Focus (Fullscreen Toggle) -->
      <button
        class="h-full w-10 flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-muted transition-colors duration-150 cursor-pointer"
        :title="isFullscreen ? 'Esci da Full Focus' : 'Attiva Full Focus'"
        @click="uiStore.toggleFullscreen()"
      >
        <Minimize2 v-if="isFullscreen" class="h-3.5 w-3.5 text-primary" />
        <Maximize2 v-else class="h-3.5 w-3.5" />
      </button>

      <button
        class="h-full w-10 flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-muted transition-colors duration-150 cursor-pointer"
        title="Riduci a icona"
        @click="minimizeWindow"
      >
        <Minus class="h-3.5 w-3.5" />
      </button>

      <button
        class="h-full w-10 flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-muted transition-colors duration-150 cursor-pointer"
        title="Ingrandisci"
        @click="toggleMaximizeWindow"
      >
        <Square class="h-3 w-3" />
      </button>

      <button
        class="h-full w-12 flex items-center justify-center text-muted-foreground hover:text-white hover:bg-destructive transition-colors duration-150 cursor-pointer"
        title="Chiudi"
        @click="closeWindow"
      >
        <X class="h-4 w-4" />
      </button>
    </div>
  </div>
</template>
