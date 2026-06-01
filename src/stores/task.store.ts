import { defineStore } from 'pinia'
import { ref } from 'vue'
import { api } from '@/lib/tauri'
import type { Task } from '@/types/task'

export const useTaskStore = defineStore('task', () => {
  const tasks = ref<Task[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function loadTasks() {
    loading.value = true
    error.value = null
    try {
      tasks.value = await api.tasks.list()
    } catch (e: any) {
      error.value = e.toString()
    } finally {
      loading.value = false
    }
  }

  async function addTask(title: string) {
    loading.value = true
    error.value = null
    try {
      const newTask = await api.tasks.create(title)
      tasks.value.unshift(newTask)
    } catch (e: any) {
      error.value = e.toString()
      throw e
    } finally {
      loading.value = false
    }
  }

  async function toggleTask(id: string, completed: boolean) {
    loading.value = true
    error.value = null
    try {
      const updatedTask = await api.tasks.updateCompleted(id, completed)
      const index = tasks.value.findIndex(t => t.id === id)
      if (index !== -1) {
        tasks.value[index] = updatedTask
      }
      // Re-sort local tasks list (completed goes to bottom)
      sortLocalTasks()
    } catch (e: any) {
      error.value = e.toString()
    } finally {
      loading.value = false
    }
  }

  async function deleteTask(id: string) {
    loading.value = true
    error.value = null
    try {
      await api.tasks.delete(id)
      tasks.value = tasks.value.filter(t => t.id !== id)
    } catch (e: any) {
      error.value = e.toString()
    } finally {
      loading.value = false
    }
  }

  async function reorder(ids: string[]) {
    loading.value = true
    error.value = null
    try {
      tasks.value = await api.tasks.reorder(ids)
    } catch (e: any) {
      error.value = e.toString()
    } finally {
      loading.value = false
    }
  }

  function sortLocalTasks() {
    tasks.value.sort((a, b) => {
      if (a.completed === b.completed) {
        return a.sortOrder - b.sortOrder
      }
      return a.completed ? 1 : -1
    })
  }

  return {
    tasks,
    loading,
    error,
    loadTasks,
    addTask,
    toggleTask,
    deleteTask,
    reorder,
  }
})
