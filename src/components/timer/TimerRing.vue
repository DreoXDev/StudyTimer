<script lang="ts" setup>
import { computed } from 'vue'

const props = defineProps({
  progress: {
    type: Number,
    required: true,
  },
  status: {
    type: String,
    required: true, // 'idle' | 'running' | 'paused' | 'completed'
  },
})

const size = 300
const strokeWidth = 8
const radius = (size - strokeWidth * 2) / 2
const circumference = 2 * Math.PI * radius

const strokeDashoffset = computed(() => {
  return circumference - (props.progress / 100) * circumference
})
</script>

<template>
  <div class="relative flex items-center justify-center">
    <svg :width="size" :height="size" class="transform -rotate-90">
      <!-- Background Circle Track -->
      <circle
        class="text-muted/20"
        stroke="currentColor"
        :stroke-width="strokeWidth"
        fill="transparent"
        :r="radius"
        :cx="size / 2"
        :cy="size / 2"
      />
      <!-- Active Progress Circle -->
      <circle
        class="text-primary transition-all duration-300 ease-out"
        :class="{
          'animate-pulse': status === 'running',
          'drop-shadow-[0_0_8px_rgba(239,68,68,0.5)]': status === 'running',
          'opacity-50': status === 'paused',
        }"
        stroke="currentColor"
        :stroke-width="strokeWidth"
        stroke-linecap="round"
        fill="transparent"
        :r="radius"
        :cx="size / 2"
        :cy="size / 2"
        :stroke-dasharray="circumference"
        :stroke-dashoffset="strokeDashoffset"
      />
    </svg>
    <!-- Slot inside the ring (usually for the clock digits) -->
    <div class="absolute flex flex-col items-center justify-center">
      <slot />
    </div>
  </div>
</template>

<style scoped>
@keyframes pulse {
  0%, 100% {
    opacity: 1;
    filter: drop-shadow(0 0 8px rgba(239,68,68,0.6));
  }
  50% {
    opacity: 0.85;
    filter: drop-shadow(0 0 14px rgba(239,68,68,0.3));
  }
}

.animate-pulse {
  animation: pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}
</style>
