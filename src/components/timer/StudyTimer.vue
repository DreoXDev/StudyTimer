<script lang="ts" setup>
import { ref, computed, nextTick } from 'vue'
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
  canEditDuration,
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

// Manual Input Setup (Fase 6)
const isEditing = ref(false)
const editMinutes = ref(Math.floor(plannedDurationSeconds.value / 60))
const editInput = ref<HTMLInputElement | null>(null)

const startEditing = () => {
  if (canEditDuration.value) {
    editMinutes.value = Math.floor(plannedDurationSeconds.value / 60)
    isEditing.value = true
    nextTick(() => {
      editInput.value?.focus()
      editInput.value?.select()
    })
  }
}

const saveEditing = () => {
  if (!isEditing.value) return
  const mins = parseInt(editMinutes.value.toString(), 10)
  if (!isNaN(mins) && mins > 0 && mins <= 480) {
    timerStore.setPreset(mins * 60, mins >= 45 ? 'deep' : 'focus')
  }
  isEditing.value = false
}
</script>

<template>
  <div class="flex flex-col items-center justify-between h-full py-8 bg-transparent select-none relative">
    
    <!-- Timer Header Mode Badge -->
    <div class="flex flex-col items-center gap-1.5 shrink-0">
      <Badge :variant="modeBadgeVariant" class="px-3.5 py-1 rounded-full uppercase tracking-wider font-semibold text-[10px]">
        {{ modeLabel }}
      </Badge>
    </div>
 
    <!-- Timer Central Ring (Glow circle) -->
    <div class="my-6 flex items-center justify-center flex-1">
      <TimerRing :progress="progress" :status="status">
        <div class="flex flex-col items-center">
          
          <!-- Manual Input / Interactive Digits -->
          <div v-if="isEditing" class="flex flex-col items-center">
            <input
              ref="editInput"
              v-model="editMinutes"
              type="number"
              min="1"
              max="480"
              class="w-24 text-center bg-transparent border-b border-primary/50 focus:border-primary text-5xl font-bold font-mono tracking-tight text-foreground focus:outline-none tabular-nums"
              @keydown.enter="saveEditing"
              @blur="saveEditing"
            />
            <span class="text-[9px] font-bold text-muted-foreground uppercase tracking-widest mt-1.5">Modifica min</span>
          </div>
          
          <div 
            v-else 
            class="flex flex-col items-center select-none" 
            :class="canEditDuration ? 'cursor-pointer group' : 'cursor-not-allowed opacity-75'" 
            @click="startEditing" 
            :title="canEditDuration ? 'Clicca per inserire i minuti a mano' : ''"
          >
            <span 
              class="font-mono text-6xl font-bold tracking-tight tabular-nums transition-colors"
              :class="canEditDuration ? 'group-hover:text-primary' : ''"
            >
              {{ timeFormatted }}
            </span>
            <span class="text-[9px] font-bold text-muted-foreground uppercase tracking-widest mt-1.5 transition-opacity duration-200" :class="canEditDuration ? 'opacity-100' : 'opacity-50'">
              {{ status === 'idle' ? 'Imposta' : status }}
            </span>
          </div>
 
        </div>
      </TimerRing>
    </div>

    <!-- Controls & Presets (Pill layout) -->
    <div class="w-full flex flex-col gap-6 items-center shrink-0">
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
        class="scale-95"
      />
    </div>
  </div>
</template>

<style scoped>
/* Remove standard input spinners */
input[type=number]::-webkit-inner-spin-button, 
input[type=number]::-webkit-outer-spin-button { 
  -webkit-appearance: none; 
  margin: 0; 
}
input[type=number] {
  -moz-appearance: textfield;
}
</style>
