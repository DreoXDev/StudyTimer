<script lang="ts" setup>
import { ref, computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useTaskStore } from '@/stores/task.store'
import { Plus, Trash2, CheckCircle2 } from 'lucide-vue-next'
import { Input } from '@/components/ui/input'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Checkbox } from '@/components/ui/checkbox'
import { toast } from 'vue-sonner'

const taskStore = useTaskStore()
const { tasks } = storeToRefs(taskStore)

const newTaskTitle = ref('')

const addTask = async () => {
  const title = newTaskTitle.value.trim()
  if (title) {
    try {
      await taskStore.addTask(title)
      newTaskTitle.value = ''
      toast.success('Task aggiunta!')
    } catch (e) {
      console.error("Errore nell'aggiunta del task:", e)
      toast.error("Impossibile aggiungere la task.")
    }
  }
}

const handleToggle = (id: string, currentCompleted: boolean) => {
  taskStore.toggleTask(id, !currentCompleted)
  if (!currentCompleted) {
    toast.success('Task completata!')
  } else {
    toast.info('Task contrassegnata come attiva.')
  }
}

const deleteTask = (id: string) => {
  taskStore.deleteTask(id)
  toast.info('Task eliminata.')
}

// Separate and sort tasks: incomplete above, completed below
const sortedTasks = computed(() => {
  return [...tasks.value].sort((a, b) => {
    if (a.completed === b.completed) {
      return a.sortOrder - b.sortOrder
    }
    return a.completed ? 1 : -1
  })
})
</script>

<template>
  <div class="flex flex-col h-full rounded-3xl border border-border bg-card p-5 shadow-sm overflow-hidden">
    <!-- Header -->
    <div class="flex items-center gap-2 pb-4 border-b border-border">
      <CheckCircle2 class="h-5 w-5 text-primary" />
      <h2 class="text-sm font-bold uppercase tracking-wider text-foreground">Task List</h2>
    </div>

    <!-- Add Task Input -->
    <form @submit.prevent="addTask" class="flex gap-2 my-4">
      <Input
        v-model="newTaskTitle"
        placeholder="Aggiungi una nuova task..."
        class="h-9 rounded-xl border-border bg-background text-sm flex-1 focus-visible:ring-primary"
      />
      <button
        type="submit"
        class="h-9 w-9 shrink-0 flex items-center justify-center rounded-xl bg-primary text-primary-foreground hover:bg-primary/90 transition-colors duration-200 cursor-pointer shadow-sm shadow-primary/20"
      >
        <Plus class="h-4 w-4" />
      </button>
    </form>

    <!-- Scrollable Task List -->
    <ScrollArea class="flex-1 pr-3 -mr-3">
      <div v-if="sortedTasks.length === 0" class="flex flex-col items-center justify-center py-12 text-center">
        <p class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Nessuna task presente</p>
        <p class="text-xs text-muted-foreground/60 mt-1">Aggiungi una task sopra per iniziare.</p>
      </div>

      <div v-else class="flex flex-col gap-2.5">
        <div
          v-for="task in sortedTasks"
          :key="task.id"
          class="flex items-start justify-between gap-3 p-3 rounded-xl bg-background border border-border/50 hover:border-border transition-all duration-200 group"
        >
          <div class="flex items-start gap-3 flex-1 min-w-0 mt-0.5">
            <Checkbox
              :checked="task.completed"
              class="rounded border-muted-foreground/45 text-primary focus-visible:ring-primary mt-0.5 cursor-pointer"
              @update:checked="handleToggle(task.id, task.completed)"
            />
            <span
              class="text-sm font-medium leading-tight break-words select-none cursor-pointer"
              :class="task.completed ? 'line-through text-muted-foreground/60 font-normal' : 'text-foreground'"
              @click="handleToggle(task.id, task.completed)"
            >
              {{ task.title }}
            </span>
          </div>

          <button
            class="h-6 w-6 shrink-0 flex items-center justify-center rounded-md hover:bg-muted text-muted-foreground/40 hover:text-destructive opacity-0 group-hover:opacity-100 transition-all duration-200 cursor-pointer"
            @click="deleteTask(task.id)"
          >
            <Trash2 class="h-3.5 w-3.5" />
          </button>
        </div>
      </div>
    </ScrollArea>
  </div>
</template>
