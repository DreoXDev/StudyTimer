<script lang="ts" setup>
import { ref } from 'vue'
import { storeToRefs } from 'pinia'
import { useUiStore } from '@/stores/ui.store'
import { useSessionStore } from '@/stores/session.store'
import { X, Calendar, Plus } from 'lucide-vue-next'
import StudyStats from './StudyStats.vue'
import SessionLog from './SessionLog.vue'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { toast } from 'vue-sonner'

const uiStore = useUiStore()
const { sessionsSidebarOpen } = storeToRefs(uiStore)

const sessionStore = useSessionStore()

const manualMinutes = ref('')
const manualNote = ref('')
const loading = ref(false)

const addManualSession = async () => {
  const mins = parseInt(manualMinutes.value, 10)
  if (isNaN(mins) || mins <= 0 || mins > 480) {
    toast.error('Inserisci una durata valida (1 - 480 minuti)')
    return
  }

  loading.value = true
  const now = new Date()
  const started = new Date(now.getTime() - mins * 60000)

  try {
    await sessionStore.createSession({
      id: Date.now().toString(),
      startedAt: started.toISOString(),
      endedAt: now.toISOString(),
      plannedDurationSeconds: mins * 60,
      actualDurationSeconds: mins * 60,
      completed: true,
      mode: 'focus',
      note: manualNote.value ? `Manuale: ${manualNote.value}` : 'Aggiunta manuale',
    })
    manualMinutes.value = ''
    manualNote.value = ''
    toast.success('Sessione manuale aggiunta con successo!')
  } catch (e) {
    console.error(e)
    toast.error("Errore durante il salvataggio della sessione.")
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div>
    <!-- Backdrop Overlay (closes sidebar when clicked) -->
    <Transition name="fade">
      <div
        v-if="sessionsSidebarOpen"
        class="fixed inset-0 z-40 bg-black/60 backdrop-blur-[2px] transition-all duration-300"
        @click="uiStore.closeSidebars()"
      ></div>
    </Transition>

    <!-- Sidebar Container -->
    <div
      class="fixed top-10 left-0 bottom-0 z-40 w-[400px] border-r border-border/50 bg-card/90 backdrop-blur-xl shadow-2xl transition-transform duration-300 ease-out flex flex-col"
      :class="sessionsSidebarOpen ? 'translate-x-0' : '-translate-x-full'"
    >
      <!-- Sidebar Header -->
      <div class="flex items-center justify-between p-5 border-b border-border/50 shrink-0">
        <div class="flex items-center gap-2">
          <Calendar class="h-4.5 w-4.5 text-primary" />
          <h2 class="text-sm font-bold uppercase tracking-wider text-foreground">Registro Studio</h2>
        </div>
        <button
          class="h-7 w-7 rounded-lg hover:bg-muted text-muted-foreground hover:text-foreground flex items-center justify-center transition-colors duration-150 cursor-pointer"
          @click="uiStore.closeSidebars()"
        >
          <X class="h-4 w-4" />
        </button>
      </div>

      <!-- Sidebar Content (Fixed stats at top, scrollable history list) -->
      <div class="flex-1 flex flex-col p-5 min-h-0 overflow-hidden gap-5">
        <!-- Statistics Section -->
        <div class="shrink-0 space-y-2">
          <h3 class="text-xs font-bold text-muted-foreground uppercase tracking-widest">Statistiche</h3>
          <StudyStats />
        </div>

        <!-- History Log List (Internal scroll inside SessionLog component) -->
        <div class="flex-1 min-h-0 flex flex-col">
          <SessionLog class="flex-1 border-0 p-0 bg-transparent shadow-none" />
        </div>
      </div>

      <!-- Sidebar Footer (Manual Add Form) -->
      <div class="p-5 border-t border-border/50 bg-background/50 shrink-0">
        <h3 class="text-xs font-bold text-muted-foreground uppercase tracking-widest mb-3">Registrazione Manuale</h3>
        <form @submit.prevent="addManualSession" class="space-y-3">
          <div class="flex gap-2">
            <div class="w-24 shrink-0">
              <Input
                v-model="manualMinutes"
                type="number"
                min="1"
                max="480"
                placeholder="Minuti"
                class="h-9 rounded-xl border-border bg-card text-xs font-semibold text-center focus-visible:ring-primary"
                required
              />
            </div>
            <div class="flex-1">
              <Input
                v-model="manualNote"
                type="text"
                placeholder="Note della sessione (es. Matematica)"
                class="h-9 rounded-xl border-border bg-card text-xs focus-visible:ring-primary"
              />
            </div>
          </div>
          <Button
            type="submit"
            size="sm"
            :disabled="loading"
            class="w-full h-9 rounded-xl bg-primary text-primary-foreground hover:bg-primary/90 text-xs font-bold tracking-wide flex items-center justify-center gap-1.5 shadow-md shadow-primary/10 cursor-pointer"
          >
            <Plus class="h-3.5 w-3.5" />
            <span>Aggiungi Sessione</span>
          </Button>
        </form>
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
