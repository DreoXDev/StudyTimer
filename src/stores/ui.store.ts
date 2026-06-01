import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useUiStore = defineStore('ui', () => {
  const sessionsSidebarOpen = ref(false)
  const tasksSidebarOpen = ref(false)

  function toggleSessionsSidebar() {
    sessionsSidebarOpen.value = !sessionsSidebarOpen.value
    if (sessionsSidebarOpen.value) {
      tasksSidebarOpen.value = false
    }
  }

  function toggleTasksSidebar() {
    tasksSidebarOpen.value = !tasksSidebarOpen.value
    if (tasksSidebarOpen.value) {
      sessionsSidebarOpen.value = false
    }
  }

  function closeSidebars() {
    sessionsSidebarOpen.value = false
    tasksSidebarOpen.value = false
  }

  return {
    sessionsSidebarOpen,
    tasksSidebarOpen,
    toggleSessionsSidebar,
    toggleTasksSidebar,
    closeSidebars,
  }
})
