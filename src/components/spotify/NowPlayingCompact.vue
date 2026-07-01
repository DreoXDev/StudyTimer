<script lang="ts" setup>
import { storeToRefs } from 'pinia'
import { useMediaStore } from '@/stores/media.store'
import { Music, Play, Pause, SkipBack, SkipForward, Radio } from 'lucide-vue-next'

const mediaStore = useMediaStore()
const { current } = storeToRefs(mediaStore)
</script>

<template>
  <div
    v-if="current && current.available"
    class="w-72 h-[72px] rounded-2xl border border-border bg-card/60 backdrop-blur-md px-3.5 py-3 shadow-md flex items-center justify-between gap-3 relative overflow-hidden group select-none pointer-events-auto"
  >
    <!-- Background thin progress bar at the very bottom -->
    <div class="absolute bottom-0 left-0 right-0 h-[2px] bg-muted/20">
      <div
        class="h-full bg-primary transition-all duration-1000 ease-linear"
        :style="{ width: `${((current.progressMs || 0) / (current.durationMs || 1)) * 100}%` }"
      ></div>
    </div>

    <!-- Artwork Box -->
    <div class="h-[44px] w-[44px] rounded-lg bg-gradient-to-br from-primary/30 to-primary/5 flex items-center justify-center shrink-0 shadow-sm border border-border/40 text-primary">
      <Radio class="h-5 w-5" />
    </div>

    <!-- Text Metadata -->
    <div class="flex-1 min-w-0 flex flex-col justify-center">
      <span class="text-xs font-bold text-foreground truncate block leading-tight">
        {{ current.title || 'In riproduzione...' }}
      </span>
      <span class="text-[10px] font-semibold text-muted-foreground truncate block mt-0.5 leading-tight">
        {{ current.artist || 'Sconosciuto' }} <span v-if="current.source" class="opacity-40 font-mono">· {{ current.source }}</span>
      </span>
    </div>

    <!-- Controls Mini -->
    <div class="flex items-center gap-1.5 shrink-0 z-10 pr-0.5">
      <button
        class="h-6 w-6 rounded-md flex items-center justify-center hover:bg-muted text-muted-foreground hover:text-foreground cursor-pointer active:scale-95 transition-all"
        title="Traccia precedente"
        @click="mediaStore.previous()"
      >
        <SkipBack class="h-3.5 w-3.5 fill-current" />
      </button>

      <button
        class="h-7 w-7 rounded-full bg-primary text-primary-foreground flex items-center justify-center hover:scale-105 hover:bg-primary/95 shadow-sm shadow-primary/20 cursor-pointer active:scale-95 transition-all"
        :title="current.isPlaying ? 'Pausa' : 'Riproduci'"
        @click="mediaStore.playPause()"
      >
        <Play v-if="!current.isPlaying" class="h-3 w-3 fill-current ml-0.5" />
        <Pause v-else class="h-3 w-3 fill-current" />
      </button>

      <button
        class="h-6 w-6 rounded-md flex items-center justify-center hover:bg-muted text-muted-foreground hover:text-foreground cursor-pointer active:scale-95 transition-all"
        title="Traccia successiva"
        @click="mediaStore.next()"
      >
        <SkipForward class="h-3.5 w-3.5 fill-current" />
      </button>
    </div>
  </div>

  <div
    v-else
    class="w-72 h-[72px] rounded-2xl border border-border/50 bg-card/40 backdrop-blur-md px-4 py-3 flex items-center gap-3 text-muted-foreground opacity-60 pointer-events-auto"
  >
    <Music class="h-5 w-5 shrink-0 text-muted-foreground/40" />
    <div class="flex flex-col">
      <span class="text-xs font-bold leading-tight">Nessun media</span>
      <span class="text-[9px] font-semibold uppercase tracking-widest mt-0.5 leading-none">System audio off</span>
    </div>
  </div>
</template>
