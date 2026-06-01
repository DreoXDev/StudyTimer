<script lang="ts" setup>
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
  { label: '15m Focus', seconds: 15 * 60, mode: 'focus' },
  { label: '30m Focus', seconds: 30 * 60, mode: 'focus' },
  { label: '45m Deep', seconds: 45 * 60, mode: 'deep' },
  { label: '60m Deep', seconds: 60 * 60, mode: 'deep' },
  { label: '5m Pausa', seconds: 5 * 60, mode: 'break' },
  { label: '15m Pausa', seconds: 15 * 60, mode: 'break' },
]
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
  </div>
</template>
