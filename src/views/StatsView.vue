<script lang="ts" setup>
import { ref, computed, onMounted, watch } from 'vue'
import { storeToRefs } from 'pinia'
import {
  Clock,
  Cigarette,
  Calendar,
  ArrowDownToLine,
  RefreshCw,
  Cloud,
  Lock,
  Trash2,
  TrendingUp,
  LogOut,
  Info
} from 'lucide-vue-next'

import { api } from '@/lib/tauri'
import { useSyncStore } from '@/stores/sync.store'
import { useSmokingStore } from '@/stores/smoking.store'
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

const syncStore = useSyncStore()
const smokingStore = useSmokingStore()
const { isAuthenticated, syncing, userEmail, isConfigured } = storeToRefs(syncStore)

const rangeType = ref<'today' | 'week' | 'month' | 'year' | 'custom'>('week')
const customStartDate = ref('')
const customEndDate = ref('')

const summary = ref<TrackingSummary | null>(null)
const recentEvents = ref<TrackingEvent[]>([])
const loading = ref(false)

// Supabase Auth Form
const authEmail = ref('')
const authPassword = ref('')
const authLoading = ref(false)
const authMode = ref<'login' | 'signup'>('login')

// Export Form
const exportFormat = ref<'json' | 'csv' | 'markdown'>('csv')
const exportStudy = ref(true)
const exportSmoking = ref(true)

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
    
    // Fetch Recent Events
    recentEvents.value = await api.tracking.listEvents(undefined, start, end, 50)
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
  await syncStore.init()
  await loadData()
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

const totalCigarettes = computed(() => {
  return summary.value ? summary.value.smoking.totalCigarettes : 0
})

// Max values for chart scaling
const maxStudySeconds = computed(() => {
  if (!summary.value || summary.value.study.byBucket.length === 0) return 1
  return Math.max(...summary.value.study.byBucket.map(b => b.seconds), 1)
})

