<script lang="ts" setup>
import { ref, computed, onMounted, watch } from 'vue'
import {
  Clock,
  Calendar,
  RefreshCw,
  Trash2,
  TrendingUp,
  CheckSquare,
  Cloud
} from 'lucide-vue-next'

import { api } from '@/lib/tauri'
import { useTaskStore } from '@/stores/task.store'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { toast } from 'vue-sonner'
import type { TrackingEvent, TrackingSummary } from '@/types/tracking'

const taskStore = useTaskStore()
const rangeType = ref<'today' | 'week' | 'month' | 'year' | 'custom'>('week')
const customStartDate = ref('')
const customEndDate = ref('')

const summary = ref<TrackingSummary | null>(null)
const recentEvents = ref<TrackingEvent[]>([])
const loading = ref(false)

// Helper: Calculate range dates in ISO strings
function calculateBoundaries() {
  const now = new Date()
  let start = new Date()
  let end = new Date()
  let granularity = 'day'

  switch (rangeType.value) {
    case 'today': {
      start.setHours(0, 0, 0, 0)
      end.setHours(23, 59, 59, 999)
      granularity = 'hour'
      break
    }
    case 'week': {
      start.setDate(now.getDate() - 6)
      start.setHours(0, 0, 0, 0)
      end.setHours(23, 59, 59, 999)
      granularity = 'day'
      break
    }
    case 'month': {
      start.setDate(now.getDate() - 29)
      start.setHours(0, 0, 0, 0)
      end.setHours(23, 59, 59, 999)
      granularity = 'day'
      break
    }
    case 'year': {
      start.setDate(now.getDate() - 364)
      start.setHours(0, 0, 0, 0)
      end.setHours(23, 59, 59, 999)
      granularity = 'month'
      break
    }
    case 'custom': {
      if (customStartDate.value) {
        start = new Date(customStartDate.value)
        start.setHours(0, 0, 0, 0)
      } else {
        start.setDate(now.getDate() - 7)
        start.setHours(0, 0, 0, 0)
      }
      if (customEndDate.value) {
        end = new Date(customEndDate.value)
        end.setHours(23, 59, 59, 999)
      } else {
        end.setHours(23, 59, 59, 999)
      }
      
      const diffDays = Math.ceil((end.getTime() - start.getTime()) / (1000 * 60 * 60 * 24))
      if (diffDays <= 2) {
        granularity = 'hour'
      } else if (diffDays <= 60) {
        granularity = 'day'
      } else {
        granularity = 'month'
      }
      break
    }
  }

  return {
    start: start.toISOString(),
    end: end.toISOString(),
    granularity
  }
}

async function loadData() {
  loading.value = true
  try {
    const { start, end, granularity } = calculateBoundaries()
    
    // Fetch Summary
    summary.value = await api.tracking.getSummary(start, end, granularity)
    
    // Fetch Recent Events (only study_sessions)
    recentEvents.value = await api.tracking.listEvents('study_session', start, end, 50)
  } catch (e: any) {
    console.error('Errore nel caricamento delle statistiche:', e)
    toast.error('Errore nel caricamento dei dati di tracciamento.')
  } finally {
    loading.value = false
  }
}

watch(rangeType, () => {
  if (rangeType.value !== 'custom') {
    loadData()
  }
})

// Watch custom dates to reload data when changed
watch([customStartDate, customEndDate], () => {
  if (rangeType.value === 'custom' && customStartDate.value && customEndDate.value) {
    loadData()
  }
})

onMounted(async () => {
  await loadData()
  await taskStore.loadTasks()
})

// Metrics computation
const totalStudyHours = computed(() => {
  if (!summary.value) return '0h'
  const secs = summary.value.study.totalSeconds
  const hrs = Math.floor(secs / 3600)
  const mins = Math.floor((secs % 3600) / 60)
  return hrs > 0 ? `${hrs}h ${mins}m` : `${mins}m`
})

const averageSessionMins = computed(() => {
  if (!summary.value) return '0 min'
  const secs = summary.value.study.averageSessionSeconds
  return `${Math.round(secs / 60)} min`
})

