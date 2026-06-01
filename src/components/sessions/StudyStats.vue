<script lang="ts" setup>
import { storeToRefs } from 'pinia'
import { useSessionStore } from '@/stores/session.store'

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
  <div class="grid grid-cols-3 gap-2">
    <!-- Today Time Card -->
    <div class="flex flex-col items-center justify-center rounded-xl border border-border bg-background/55 py-2 px-1 text-center shadow-sm">
      <span class="text-[9px] font-bold text-muted-foreground uppercase tracking-widest leading-tight">Oggi</span>
      <span class="text-sm font-bold tracking-tight text-foreground mt-1 tabular-nums leading-none">
        {{ formatDuration(stats.todayMinutes) }}
      </span>
    </div>

    <!-- Today Sessions Card -->
    <div class="flex flex-col items-center justify-center rounded-xl border border-border bg-background/55 py-2 px-1 text-center shadow-sm">
      <span class="text-[9px] font-bold text-muted-foreground uppercase tracking-widest leading-tight">Sessioni</span>
      <span class="text-sm font-bold tracking-tight text-foreground mt-1 tabular-nums leading-none">
        {{ stats.todaySessionsCount }}
      </span>
    </div>

    <!-- Week Time Card -->
    <div class="flex flex-col items-center justify-center rounded-xl border border-border bg-background/55 py-2 px-1 text-center shadow-sm">
      <span class="text-[9px] font-bold text-muted-foreground uppercase tracking-widest leading-tight">7 Giorni</span>
      <span class="text-sm font-bold tracking-tight text-foreground mt-1 tabular-nums leading-none">
        {{ formatDuration(stats.weekMinutes) }}
      </span>
    </div>
  </div>
</template>