const maxCigarettes = computed(() => {
  if (!summary.value || summary.value.smoking.byBucket.length === 0) return 1
  return Math.max(...summary.value.smoking.byBucket.map(b => b.count), 1)
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
  if (confirm('Sei sicuro di voler eliminare questo evento permanentemente?')) {
    try {
      await api.tracking.deleteEvent(id)
      toast.success('Log eliminato correttamente.')
      await loadData()
      await smokingStore.loadSmokingTodayCount() // Update fumo dropdown count as well
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
    case 'cigarette_smoked':
      return 'bg-red-500/10 border-red-500/20 text-primary'
    case 'timer_interrupted':
      return 'bg-yellow-500/10 border-yellow-500/20 text-yellow-400'
    default:
      return 'bg-muted border-border text-muted-foreground'
  }
}

function getEventLabel(type: string) {
  switch (type) {
    case 'study_session': return 'Studio'
    case 'cigarette_smoked': return 'Fumo'
    case 'timer_interrupted': return 'Interrotto'
    default: return type
  }
}

// Sync execution
async function handleSync() {
  toast.promise(syncStore.sync(), {
    loading: 'Sincronizzazione in corso...',
    success: (res: boolean) => {
      if (res) {
        loadData()
        smokingStore.loadSmokingTodayCount()
        return 'Sincronizzazione completata!'
      } else {
        throw new Error()
      }
    },
    error: () => syncStore.error || 'Sincronizzazione fallita.'
  })
}

// Auth submission
async function handleAuth() {
  authLoading.value = true
  try {
    if (authMode.value === 'login') {
      await syncStore.login(authEmail.value, authPassword.value)
      toast.success('Autenticazione riuscita!')
    } else {
      await syncStore.signup(authEmail.value, authPassword.value)
      toast.success('Registrazione completata! Controlla la tua email.')
    }
    authEmail.value = ''
    authPassword.value = ''
  } catch (e: any) {
    toast.error(e.message || 'Errore di autenticazione.')
  } finally {
    authLoading.value = false
  }
}

// Export Trigger
async function handleExport() {
  const { start, end } = calculateBoundaries()
  const types: string[] = []
  if (exportStudy.value) types.push('study_session')
  if (exportSmoking.value) types.push('cigarette_smoked')

  if (types.length === 0) {
    toast.error('Seleziona almeno un tipo di dato da esportare.')
    return
  }

  try {
    const rawData = await api.export.exportData(exportFormat.value, start, end, types)
    if (!rawData) {
      toast.info('Nessun dato trovato per l\'esportazione in questo periodo.')
      return
    }

    const blob = new Blob([rawData], { type: 'text/plain;charset=utf-8' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    
    let ext = 'csv'
    if (exportFormat.value === 'json') ext = 'json'
    else if (exportFormat.value === 'markdown') ext = 'md'

    a.download = `studytimer_export_${rangeType.value}_${new Date().toISOString().slice(0, 10)}.${ext}`
    a.click()
    URL.revokeObjectURL(url)
    
    toast.success('Esportazione completata con successo!')
  } catch (e) {
    console.error('Esportazione fallita:', e)
    toast.error('Errore durante l\'esportazione.')
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
        <p class="text-xs text-muted-foreground">Analizza le sessioni di studio, abitudini ed esporta i log locali.</p>
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

      <Card class="bg-card/30 border-border/30 backdrop-blur-sm shadow-sm">
        <CardContent class="p-4 flex items-center gap-3">
          <div class="p-2.5 rounded-lg bg-primary/10 text-primary">
            <Cigarette class="h-5 w-5" />
          </div>
          <div>
            <p class="text-[10px] text-muted-foreground uppercase tracking-wider font-semibold">Sigarette</p>
            <h3 class="text-lg font-bold font-mono tracking-tight text-foreground mt-0.5">{{ totalCigarettes }}</h3>
          </div>
        </CardContent>
      </Card>

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

      <!-- Smoking Chart -->
      <Card class="bg-card/30 border-border/30 backdrop-blur-sm shadow-sm">
        <CardHeader class="p-4 pb-0">
          <CardTitle class="text-xs font-bold uppercase tracking-wider text-muted-foreground flex items-center justify-between">
            <span>Consumo Sigarette</span>
            <Cigarette class="h-4 w-4 text-primary" />
          </CardTitle>
        </CardHeader>
        <CardContent class="p-4">
          <div v-if="loading" class="h-48 flex items-center justify-center">
            <RefreshCw class="h-6 w-6 animate-spin text-muted-foreground" />
          </div>
          <div v-else-if="!summary || summary.smoking.byBucket.length === 0" class="h-48 flex flex-col items-center justify-center text-center">
            <Cigarette class="h-8 w-8 text-muted-foreground/30 mb-2" />
            <p class="text-xs text-muted-foreground">Nessun dato di fumo in questo periodo.</p>
          </div>
          <div v-else class="h-48 flex items-end justify-between gap-1 pt-6 overflow-x-auto select-none min-w-[200px] border-b border-border/30">
            <div
              v-for="b in summary.smoking.byBucket"
              :key="b.bucket"
              class="flex-1 flex flex-col items-center group min-w-[20px] max-w-[50px] h-full justify-end relative"
            >
              <!-- Tooltip on Hover -->
              <div class="absolute bottom-full mb-1 opacity-0 group-hover:opacity-100 transition-opacity duration-200 pointer-events-none bg-background/95 border border-border/50 text-[10px] text-foreground font-mono font-bold px-2 py-1 rounded shadow-lg z-20 whitespace-nowrap">
                {{ b.count }} {{ b.count === 1 ? 'sigaretta' : 'sigarette' }}
              </div>
              <!-- Vertical Bar -->
              <div
                class="w-full bg-primary/20 hover:bg-primary/60 border border-primary/20 hover:border-primary/40 rounded-t-sm transition-all duration-200"
                :style="{ height: `${(b.count / maxCigarettes) * 90}%` }"
              ></div>
              <!-- Label below -->
              <span class="text-[8px] font-mono text-muted-foreground mt-2 truncate w-full text-center">
                {{ formatBucketLabel(b.bucket, summary.rangeEnd ? calculateBoundaries().granularity : 'day') }}
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
                    <span v-if="event.eventType === 'study_session'" class="font-mono">
                      {{ formatSeconds(event.durationSeconds || 0) }}
                    </span>
                    <span v-else-if="event.eventType === 'cigarette_smoked'">
                      1 sigaretta
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

      <!-- Settings & Cloud Sync Tools (5 columns) -->
      <div class="lg:col-span-5 space-y-6">
        <!-- Supabase Cloud Sync Card -->
        <Card class="bg-card/25 border-border/20 backdrop-blur-sm shadow-sm">
          <CardHeader class="p-4 pb-2">
            <CardTitle class="text-xs font-bold uppercase tracking-wider text-muted-foreground flex items-center justify-between">
              <span>Sincronizzazione Supabase</span>
              <Cloud class="h-4 w-4" :class="isAuthenticated ? 'text-green-400' : 'text-muted-foreground'" />
            </CardTitle>
          </CardHeader>
          <CardContent class="p-4 space-y-4">
            <!-- Case 1: Supabase not configured in .env -->
            <div v-if="!isConfigured" class="flex gap-2.5 p-3 rounded-lg border border-yellow-500/20 bg-yellow-500/5 text-yellow-400 text-xs">
              <Info class="h-4 w-4 shrink-0 mt-0.5" />
              <div class="space-y-1 leading-normal">
                <p class="font-bold uppercase text-[9px] tracking-wider">Missing Config</p>
                <p class="text-[10px] text-yellow-400/80">Configura `VITE_SUPABASE_URL` e `VITE_SUPABASE_ANON_KEY` nel file `.env` per attivare la sincronizzazione cloud.</p>
              </div>
            </div>

            <!-- Case 2: Configured but NOT logged in -->
            <div v-else-if="!isAuthenticated" class="space-y-3">
              <div class="text-[10px] text-muted-foreground bg-muted/20 p-2.5 rounded-lg border border-border/30">
                Sincronizza i dati di studio e tracciamento tra più dispositivi. I dati locali SQLite rimangono primari ed attivi offline.
              </div>

              <!-- Auth Mode Selector -->
              <div class="flex bg-muted/40 p-0.5 rounded-lg border border-border/30 gap-px">
                <button
                  class="flex-1 h-7 text-[10px] font-bold uppercase tracking-wider rounded-md cursor-pointer transition-all"
                  :class="authMode === 'login' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'"
                  @click="authMode = 'login'"
                >
                  Accedi
                </button>
                <button
                  class="flex-1 h-7 text-[10px] font-bold uppercase tracking-wider rounded-md cursor-pointer transition-all"
                  :class="authMode === 'signup' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'"
                  @click="authMode = 'signup'"
                >
                  Registrati
                </button>
              </div>

              <!-- Login Form -->
              <form @submit.prevent="handleAuth" class="space-y-3.5">
                <div class="space-y-1.5">
                  <Label for="auth-email" class="text-[10px] text-muted-foreground uppercase tracking-wider">Email</Label>
                  <Input id="auth-email" type="email" placeholder="nome@esempio.com" v-model="authEmail" class="h-8.5 text-xs bg-background/50 border-border/40 focus:border-primary/50 text-foreground" required />
                </div>
                <div class="space-y-1.5">
                  <Label for="auth-pwd" class="text-[10px] text-muted-foreground uppercase tracking-wider">Password</Label>
                  <Input id="auth-pwd" type="password" placeholder="••••••••" v-model="authPassword" class="h-8.5 text-xs bg-background/50 border-border/40 focus:border-primary/50 text-foreground" required />
                </div>
                <Button type="submit" class="w-full h-9 bg-primary hover:bg-primary/95 text-white text-xs font-semibold cursor-pointer" :disabled="authLoading">
                  <RefreshCw v-if="authLoading" class="mr-2 h-3.5 w-3.5 animate-spin" />
                  <Lock v-else class="mr-2 h-3.5 w-3.5" />
                  {{ authMode === 'login' ? 'Accedi al Cloud' : 'Crea Nuovo Account' }}
                </Button>
              </form>
            </div>

            <!-- Case 3: Configured and LOGGED in -->
            <div v-else class="space-y-4">
              <div class="flex items-center justify-between p-3 rounded-lg border border-border/30 bg-muted/20">
                <div class="min-w-0">
                  <p class="text-[10px] text-muted-foreground uppercase tracking-wider font-semibold">Account</p>
                  <p class="text-xs font-semibold truncate text-foreground mt-0.5 font-mono">{{ userEmail }}</p>
                </div>
                <Button variant="ghost" size="icon" class="h-8 w-8 text-muted-foreground hover:text-primary hover:bg-primary/5 cursor-pointer rounded-lg" title="Scollega account" @click="syncStore.logout()">
                  <LogOut class="h-4 w-4" />
                </Button>
              </div>

              <!-- Sync actions -->
              <div class="space-y-2">
                <div class="flex items-center justify-between text-[10px] text-muted-foreground px-1 font-mono">
                  <span>Ultimo sync:</span>
                  <span>{{ syncStore.lastSyncedAt ? new Date(syncStore.lastSyncedAt).toLocaleString('it-IT') : 'Mai' }}</span>
                </div>
                <Button
                  class="w-full h-9 bg-primary hover:bg-primary/95 text-white font-semibold text-xs cursor-pointer shadow-[0_0_10px_rgba(239,68,68,0.15)]"
                  :disabled="syncing"
                  @click="handleSync"
                >
                  <RefreshCw class="mr-2 h-3.5 w-3.5" :class="{ 'animate-spin': syncing }" />
                  Sincronizza Ora
                </Button>
              </div>
            </div>
          </CardContent>
        </Card>

        <!-- Data Exporter Card -->
        <Card class="bg-card/25 border-border/20 backdrop-blur-sm shadow-sm">
          <CardHeader class="p-4 pb-2">
            <CardTitle class="text-xs font-bold uppercase tracking-wider text-muted-foreground flex items-center justify-between">
              <span>Esporta Dati Locali</span>
              <ArrowDownToLine class="h-4 w-4 text-muted-foreground" />
            </CardTitle>
          </CardHeader>
          <CardContent class="p-4 space-y-4">
            <div class="space-y-3">
              <!-- Format Selection -->
              <div class="space-y-1.5">
                <Label class="text-[10px] text-muted-foreground uppercase tracking-wider">Formato File</Label>
                <div class="flex bg-muted/40 p-0.5 rounded-lg border border-border/30 gap-px">
                  <button
                    v-for="fmt in ['csv', 'json', 'markdown']"
                    :key="fmt"
                    class="flex-1 h-7 text-[10px] font-bold uppercase tracking-wider rounded-md cursor-pointer transition-all"
                    :class="exportFormat === fmt ? 'bg-background text-foreground shadow-sm border border-border/30' : 'text-muted-foreground hover:text-foreground'"
                    @click="exportFormat = fmt as any"
                  >
                    {{ fmt === 'markdown' ? 'Obsidian' : fmt }}
                  </button>
                </div>
              </div>

              <!-- Data Types Selection -->
              <div class="space-y-2.5 pt-1">
                <Label class="text-[10px] text-muted-foreground uppercase tracking-wider">Dati da Includere</Label>
                
                <div class="flex flex-col gap-2 bg-muted/10 p-2.5 rounded-lg border border-border/30">
                  <label class="flex items-center gap-2.5 text-xs text-foreground cursor-pointer select-none">
                    <input
                      type="checkbox"
                      v-model="exportStudy"
                      class="h-4 w-4 rounded border-border bg-background text-primary focus:ring-primary/50 cursor-pointer accent-primary"
                    />
                    <span>Sessioni di Studio (study_session)</span>
                  </label>
                  
                  <label class="flex items-center gap-2.5 text-xs text-foreground cursor-pointer select-none">
                    <input
                      type="checkbox"
                      v-model="exportSmoking"
                      class="h-4 w-4 rounded border-border bg-background text-primary focus:ring-primary/50 cursor-pointer accent-primary"
                    />
                    <span>Registro Fumo (cigarette_smoked)</span>
                  </label>
                </div>
              </div>

              <!-- Export Execute Button -->
              <Button
                variant="outline"
                class="w-full h-9 border-border hover:bg-muted/80 hover:text-foreground text-xs font-semibold cursor-pointer mt-1"
                @click="handleExport"
              >
                <ArrowDownToLine class="mr-2 h-3.5 w-3.5" />
                Scarica File Esportazione
              </Button>
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