// Task Metrics
const totalTasksCount = computed(() => taskStore.tasks.length)
const completedTasksCount = computed(() => taskStore.tasks.filter(t => t.completed).length)
const taskCompletionRate = computed(() => {
  if (totalTasksCount.value === 0) return 0
  return Math.round((completedTasksCount.value / totalTasksCount.value) * 100)
})

// Filter completed tasks by range boundaries
const tasksCompletedInPeriod = computed(() => {
  const { start, end } = calculateBoundaries()
  const startTime = new Date(start).getTime()
  const endTime = new Date(end).getTime()
  
  return taskStore.tasks.filter(t => {
    if (!t.completed || !t.completedAt) return false
    const compTime = new Date(t.completedAt).getTime()
    return compTime >= startTime && compTime <= endTime
  })
})

const tasksAddedInPeriod = computed(() => {
  const { start, end } = calculateBoundaries()
  const startTime = new Date(start).getTime()
  const endTime = new Date(end).getTime()
  
  return taskStore.tasks.filter(t => {
    const createTime = new Date(t.createdAt).getTime()
    return createTime >= startTime && createTime <= endTime
  })
})

// Group completed tasks by bucket
const tasksCompletedByBucket = computed(() => {
  if (!summary.value || taskStore.tasks.length === 0) return []
  const granularity = calculateBoundaries().granularity
  
  return summary.value.study.byBucket.map(b => {
    const bucketStr = b.bucket // e.g. "2026-07-01" or "2026-07-01T11"
    let count = 0
    
    for (const task of taskStore.tasks) {
      if (task.completed && task.completedAt) {
        const compDate = task.completedAt // ISO string
        if (granularity === 'day') {
          if (compDate.substring(0, 10) === bucketStr.substring(0, 10)) {
            count++
          }
        } else if (granularity === 'hour') {
          if (compDate.substring(0, 13) === bucketStr.substring(0, 13)) {
            count++
          }
        } else if (granularity === 'month') {
          if (compDate.substring(0, 7) === bucketStr.substring(0, 7)) {
            count++
          }
        }
      }
    }
    
    return {
      bucket: b.bucket,
      count
    }
  })
})

// Max values for chart scaling
const maxStudySeconds = computed(() => {
  if (!summary.value || summary.value.study.byBucket.length === 0) return 1
  return Math.max(...summary.value.study.byBucket.map(b => b.seconds), 1)
})

const maxCompletedTasks = computed(() => {
  if (tasksCompletedByBucket.value.length === 0) return 1
  return Math.max(...tasksCompletedByBucket.value.map(b => b.count), 1)
})

// Format Bucket labels
function formatBucketLabel(bucket: string, granularity: string) {
  if (!bucket) return ''
  try {
    if (granularity === 'hour') {
      const parts = bucket.split('T')
      if (parts.length > 1) {
        return parts[1] + ':00'
      }
      return bucket.substring(11, 13) + ':00'
    } else if (granularity === 'day') {
      const date = new Date(bucket)
      if (isNaN(date.getTime())) {
        return bucket.substring(8, 10)
      }
      return `${date.getDate()}/${date.getMonth() + 1}`
    } else if (granularity === 'month') {
      const parts = bucket.split('-')
      if (parts.length > 1) {
        const monthNum = parseInt(parts[1], 10)
        const months = ['Gen', 'Feb', 'Mar', 'Apr', 'Mag', 'Giu', 'Lug', 'Ago', 'Set', 'Ott', 'Nov', 'Dic']
        return months[monthNum - 1] || bucket
      }
      return bucket
    }
  } catch (e) {
    return bucket
  }
  return bucket
}

// Format study duration in chart tooltip
function formatSeconds(secs: number) {
  const hrs = Math.floor(secs / 3600)
  const mins = Math.round((secs % 3600) / 60)
  return hrs > 0 ? `${hrs}h ${mins}m` : `${mins}m`
}

// Delete Event Handlers
async function handleDeleteEvent(id: string) {
  if (confirm('Sei sicuro di voler eliminare questa sessione permanentemente?')) {
    try {
      await api.tracking.deleteEvent(id)
      toast.success('Log eliminato correttamente.')
      await loadData()
    } catch (e) {
      console.error(e)
      toast.error('Errore durante l\'eliminazione del log.')
    }
  }
}

