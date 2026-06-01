<script lang="ts" setup>
import { computed } from 'vue'
import { useNow } from '@vueuse/core'
import { Clock } from 'lucide-vue-next'

const now = useNow()

const timeStr = computed(() => {
  const hours = now.value.getHours().toString().padStart(2, '0')
  const minutes = now.value.getMinutes().toString().padStart(2, '0')
  return `${hours}:${minutes}`
})

const secondsStr = computed(() => {
  return now.value.getSeconds().toString().padStart(2, '0')
})

const dateStr = computed(() => {
  const formatted = now.value.toLocaleDateString('it-IT', {
    weekday: 'long',
    day: 'numeric',
    month: 'long',
  })
  // Capitalize the first letter
  return formatted.charAt(0).toUpperCase() + formatted.slice(1)
})
</script>

<template>
  <div class="flex items-center gap-4 rounded-2xl border border-border bg-card p-5 shadow-sm">
    <div class="flex h-12 w-12 items-center justify-center rounded-xl bg-primary/10 text-primary">
      <Clock class="h-6 w-6" />
    </div>
    <div>
      <div class="flex items-baseline font-mono text-3xl font-bold tracking-tight">
        <span>{{ timeStr }}</span>
        <span class="text-sm font-medium text-muted-foreground ml-1">:{{ secondsStr }}</span>
      </div>
      <p class="text-sm font-medium text-muted-foreground mt-0.5">
        {{ dateStr }}
      </p>
    </div>
  </div>
</template>
