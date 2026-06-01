<script lang="ts" setup>
import { Play, Pause, RotateCcw } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'

defineProps({
  status: {
    type: String,
    required: true, // 'idle' | 'running' | 'paused' | 'completed'
  },
})

defineEmits(['start', 'pause', 'resume', 'reset'])
</script>

<template>
  <div class="flex items-center justify-center gap-3">
    <!-- Reset Button (only when running or paused) -->
    <Button
      v-if="status === 'running' || status === 'paused'"
      variant="outline"
      size="icon"
      class="h-12 w-12 rounded-xl border-border bg-card hover:bg-muted/80 hover:text-foreground text-muted-foreground transition-colors duration-200"
      @click="$emit('reset')"
    >
      <RotateCcw class="h-5 w-5" />
    </Button>

    <!-- Start / Pause / Resume Button -->
    <Button
      v-if="status === 'idle' || status === 'completed'"
      variant="default"
      class="h-12 px-6 rounded-xl font-semibold tracking-medium bg-primary text-primary-foreground hover:bg-primary/90 shadow-md shadow-primary/20 flex items-center gap-2 min-w-[120px] transition-all duration-200"
      @click="$emit('start')"
    >
      <Play class="h-4 w-4 fill-current" />
      <span>Avvia</span>
    </Button>

    <Button
      v-else-if="status === 'running'"
      variant="secondary"
      class="h-12 px-6 rounded-xl font-semibold tracking-medium bg-secondary text-secondary-foreground hover:bg-muted border border-border flex items-center gap-2 min-w-[120px] transition-all duration-200"
      @click="$emit('pause')"
    >
      <Pause class="h-4 w-4 fill-current" />
      <span>Pausa</span>
    </Button>

    <Button
      v-else-if="status === 'paused'"
      variant="default"
      class="h-12 px-6 rounded-xl font-semibold tracking-medium bg-primary text-primary-foreground hover:bg-primary/90 shadow-md shadow-primary/20 flex items-center gap-2 min-w-[120px] transition-all duration-200"
      @click="$emit('resume')"
    >
      <Play class="h-4 w-4 fill-current" />
      <span>Riprendi</span>
    </Button>
  </div>
</template>
