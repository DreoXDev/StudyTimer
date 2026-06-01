<script lang="ts" setup>
import { onMounted } from 'vue'
import { useSessionStore } from '@/stores/session.store'
import { useTaskStore } from '@/stores/task.store'
import CurrentClock from '../components/clock/CurrentClock.vue'
import StudyStats from '../components/sessions/StudyStats.vue'
import StudyTimer from '../components/timer/StudyTimer.vue'
import TaskPanel from '../components/tasks/TaskPanel.vue'
import NowPlaying from '../components/spotify/NowPlaying.vue'
import SessionLog from '../components/sessions/SessionLog.vue'

const sessionStore = useSessionStore()
const taskStore = useTaskStore()

onMounted(async () => {
  await Promise.all([
    sessionStore.loadSessions(),
    sessionStore.loadStats(),
    taskStore.loadTasks(),
  ])
})
</script>

<template>
  <div class="min-h-screen w-screen overflow-hidden bg-background text-foreground select-none">
    <!-- Grid container spanning exactly screen height to be ideal for second monitors -->
    <div class="mx-auto grid h-screen max-w-[1600px] grid-cols-12 grid-rows-12 gap-4 p-6">
      
      <!-- Top Left: Current Clock -->
      <section class="col-span-4 row-span-2">
        <CurrentClock />
      </section>

      <!-- Top Center: Study Stats -->
      <section class="col-span-4 col-start-5 row-span-2">
        <StudyStats />
      </section>

      <!-- Right Column: Task List (spans 8 rows, full height down to bottom sections) -->
      <aside class="col-span-4 col-start-9 row-span-8 row-start-1">
        <TaskPanel />
      </aside>

      <!-- Main Center: Large Study Timer -->
      <section class="col-span-8 col-start-1 row-span-6 row-start-3">
        <StudyTimer />
      </section>

      <!-- Bottom Left: Spotify Connection -->
      <section class="col-span-4 row-span-4 row-start-9">
        <NowPlaying />
      </section>

      <!-- Bottom Right: Session Log -->
      <section class="col-span-8 col-start-5 row-span-4 row-start-9">
        <SessionLog />
      </section>
      
    </div>
  </div>
</template>
