<script lang="ts" setup>
import { ref, computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useUiStore } from '@/stores/ui.store'
import { useTaskStore } from '@/stores/task.store'
import { X, CheckSquare, Plus, Trash2, CheckCircle2, Circle, ChevronDown, ChevronRight } from 'lucide-vue-next'
import { Input } from '@/components/ui/input'
import { ScrollArea } from '@/components/ui/scroll-area'
import { toast } from 'vue-sonner'

const uiStore = useUiStore()
const { tasksSidebarOpen } = storeToRefs(uiStore)

const taskStore = useTaskStore()
const { tasks } = storeToRefs(taskStore)

const newTaskTitle = ref('')
const showCompleted = ref(false)

const activeTasks = computed(() => tasks.value.filter(t => !t.completed))
const completedTasks = computed(() => tasks.value.filter(t => t.completed))

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

const toggleTask = (id: string, currentCompleted: boolean) => {
  taskStore.toggleTask(id, !currentCompleted)
  if (!currentCompleted) {
    toast.success('Task completata!')
  } else {
    toast.info('Task riattivata.')
  }
}

const deleteTask = (id: string) => {
  taskStore.deleteTask(id)
  toast.info('Task eliminata.')
}
</script>

<template>
  <div>
    <!-- Backdrop Overlay -->
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
        <div class="flex flex-col gap-0.5">
          <div class="flex items-center gap-2">
            <CheckSquare class="h-4.5 w-4.5 text-primary" />
            <h2 class="text-sm font-bold uppercase tracking-wider text-foreground">Focus Tasks</h2>
          </div>
          <span class="text-[10px] font-semibold text-muted-foreground">
            {{ activeTasks.length }} attivi · {{ completedTasks.length }} completati
          </span>
        </div>
        <button
          class="h-7 w-7 rounded-lg hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center transition-colors duration-150 cursor-pointer"
          @click="uiStore.closeSidebars()"
        >
          <X class="h-4 w-4" />
        </button>
      </div>

      <!-- Add Task Form (Fixed at top) -->
      <div class="p-5 border-b border-border/30 shrink-0 bg-background/20">
        <form @submit.prevent="addTask" class="flex gap-2">
          <Input
            v-model="newTaskTitle"
            placeholder="Aggiungi una task..."
            class="h-9 rounded-xl border-border bg-card text-xs font-medium flex-1 focus-visible:ring-primary focus-visible:ring-offset-0"
          />
          <button
            type="submit"
            class="h-9 w-9 shrink-0 flex items-center justify-center rounded-xl bg-primary text-primary-foreground hover:bg-primary/95 transition-all duration-200 cursor-pointer shadow-md shadow-primary/10 active:scale-95"
          >
            <Plus class="h-4 w-4" />
          </button>
        </form>
      </div>

      <!-- Task List Content (Scrollable) -->
      <div class="flex-1 overflow-hidden flex flex-col min-h-0">
        <ScrollArea class="flex-1 px-5 py-4">
          <!-- Empty State -->
          <div v-if="tasks.length === 0" class="flex flex-col items-center justify-center py-16 text-center">
            <CheckSquare class="h-8 w-8 text-muted-foreground/30 mb-2" />
            <p class="text-xs font-bold text-muted-foreground uppercase tracking-wider">Nessuna task</p>
            <p class="text-[10px] text-muted-foreground/60 mt-1 max-w-[200px]">Aggiungi un piccolo obiettivo per iniziare a concentrarti.</p>
          </div>

          <div v-else class="flex flex-col gap-4">
            <!-- Active Tasks Section -->
            <div v-if="activeTasks.length > 0" class="flex flex-col gap-2">
              <span class="text-[10px] font-bold text-muted-foreground uppercase tracking-widest mb-1 block">In Corso</span>
              
              <div
                v-for="task in activeTasks"
                :key="task.id"
                class="flex items-center justify-between gap-3 p-3 rounded-xl bg-background/50 border border-border/50 hover:border-border transition-all duration-200 group"
              >
                <div class="flex items-center gap-2.5 min-w-0 flex-1 cursor-pointer" @click="toggleTask(task.id, task.completed)">
                  <button class="text-muted-foreground/50 hover:text-primary transition-colors shrink-0">
                    <Circle class="h-4 w-4" />
                  </button>
                  <span class="text-xs font-medium text-foreground truncate block select-none">
                    {{ task.title }}
                  </span>
                </div>
                <button
                  class="h-6 w-6 shrink-0 flex items-center justify-center rounded-md hover:bg-muted text-muted-foreground/45 hover:text-destructive opacity-0 group-hover:opacity-100 transition-all duration-200 cursor-pointer"
                  @click="deleteTask(task.id)"
                >
                  <Trash2 class="h-3.5 w-3.5" />
                </button>
              </div>
            </div>

            <!-- Completed Tasks Section -->
            <div v-if="completedTasks.length > 0" class="flex flex-col gap-2">
              <button
                class="flex items-center gap-1 text-[10px] font-bold text-muted-foreground uppercase tracking-widest hover:text-foreground transition-colors text-left w-full cursor-pointer py-1"
                @click="showCompleted = !showCompleted"
              >
                <ChevronDown v-if="showCompleted" class="h-3 w-3" />
                <ChevronRight v-else class="h-3 w-3" />
                <span>Completati ({{ completedTasks.length }})</span>
              </button>

              <div v-if="showCompleted" class="flex flex-col gap-2 transition-all duration-300">
                <div
                  v-for="task in completedTasks"
                  :key="task.id"
                  class="flex items-center justify-between gap-3 p-3 rounded-xl bg-background/25 border border-border/30 opacity-60 group"
                >
                  <div class="flex items-center gap-2.5 min-w-0 flex-1 cursor-pointer" @click="toggleTask(task.id, task.completed)">
                    <button class="text-primary shrink-0">
                      <CheckCircle2 class="h-4 w-4 fill-primary/10" />
                    </button>
                    <span class="text-xs font-normal text-muted-foreground line-through truncate block select-none">
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
            </div>
          </div>
        </ScrollArea>
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
