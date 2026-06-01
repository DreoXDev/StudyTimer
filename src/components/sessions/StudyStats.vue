<script lang="ts" setup>
import { storeToRefs } from 'pinia'
import { useSessionStore } from '@/stores/session.store'
import { BarChart3, Timer, Calendar } from 'lucide-vue-next'

const sessionStore = useSessionStore()
const { stats } = storeToRefs(sessionStore)

const formatDuration = (minutes: number) => {
  const h = Math.floor(minutes / 60)
  const m = minutes % 60
  if (h === 0) return `${m}m`
  return `${h}h ${m}m`
}
</script>

<template>
  <div class="grid grid-cols-3 gap-4 h-full">
    <!-- Today Time Card -->
    <div class="flex items-center gap-3 rounded-2xl border border-border bg-card p-4 shadow-sm">
      <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-primary/10 text-primary">
        <Timer class="h-5 w-5" />
      </div>
      <div class="overflow-hidden">
        <p class="text-xs font-semibold text-muted-foreground uppercase tracking-wider truncate">Studio Oggi</p>
        <p class="text-lg font-bold tracking-tight mt-0.5 truncate">{{ formatDuration(stats.todayMinutes) }}</p>
      </div>
    </div>

    <!-- Today Sessions Card -->
    <div class="flex items-center gap-3 rounded-2xl border border-border bg-card p-4 shadow-sm">
      <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-primary/10 text-primary">
        <BarChart3 class="h-5 w-5" />
      </div>
      <div class="overflow-hidden">
        <p class="text-xs font-semibold text-muted-foreground uppercase tracking-wider truncate">Sessioni</p>
        <p class="text-lg font-bold tracking-tight mt-0.5 truncate">{{ stats.todaySessionsCount }}</p>
      </div>
    </div>

    <!-- Week Time Card -->
    <div class="flex items-center gap-3 rounded-2xl border border-border bg-card p-4 shadow-sm">
      <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-primary/10 text-primary">
        <Calendar class="h-5 w-5" />
      </div>
      <div class="overflow-hidden">
        <p class="text-xs font-semibold text-muted-foreground uppercase tracking-wider truncate">Ultimi 7 Giorni</p>
        <p class="text-lg font-bold tracking-tight mt-0.5 truncate">{{ formatDuration(stats.weekMinutes) }}</p>
      </div>
    </div>
  </div>
</template>
