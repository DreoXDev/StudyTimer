import { defineStore } from 'pinia'
import { ref } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

export const useUiStore = defineStore('ui', () => {
  const sessionsSidebarOpen = ref(false)
  const tasksSidebarOpen = ref(false)
  const isFullscreen = ref(false)
  const currentView = ref<'home' | 'stats'>('home')

  const appWindow = getCurrentWindow()

  function setView(view: 'home' | 'stats') {
    currentView.value = view
    sessionsSidebarOpen.value = false
    tasksSidebarOpen.value = false
  }

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

  async function toggleFullscreen() {
    const next = !isFullscreen.value
    if (next) {
      try {
        const isMaximized = await appWindow.isMaximized()
        if (isMaximized) {
          await appWindow.unmaximize()
        }
      } catch (e) {
        console.error('Errore nel verificare/unmaximize finestra:', e)
      }
    }
    await appWindow.setFullscreen(next)
    isFullscreen.value = next
  }

  async function setFullscreen(value: boolean) {
    if (value) {
      try {
        const isMaximized = await appWindow.isMaximized()
        if (isMaximized) {
          await appWindow.unmaximize()
        }
      } catch (e) {
        console.error('Errore nel verificare/unmaximize finestra:', e)
      }
    }
    await appWindow.setFullscreen(value)
    isFullscreen.value = value
  }

  return {
    sessionsSidebarOpen,
    tasksSidebarOpen,
    isFullscreen,
    currentView,
    setView,
    toggleSessionsSidebar,
    toggleTasksSidebar,
    closeSidebars,
    toggleFullscreen,
    setFullscreen,
  }
})
