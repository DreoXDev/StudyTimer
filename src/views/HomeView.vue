<script lang="ts" setup>
import CurrentClock from '@/components/clock/CurrentClock.vue'
import StudyTimer from '@/components/timer/StudyTimer.vue'
import NowPlayingCompact from '@/components/spotify/NowPlayingCompact.vue'
</script>

<template>
  <div class="flex-1 w-full relative flex flex-col justify-between items-center py-6 px-8 h-[calc(100vh-40px)] select-none">
    
    <!-- Top Ambient Gradient Background Effects -->
    <div class="absolute -top-40 left-1/4 right-1/4 h-80 rounded-full bg-primary/3 blur-[120px] pointer-events-none"></div>
    <div class="absolute -bottom-40 left-5 h-80 w-80 rounded-full bg-primary/2 blur-[100px] pointer-events-none"></div>

    <!-- Header Row: Digital Clock (Centered) -->
    <div class="w-full flex items-center justify-center shrink-0 h-14 z-10">
      <CurrentClock />
    </div>

    <!-- Main Center Section: Focus Timer -->
    <main class="flex-1 flex items-center justify-center w-full z-10 max-h-[500px]">
      <StudyTimer />
    </main>

    <!-- Bottom Layout Section: Left NowPlaying, Center/Right empty (clean look) -->
    <div class="w-full flex items-end justify-between shrink-0 h-20 z-10 pointer-events-none">
      
      <!-- Bottom Left: System Media controller -->
      <NowPlayingCompact />

      <!-- Bottom Right is empty for a cleaner look -->
      <div class="w-10"></div>

    </div>

  </div>
</template>
