<script lang="ts" setup>
import { storeToRefs } from 'pinia'
import { useSessionStore } from '@/stores/session.store'
import { History, Trash2, CheckCircle2, AlertTriangle } from 'lucide-vue-next'
import { ScrollArea } from '@/components/ui/scroll-area'

const sessionStore = useSessionStore()
const { sessions } = storeToRefs(sessionStore)

const formatTime = (isoString: string) => {
  const date = new Date(isoString)
  return date.toLocaleTimeString('it-IT', { hour: '2-digit', minute: '2-digit' })
}

const formatMinutes = (seconds: number) => {
  return `${Math.round(seconds / 60)} min`
}

const deleteSession = (id: string) => {
  sessionStore.deleteSession(id)
}
</script>

<template>
  <div class="flex flex-col h-full rounded-3xl border border-border bg-card p-5 shadow-sm overflow-hidden">
    <!-- Header -->
    <div class="flex items-center gap-2 pb-4 border-b border-border">
      <History class="h-5 w-5 text-primary" />
      <h2 class="text-sm font-bold uppercase tracking-wider text-foreground">Registro Sessioni</h2>
    </div>

    <!-- Scrollable List -->
    <ScrollArea class="flex-1 mt-4 pr-3 -mr-3">
      <div v-if="sessions.length === 0" class="flex flex-col items-center justify-center py-12 text-center">
        <p class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Nessuna sessione registrata</p>
        <p class="text-xs text-muted-foreground/60 mt-1">Completa il tuo primo timer per vederlo qui.</p>
      </div>

      <div v-else class="flex flex-col gap-2.5">
        <div
          v-for="session in sessions"
          :key="session.id"
          class="flex items-center justify-between gap-4 p-3 rounded-xl bg-background border border-border/50 hover:border-border transition-all duration-200 group"
        >
          <!-- Info Section -->
          <div class="flex items-center gap-3 min-w-0">
            <!-- Icon Indicator -->
            <div
              class="h-8 w-8 rounded-lg flex items-center justify-center shrink-0"
              :class="
                session.completed
                  ? 'bg-primary/10 text-primary'
                  : 'bg-muted text-muted-foreground'
              "
            >
              <CheckCircle2 v-if="session.completed" class="h-4.5 w-4.5" />
              <AlertTriangle v-else class="h-4.5 w-4.5" />
            </div>

            <!-- Details -->
            <div class="min-w-0">
              <div class="flex items-center gap-2 flex-wrap">
                <span class="text-xs font-bold text-foreground">
                  {{ session.mode === 'deep' ? 'Deep Work' : session.mode === 'focus' ? 'Focus' : 'Pausa' }}
                </span>
                <span class="text-[10px] text-muted-foreground font-mono">
                  {{ formatTime(session.startedAt) }} - {{ formatTime(session.endedAt) }}
                </span>
              </div>
              <p class="text-[11px] text-muted-foreground mt-0.5 truncate">
                {{ formatMinutes(session.actualDurationSeconds) }} effettivi / {{ formatMinutes(session.plannedDurationSeconds) }} previsti
                <span v-if="session.note" class="italic opacity-80"> ({{ session.note }})</span>
              </p>
            </div>
          </div>

          <!-- Actions -->
          <button
            class="h-6 w-6 shrink-0 flex items-center justify-center rounded-md hover:bg-muted text-muted-foreground/40 hover:text-destructive opacity-0 group-hover:opacity-100 transition-all duration-200 cursor-pointer"
            @click="deleteSession(session.id)"
          >
            <Trash2 class="h-3.5 w-3.5" />
          </button>
        </div>
      </div>
    </ScrollArea>
  </div>
</template>
