<script lang="ts" setup>
import { storeToRefs } from 'pinia'
import { useSpotifyStore } from '@/stores/spotify.store'
import { Music, Play, Pause, SkipBack, SkipForward, Radio, Loader2 } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'

const spotifyStore = useSpotifyStore()
const { connected, loading, track } = storeToRefs(spotifyStore)

const formatTime = (ms?: number) => {
  if (ms === undefined) return '0:00'
  const totalSecs = Math.floor(ms / 1000)
  const mins = Math.floor(totalSecs / 60)
  const secs = (totalSecs % 60).toString().padStart(2, '0')
  return `${mins}:${secs}`
}
</script>

<template>
  <div class="flex flex-col justify-between h-full rounded-3xl border border-border bg-card p-5 shadow-sm relative overflow-hidden">
    <!-- Spotify Logo Watermark/Icon in the top right -->
    <div class="absolute top-4 right-4 text-emerald-500/10">
      <Radio class="h-8 w-8" />
    </div>

    <!-- Header -->
    <div class="flex items-center justify-between pb-3 border-b border-border">
      <div class="flex items-center gap-2">
        <div class="h-2 w-2 rounded-full" :class="connected ? 'bg-emerald-500 animate-pulse' : 'bg-muted-foreground/30'"></div>
        <h2 class="text-xs font-bold uppercase tracking-wider text-foreground">Spotify Connection</h2>
      </div>
      <span class="text-[10px] font-semibold text-muted-foreground uppercase tracking-widest bg-muted/30 px-2 py-0.5 rounded-md">
        {{ connected ? 'Connected' : 'Offline' }}
      </span>
    </div>

    <!-- Main Widget Body -->
    <div class="flex-1 flex flex-col justify-center my-4">
      <div v-if="!connected" class="flex flex-col items-center justify-center py-4 text-center">
        <div class="h-12 w-12 rounded-full bg-emerald-500/10 text-emerald-500 flex items-center justify-center mb-3">
          <Music class="h-6 w-6" />
        </div>
        <p class="text-xs font-bold text-foreground">Spotify scollegato</p>
        <p class="text-[11px] text-muted-foreground mt-1 max-w-[200px]">
          Connetti il tuo account Spotify per visualizzare la traccia in riproduzione.
        </p>
        
        <!-- Connection button with loader -->
        <Button
          size="sm"
          variant="outline"
          :disabled="loading"
          class="h-8 mt-3 rounded-lg border-emerald-500/30 text-emerald-500 bg-emerald-500/5 hover:bg-emerald-500/10 hover:text-emerald-400 text-xs font-semibold px-4 cursor-pointer transition-colors duration-200"
          @click="spotifyStore.connect()"
        >
          <Loader2 v-if="loading" class="mr-2 h-3.5 w-3.5 animate-spin" />
          <span>{{ loading ? 'Connessione...' : 'Connetti Spotify' }}</span>
        </Button>
      </div>

      <div v-else class="flex items-center gap-3.5">
        <!-- Album Art Box -->
        <div class="h-16 w-16 rounded-xl bg-gradient-to-br from-emerald-600 to-emerald-950 flex items-center justify-center shadow-md relative overflow-hidden shrink-0 group">
          <div class="absolute inset-0 bg-black/20 group-hover:bg-black/10 transition-colors"></div>
          <Music class="h-7 w-7 text-emerald-200/90" />
        </div>

        <!-- Track Info & Controls -->
        <div class="flex-1 min-w-0">
          <p class="text-sm font-bold text-foreground truncate select-none">{{ track.title }}</p>
          <p class="text-xs text-muted-foreground mt-0.5 truncate select-none">{{ track.artist }}</p>

          <!-- Controls -->
          <div class="flex items-center gap-3 mt-3">
            <button class="text-muted-foreground hover:text-foreground cursor-pointer transition-colors" disabled>
              <SkipBack class="h-4 w-4 fill-current" />
            </button>
            <button
              class="h-7 w-7 rounded-full bg-emerald-500 text-black flex items-center justify-center hover:scale-105 hover:bg-emerald-400 cursor-pointer active:scale-95 transition-all shadow-sm"
              @click="spotifyStore.togglePlay()"
            >
              <Play v-if="!track.isPlaying" class="h-3 w-3 fill-current ml-0.5" />
              <Pause v-else class="h-3 w-3 fill-current" />
            </button>
            <button class="text-muted-foreground hover:text-foreground cursor-pointer transition-colors" disabled>
              <SkipForward class="h-4 w-4 fill-current" />
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Progress Bar (Visible only when connected) -->
    <div v-if="connected" class="mt-1">
      <div class="w-full h-1 bg-muted rounded-full overflow-hidden">
        <div
          class="h-full bg-emerald-500 rounded-full transition-all duration-300"
          :style="{ width: `${((track.progressMs || 0) / (track.durationMs || 1)) * 100}%` }"
        ></div>
      </div>
      <div class="flex justify-between items-center text-[10px] text-muted-foreground font-mono font-medium mt-1.5">
        <span>{{ formatTime(track.progressMs) }}</span>
        <span>{{ formatTime(track.durationMs) }}</span>
      </div>
    </div>
    <!-- Simple Mock Disconnect Trigger -->
    <div v-else class="h-5"></div>
  </div>
</template>
