<script lang="ts" setup>
import { storeToRefs } from 'pinia'
import { useUiStore } from '@/stores/ui.store'
import { X, CheckSquare } from 'lucide-vue-next'
import TaskPanel from './TaskPanel.vue'

const uiStore = useUiStore()
const { tasksSidebarOpen } = storeToRefs(uiStore)
</script>

<template>
  <div>
    <!-- Backdrop Overlay (closes sidebar when clicked) -->
    <Transition name="fade">
      <div
        v-if="tasksSidebarOpen"
        class="fixed inset-0 z-40 bg-black/60 backdrop-blur-[2px] transition-all duration-300"
        @click="uiStore.closeSidebars()"
      ></div>
    </Transition>

    <!-- Sidebar Container -->
    <div
      class="fixed top-10 right-0 bottom-0 z-40 w-[380px] border-l border-border/50 bg-card/90 backdrop-blur-xl shadow-2xl transition-transform duration-300 ease-out flex flex-col"
      :class="tasksSidebarOpen ? 'translate-x-0' : 'translate-x-full'"
    >
      <!-- Sidebar Header -->
      <div class="flex items-center justify-between p-5 border-b border-border/50 shrink-0">
        <div class="flex items-center gap-2">
          <CheckSquare class="h-4.5 w-4.5 text-primary" />
          <h2 class="text-sm font-bold uppercase tracking-wider text-foreground">Focus Tasks</h2>
        </div>
        <button
          class="h-7 w-7 rounded-lg hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center transition-colors duration-150 cursor-pointer"
          @click="uiStore.closeSidebars()"
        >
          <X class="h-4 w-4" />
        </button>
      </div>

      <!-- Sidebar Content (Scrollable TaskPanel) -->
      <div class="flex-1 overflow-hidden">
        <!-- Re-use TaskPanel but strip its outer styling inside the stylesheet or by setting classes -->
        <TaskPanel class="border-0 bg-transparent rounded-none p-5 shadow-none" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
