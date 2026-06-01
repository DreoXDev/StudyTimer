<script lang="ts" setup>
import { ref } from 'vue'
import { Input } from '@/components/ui/input'

interface Preset {
  label: string
  seconds: number
  mode: 'focus' | 'break' | 'deep'
}

defineProps({
  currentDurationSeconds: {
    type: Number,
    required: true,
  },
  currentMode: {
    type: String,
    required: true, // 'focus' | 'break' | 'deep'
  },
  disabled: {
    type: Boolean,
    default: false,
  },
})

const emit = defineEmits(['select-preset'])

const presets: Preset[] = [
  { label: '25m Focus', seconds: 25 * 60, mode: 'focus' },
  { label: '50m Deep', seconds: 50 * 60, mode: 'deep' },
  { label: '90m Study', seconds: 90 * 60, mode: 'deep' },
  { label: '5m Break', seconds: 5 * 60, mode: 'break' },
  { label: '15m Break', seconds: 15 * 60, mode: 'break' },
]

const customMinutes = ref('')

const handleCustomSubmit = () => {
  const mins = parseInt(customMinutes.value, 10)
  if (!isNaN(mins) && mins > 0 && mins <= 480) {
    emit('select-preset', {
      seconds: mins * 60,
      mode: mins >= 45 ? 'deep' : 'focus',
    })
    customMinutes.value = ''
  }
}
</script>

<template>
  <div class="flex flex-col gap-4">
    <!-- Presets Grid -->
    <div class="flex flex-wrap gap-2 justify-center">
      <button
        v-for="preset in presets"
        :key="preset.label"
        :disabled="disabled"
        class="px-3.5 py-1.5 rounded-full text-xs font-semibold border transition-all duration-200 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
        :class="
          currentDurationSeconds === preset.seconds && currentMode === preset.mode
            ? 'bg-primary text-primary-foreground border-primary shadow-sm shadow-primary/25'
            : 'bg-card border-border hover:bg-muted/80 text-muted-foreground hover:text-foreground'
        "
        @click="emit('select-preset', { seconds: preset.seconds, mode: preset.mode })"
      >
        {{ preset.label }}
      </button>
    </div>

    <!-- Custom Input -->
    <div class="flex items-center justify-center gap-2">
      <span class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Minuti personalizzati:</span>
      <form @submit.prevent="handleCustomSubmit" class="flex items-center gap-1.5">
        <Input
          v-model="customMinutes"
          type="number"
          min="1"
          max="480"
          :disabled="disabled"
          placeholder="25"
          class="h-8 w-14 rounded-lg bg-card border-border text-center font-semibold text-xs py-0 px-1 disabled:opacity-50"
        />
        <button
          type="submit"
          :disabled="disabled || !customMinutes"
          class="h-8 px-3 rounded-lg bg-secondary text-secondary-foreground border border-border text-xs font-semibold hover:bg-muted cursor-pointer transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          Applica
        </button>
      </form>
    </div>
  </div>
</template>
