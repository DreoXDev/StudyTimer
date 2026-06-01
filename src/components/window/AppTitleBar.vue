<script lang="ts" setup>
import { getCurrentWindow } from '@tauri-apps/api/window'
import { Minimize2, Maximize2, Minus, Square, X, Calendar, CheckSquare } from 'lucide-vue-next'
import { useUiStore } from '@/stores/ui.store'
import { storeToRefs } from 'pinia'

const appWindow = getCurrentWindow()
const uiStore = useUiStore()
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
</script>

<template>
  <div
    class="h-10 w-full flex items-center justify-between border-b border-border/50 bg-background/85 backdrop-blur-md select-none relative z-50 text-foreground"
    data-tauri-drag-region
  >
    <!-- Left Section: Title & Icon -->
    <div class="flex items-center gap-2 pl-4 h-full" data-tauri-drag-region>
      <div class="h-2 w-2 rounded-full bg-primary animate-pulse"></div>
      <span class="text-xs font-bold uppercase tracking-wider text-muted-foreground cursor-default" data-tauri-drag-region>
        Study Timer
      </span>
    </div>

    <!-- Center Section: Sidebar Toggles (not draggable to allow clicking) -->
    <div class="flex items-center gap-1.5 h-full">
      <button
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

      <button
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
