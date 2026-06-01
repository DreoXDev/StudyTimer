<script lang="ts" setup>
import { computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useTimerStore } from '@/stores/timer.store'
import TimerRing from './TimerRing.vue'
import TimerControls from './TimerControls.vue'
import TimerPresetPicker from './TimerPresetPicker.vue'
import { Badge } from '@/components/ui/badge'

const timerStore = useTimerStore()
const {
  status,
  mode,
  plannedDurationSeconds,
  remainingSeconds,
  progress,
} = storeToRefs(timerStore)

const timeFormatted = computed(() => {
  const mins = Math.floor(remainingSeconds.value / 60).toString().padStart(2, '0')
  const secs = (remainingSeconds.value % 60).toString().padStart(2, '0')
  return `${mins}:${secs}`
})

const modeLabel = computed(() => {
  switch (mode.value) {
    case 'focus': return 'Sessione Focus'
    case 'deep': return 'Deep Work'
    case 'break': return 'Pausa Studio'
  }
})

const modeBadgeVariant = computed(() => {
  switch (mode.value) {
    case 'focus': return 'default'
    case 'deep': return 'destructive'
    case 'break': return 'secondary'
  }
})

const selectPreset = (payload: { seconds: number; mode: 'focus' | 'break' | 'deep' }) => {
  timerStore.setPreset(payload.seconds, payload.mode)
}
</script>

<template>
  <div class="flex flex-col items-center justify-between h-full rounded-3xl border border-border bg-card p-6 shadow-sm relative overflow-hidden">
    <!-- Ambient Background Red Glow (active when timer running) -->
    <div
      class="absolute inset-0 bg-primary/2 pointer-events-none transition-opacity duration-1000 ease-in-out"
      :class="status === 'running' ? 'opacity-100' : 'opacity-0'"
    ></div>

    <!-- Timer Header -->
    <div class="z-10 flex flex-col items-center gap-1.5 mt-2">
      <Badge :variant="modeBadgeVariant" class="px-3.5 py-1 rounded-full uppercase tracking-wider font-semibold text-[10px]">
        {{ modeLabel }}
      </Badge>
    </div>

    <!-- Timer Central Ring -->
    <div class="z-10 my-4 flex items-center justify-center">
      <TimerRing :progress="progress" :status="status">
        <div class="flex flex-col items-center">
          <span class="font-mono text-5xl font-bold tracking-tight tabular-nums select-none">
            {{ timeFormatted }}
          </span>
          <span class="text-xs font-semibold text-muted-foreground uppercase tracking-widest mt-1 select-none">
            {{ status }}
          </span>
        </div>
      </TimerRing>
    </div>

    <!-- Controls & Presets -->
    <div class="z-10 w-full flex flex-col gap-6 items-center mb-2">
      <TimerControls
        :status="status"
        @start="timerStore.start()"
        @pause="timerStore.pause()"
        @resume="timerStore.resume()"
        @reset="timerStore.reset()"
      />
      <TimerPresetPicker
        :current-duration-seconds="plannedDurationSeconds"
        :current-mode="mode"
        :disabled="status === 'running' || status === 'paused'"
        @select-preset="selectPreset"
      />
    </div>
  </div>
</template>