// Event styling helpers
function getEventBadgeClass(type: string) {
  switch (type) {
    case 'study_session':
      return 'bg-green-500/10 border-green-500/20 text-green-400'
    case 'timer_interrupted':
      return 'bg-yellow-500/10 border-yellow-500/20 text-yellow-400'
    default:
      return 'bg-muted border-border text-muted-foreground'
  }
}

function getEventLabel(type: string) {
  switch (type) {
    case 'study_session': return 'Studio'
    case 'timer_interrupted': return 'Interrotto'
    default: return type
  }
}

</script>

<template>
  <div class="flex-1 w-full h-[calc(100vh-40px)] overflow-y-auto bg-background text-foreground py-6 px-8 select-none relative">
    <!-- Background subtle ambient glow -->
    <div class="absolute top-1/4 left-1/3 right-1/3 h-96 rounded-full bg-primary/2 blur-[150px] pointer-events-none"></div>

    <!-- Stats Page Header -->
    <div class="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-border/40 pb-5 mb-6 z-10 relative">
      <div>
        <h1 class="text-xl font-bold tracking-tight text-foreground flex items-center gap-2">
          <TrendingUp class="h-5 w-5 text-primary" />
          Statistiche & Tracker
        </h1>
        <p class="text-xs text-muted-foreground">Analizza le sessioni di studio, le task completate ed esporta i log locali.</p>
      </div>

      <!-- Range Picker Button Group -->
      <div class="flex flex-wrap items-center bg-muted/40 p-0.5 rounded-lg border border-border/30 gap-px">
        <button
          v-for="rt in ['today', 'week', 'month', 'year', 'custom']"
          :key="rt"
          class="h-7 px-3 rounded-md text-xs font-semibold capitalize transition-all cursor-pointer"
          :class="
            rangeType === rt
              ? 'bg-background text-foreground shadow-sm border border-border/30'
              : 'text-muted-foreground hover:text-foreground'
          "
          @click="rangeType = rt as any"
        >
          {{ rt === 'today' ? 'oggi' : rt === 'week' ? 'settimana' : rt === 'month' ? 'mese' : rt === 'year' ? 'anno' : 'custom' }}
        </button>
      </div>
    </div>

    <!-- Custom Range Picker Inputs -->
    <div v-if="rangeType === 'custom'" class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-6 p-4 rounded-xl border border-border/30 bg-card/20 backdrop-blur-sm z-10 relative">
      <div class="space-y-1.5">
        <Label for="custom-start" class="text-xs text-muted-foreground">Data Inizio</Label>
        <Input id="custom-start" type="date" v-model="customStartDate" class="h-9 text-xs border-border/50 bg-background/50 text-foreground font-mono focus:border-primary/50" />
      </div>
      <div class="space-y-1.5">
        <Label for="custom-end" class="text-xs text-muted-foreground">Data Fine</Label>
        <Input id="custom-end" type="date" v-model="customEndDate" class="h-9 text-xs border-border/50 bg-background/50 text-foreground font-mono focus:border-primary/50" />
      </div>
    </div>

    <!-- Top Metrics Overview -->
    <div class="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-6 z-10 relative">
      <!-- Tempo Studio -->
      <Card class="bg-card/30 border-border/30 backdrop-blur-sm shadow-sm">
        <CardContent class="p-4 flex items-center gap-3">
          <div class="p-2.5 rounded-lg bg-green-500/10 text-green-400">
            <Clock class="h-5 w-5" />
          </div>
          <div>
            <p class="text-[10px] text-muted-foreground uppercase tracking-wider font-semibold">Tempo Studio</p>
            <h3 class="text-lg font-bold font-mono tracking-tight text-foreground mt-0.5">{{ totalStudyHours }}</h3>
          </div>
        </CardContent>
      </Card>

      <!-- Task Completate -->
      <Card class="bg-card/30 border-border/30 backdrop-blur-sm shadow-sm">
        <CardContent class="p-4 flex items-center gap-3">
          <div class="p-2.5 rounded-lg bg-primary/10 text-primary">
            <CheckSquare class="h-5 w-5" />
          </div>
          <div>
            <p class="text-[10px] text-muted-foreground uppercase tracking-wider font-semibold">Task Completate</p>
            <h3 class="text-lg font-bold font-mono tracking-tight text-foreground mt-0.5">
              {{ completedTasksCount }} <span class="text-xs font-normal text-muted-foreground">/ {{ totalTasksCount }}</span>
            </h3>
          </div>
        </CardContent>
      </Card>

      <!-- Media Sessione -->
      <Card class="bg-card/30 border-border/30 backdrop-blur-sm shadow-sm">
        <CardContent class="p-4 flex items-center gap-3">
          <div class="p-2.5 rounded-lg bg-blue-500/10 text-blue-400">
            <Clock class="h-5 w-5" />
          </div>
          <div>
            <p class="text-[10px] text-muted-foreground uppercase tracking-wider font-semibold">Media Sessione</p>
            <h3 class="text-lg font-bold font-mono tracking-tight text-foreground mt-0.5">{{ averageSessionMins }}</h3>
          </div>
        </CardContent>
      </Card>

      <!-- Sessioni Completate -->
      <Card class="bg-card/30 border-border/30 backdrop-blur-sm shadow-sm">
        <CardContent class="p-4 flex items-center gap-3">
          <div class="p-2.5 rounded-lg bg-yellow-500/10 text-yellow-400">
            <Calendar class="h-5 w-5" />
          </div>
          <div>
            <p class="text-[10px] text-muted-foreground uppercase tracking-wider font-semibold">Sessioni Completate</p>
            <h3 class="text-lg font-bold font-mono tracking-tight text-foreground mt-0.5">
              {{ summary ? summary.study.completedSessions : 0 }} <span class="text-xs font-normal text-muted-foreground">/ {{ summary ? (summary.study.completedSessions + summary.study.interruptedSessions) : 0 }}</span>
            </h3>
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- Visual CSS Bar Charts Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6 z-10 relative">
      <!-- Study Hours Chart -->
      <Card class="bg-card/30 border-border/30 backdrop-blur-sm shadow-sm">
        <CardHeader class="p-4 pb-0">
          <CardTitle class="text-xs font-bold uppercase tracking-wider text-muted-foreground flex items-center justify-between">
            <span>Andamento Studio (Ore)</span>
            <Clock class="h-4 w-4 text-green-400" />
          </CardTitle>
        </CardHeader>
        <CardContent class="p-4">
          <div v-if="loading" class="h-48 flex items-center justify-center">
            <RefreshCw class="h-6 w-6 animate-spin text-muted-foreground" />
          </div>
          <div v-else-if="!summary || summary.study.byBucket.length === 0" class="h-48 flex flex-col items-center justify-center text-center">
            <Clock class="h-8 w-8 text-muted-foreground/30 mb-2" />
            <p class="text-xs text-muted-foreground">Nessun dato di studio in questo periodo.</p>
          </div>
          <div v-else class="h-48 flex items-end justify-between gap-1 pt-6 overflow-x-auto select-none min-w-[200px] border-b border-border/30">
            <div
              v-for="b in summary.study.byBucket"
              :key="b.bucket"
              class="flex-1 flex flex-col items-center group min-w-[20px] max-w-[50px] h-full justify-end relative"
            >
              <!-- Tooltip on Hover -->
              <div class="absolute bottom-full mb-1 opacity-0 group-hover:opacity-100 transition-opacity duration-200 pointer-events-none bg-background/95 border border-border/50 text-[10px] text-foreground font-mono font-bold px-2 py-1 rounded shadow-lg z-20 whitespace-nowrap">
                {{ formatSeconds(b.seconds) }}
              </div>
              <!-- Vertical Bar -->
              <div
                class="w-full bg-green-500/30 hover:bg-green-500/70 border border-green-500/20 hover:border-green-500/40 rounded-t-sm transition-all duration-200"
                :style="{ height: `${(b.seconds / maxStudySeconds) * 90}%` }"
              ></div>
              <!-- Label below -->
              <span class="text-[8px] font-mono text-muted-foreground mt-2 truncate w-full text-center">
                {{ formatBucketLabel(b.bucket, summary.rangeEnd ? calculateBoundaries().granularity : 'day') }}
              </span>
            </div>
          </div>
        </CardContent>
      </Card>

      <!-- Task Completed Chart -->
      <Card class="bg-card/30 border-border/30 backdrop-blur-sm shadow-sm">
        <CardHeader class="p-4 pb-0">
          <CardTitle class="text-xs font-bold uppercase tracking-wider text-muted-foreground flex items-center justify-between">
            <span>Task Completate (Quantità)</span>
            <CheckSquare class="h-4 w-4 text-primary" />
          </CardTitle>
        </CardHeader>
        <CardContent class="p-4">
          <div v-if="loading" class="h-48 flex items-center justify-center">
            <RefreshCw class="h-6 w-6 animate-spin text-muted-foreground" />
          </div>
          <div v-else-if="tasksCompletedByBucket.length === 0" class="h-48 flex flex-col items-center justify-center text-center">
            <CheckSquare class="h-8 w-8 text-muted-foreground/30 mb-2" />
            <p class="text-xs text-muted-foreground">Nessun task completato in questo periodo.</p>
          </div>
          <div v-else class="h-48 flex items-end justify-between gap-1 pt-6 overflow-x-auto select-none min-w-[200px] border-b border-border/30">
            <div
              v-for="b in tasksCompletedByBucket"
              :key="b.bucket"
              class="flex-1 flex flex-col items-center group min-w-[20px] max-w-[50px] h-full justify-end relative"
            >
              <!-- Tooltip on Hover -->
              <div class="absolute bottom-full mb-1 opacity-0 group-hover:opacity-100 transition-opacity duration-200 pointer-events-none bg-background/95 border border-border/50 text-[10px] text-foreground font-mono font-bold px-2 py-1 rounded shadow-lg z-20 whitespace-nowrap">
                {{ b.count }} {{ b.count === 1 ? 'task completato' : 'task completati' }}
              </div>
              <!-- Vertical Bar -->
              <div
                class="w-full bg-primary/20 hover:bg-primary/60 border border-primary/20 hover:border-primary/40 rounded-t-sm transition-all duration-200"
                :style="{ height: `${(b.count / maxCompletedTasks) * 90}%` }"
              ></div>
              <!-- Label below -->
              <span class="text-[8px] font-mono text-muted-foreground mt-2 truncate w-full text-center">
                {{ formatBucketLabel(b.bucket, summary?.rangeEnd ? calculateBoundaries().granularity : 'day') }}
              </span>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- Main Content Detail Row: Timeline (Left) & Controls (Right) -->
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-6 z-10 relative pb-12">
      <!-- Chronological Timeline (7 columns) -->
      <Card class="bg-card/25 border-border/20 backdrop-blur-sm shadow-sm lg:col-span-7 flex flex-col max-h-[500px]">
        <CardHeader class="p-4 shrink-0 border-b border-border/10 flex flex-row items-center justify-between">
          <div>
            <CardTitle class="text-xs font-bold uppercase tracking-wider text-muted-foreground">Registro Log Dettagliato</CardTitle>
            <CardDescription class="text-[10px] text-muted-foreground mt-0.5">Mostra gli ultimi 50 eventi registrati in questo periodo.</CardDescription>
          </div>
          <Button variant="ghost" size="icon" class="h-7 w-7 text-muted-foreground hover:text-foreground cursor-pointer" @click="loadData" :disabled="loading">
            <RefreshCw class="h-3.5 w-3.5" :class="{ 'animate-spin': loading }" />
          </Button>
        </CardHeader>
        <CardContent class="p-0 overflow-y-auto flex-1">
          <div v-if="loading && recentEvents.length === 0" class="p-8 flex items-center justify-center">
            <RefreshCw class="h-6 w-6 animate-spin text-muted-foreground" />
          </div>
          <div v-else-if="recentEvents.length === 0" class="p-8 text-center text-xs text-muted-foreground">
            Nessun evento registrato in questo intervallo temporale.
          </div>
          <div v-else class="divide-y divide-border/10">
            <div
              v-for="event in recentEvents"
              :key="event.id"
              class="p-3.5 flex items-center justify-between hover:bg-muted/10 transition-colors"
            >
              <div class="flex items-center gap-3">
                <!-- Icon type indicator -->
                <div class="px-2.5 py-1 rounded border text-[9px] font-bold uppercase tracking-wider" :class="getEventBadgeClass(event.eventType)">
                  {{ getEventLabel(event.eventType) }}
                </div>
                <div>
                  <div class="text-xs font-semibold text-foreground flex items-center gap-1.5">
                    <span class="font-mono">
                      {{ formatSeconds(event.durationSeconds || 0) }}
                    </span>
                    <span v-if="event.note" class="text-[10px] font-normal text-muted-foreground">
                      - {{ event.note }}
                    </span>
                  </div>
                  <div class="text-[9px] text-muted-foreground font-mono mt-0.5">
                    {{ new Date(event.startedAt).toLocaleString('it-IT', { hour: '2-digit', minute: '2-digit', day: '2-digit', month: '2-digit' }) }}
                    <span v-if="event.syncStatus === 'synced'" class="text-green-500 ml-1.5 inline-flex items-center gap-0.5">
                      <Cloud class="h-2.5 w-2.5" /> Cloud
                    </span>
                    <span v-else class="text-yellow-500/80 ml-1.5 inline-flex items-center gap-0.5">
                      <Cloud class="h-2.5 w-2.5" /> Locale
                    </span>
                  </div>
                </div>
              </div>
              <Button
                variant="ghost"
                size="icon"
                class="h-7 w-7 text-muted-foreground hover:text-primary hover:bg-primary/5 cursor-pointer rounded-lg shrink-0"
                title="Elimina log permanentemente"
                @click="handleDeleteEvent(event.id)"
              >
                <Trash2 class="h-3.5 w-3.5" />
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>

      <!-- Settings Tools (5 columns) -->
      <div class="lg:col-span-5 space-y-6">
        <!-- Stats details card -->
        <Card class="bg-card/25 border-border/20 backdrop-blur-sm shadow-sm">
          <CardHeader class="p-4 pb-2">
            <CardTitle class="text-xs font-bold uppercase tracking-wider text-muted-foreground flex items-center justify-between">
              <span>Dettaglio Attività Periodo</span>
              <TrendingUp class="h-4 w-4 text-primary" />
            </CardTitle>
          </CardHeader>
          <CardContent class="p-4 space-y-4">
            <div class="space-y-3.5">
              <!-- Studio stats -->
              <div class="flex justify-between items-center py-2 border-b border-border/10">
                <span class="text-xs text-muted-foreground">Tempo di studio totale</span>
                <span class="text-xs font-bold text-foreground font-mono">{{ totalStudyHours }}</span>
              </div>
              <div class="flex justify-between items-center py-2 border-b border-border/10">
                <span class="text-xs text-muted-foreground">Sessioni completate</span>
                <span class="text-xs font-bold text-foreground font-mono">
                  {{ summary ? summary.study.completedSessions : 0 }} / {{ summary ? (summary.study.completedSessions + summary.study.interruptedSessions) : 0 }}
                </span>
              </div>

              <!-- Task stats -->
              <div class="flex justify-between items-center py-2 border-b border-border/10">
                <span class="text-xs text-muted-foreground">Task aggiunte nel periodo</span>
                <span class="text-xs font-bold text-foreground font-mono">{{ tasksAddedInPeriod.length }}</span>
              </div>
              <div class="flex justify-between items-center py-2 border-b border-border/10">
                <span class="text-xs text-muted-foreground">Task completate nel periodo</span>
                <span class="text-xs font-bold text-foreground font-mono">{{ tasksCompletedInPeriod.length }}</span>
              </div>
              <div class="flex justify-between items-center py-2">
                <span class="text-xs text-muted-foreground">Tasso di completamento generale</span>
                <span class="text-xs font-bold text-primary font-mono">{{ taskCompletionRate }}%</span>
              </div>
            </div>
            
            <div class="text-[10px] text-muted-foreground leading-relaxed bg-muted/20 p-2.5 rounded-lg border border-border/30 mt-2">
              Nota: Le metriche si basano sull'intervallo temporale selezionato. Modificando l'intervallo in alto, i dati del riepilogo verranno ricalcolati automaticamente.
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Scrollbar custom style inside recent events card */
::-webkit-scrollbar {
  width: 4px;
}
::-webkit-scrollbar-track {
  background: transparent;
}
::-webkit-scrollbar-thumb {
  background: rgba(239, 68, 68, 0.15);
  border-radius: 99px;
}
::-webkit-scrollbar-thumb:hover {
  background: rgba(239, 68, 68, 0.3);
}
</style>
