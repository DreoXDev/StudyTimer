<script lang="ts" setup>
import { computed } from 'vue'
import { useNow } from '@vueuse/core'

const now = useNow()

const timeStr = computed(() => {
  const hours = now.value.getHours().toString().padStart(2, '0')
  const minutes = now.value.getMinutes().toString().padStart(2, '0')
  return `${hours}:${minutes}`
})

const dateStr = computed(() => {
  const formatted = now.value.toLocaleDateString('it-IT', {
    weekday: 'short',
    day: 'numeric',
    month: 'short',
  })
  return formatted.toUpperCase()
})
</script>

<template>
  <div class="flex flex-col items-center select-none text-center">
    <span class="font-mono text-xl font-bold tracking-tight text-foreground/70 tabular-nums">
      {{ timeStr }}
    </span>
    <span class="text-[9px] font-bold text-muted-foreground/45 uppercase tracking-widest mt-0.5">
      {{ dateStr }}
    </span>
  </div>
</template>
