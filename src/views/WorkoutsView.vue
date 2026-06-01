<script lang="ts" setup>
import { ref, computed, onMounted } from 'vue'
import { useWorkoutStore } from '@/stores/workout.store'
import { storeToRefs } from 'pinia'
import { Plus, Dumbbell, Clock, Flame, ChevronRight, Trash2, X } from 'lucide-vue-next'
import type { WorkoutTemplate } from '@/types/workout'

const workoutStore = useWorkoutStore()
const { templates, logsThisWeek, totalMinutesThisWeek, totalCaloriesThisWeek } = storeToRefs(workoutStore)

onMounted(() => workoutStore.init())

// ── Add Template Dialog ───────────────────────────────────────
const showTemplateDialog = ref(false)
const templateForm = ref({ name: '', description: '', category: 'gym' })
const categoryOptions = ['gym', 'swimming', 'running', 'cycling', 'home', 'yoga', 'other']

async function submitTemplate() {
  if (!templateForm.value.name.trim()) return
  await workoutStore.createTemplate({
    name: templateForm.value.name,
    description: templateForm.value.description || null,
    category: templateForm.value.category || null,
    exercises_json: '[]',
  })
  templateForm.value = { name: '', description: '', category: 'gym' }
  showTemplateDialog.value = false
}

// ── Log Workout Dialog ────────────────────────────────────────
const showLogDialog = ref(false)
const selectedTemplate = ref<WorkoutTemplate | null>(null)
const logForm = ref({
  title: '',
  duration_minutes: null as number | null,
  calories: null as number | null,
  notes: '',
})

function openLog(tmpl?: WorkoutTemplate) {
  selectedTemplate.value = tmpl ?? null
  logForm.value = {
    title: tmpl?.name ?? '',
    duration_minutes: null,
    calories: null,
    notes: '',
  }
  showLogDialog.value = true
}

async function submitLog() {
  if (!logForm.value.title.trim()) return
  await workoutStore.createLog({
    template_id: selectedTemplate.value?.id ?? null,
    title: logForm.value.title,
    performed_at: new Date().toISOString(),
    duration_minutes: logForm.value.duration_minutes,
    calories: logForm.value.calories,
    exercises_json: selectedTemplate.value?.exercises_json ?? '[]',
    notes: logForm.value.notes || null,
  })
  showLogDialog.value = false
}

// ── Helpers ───────────────────────────────────────────────────
function formatDuration(mins: number | null): string {
  if (!mins) return '—'
  const h = Math.floor(mins / 60)
  const m = mins % 60
  return h > 0 ? `${h}h ${m}m` : `${m}m`
}

function formatDate(iso: string): string {
  const d = new Date(iso)
  return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })
}

function categoryIcon(cat: string | null): string {
  const icons: Record<string, string> = {
    gym: '🏋️', swimming: '🏊', running: '🏃', cycling: '🚴',
    home: '🏠', yoga: '🧘', other: '💪'
  }
  return icons[cat ?? 'other'] ?? '💪'
}

const recentLogs = computed(() => workoutStore.logs.slice(0, 15))
</script>

<template>
  <div class="h-full w-full overflow-y-auto p-6 space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-xl font-bold text-foreground tracking-tight">Workouts</h1>
        <p class="text-xs text-muted-foreground mt-0.5">Log and track your training sessions</p>
      </div>
      <div class="flex gap-2">
        <button
          @click="showTemplateDialog = true"
          class="flex items-center gap-2 px-3 py-1.5 border border-border/40 hover:bg-muted text-muted-foreground hover:text-foreground rounded-lg text-xs font-semibold transition-all cursor-pointer"
        >
          <Plus class="h-3.5 w-3.5" />
          Template
        </button>
        <button
          @click="openLog()"
          class="flex items-center gap-2 px-3 py-1.5 bg-primary/10 hover:bg-primary/20 text-primary border border-primary/20 rounded-lg text-xs font-semibold transition-all cursor-pointer"
        >
          <Dumbbell class="h-3.5 w-3.5" />
          Log Workout
        </button>
      </div>
    </div>

    <!-- Hero Stats Row -->
    <div class="grid grid-cols-3 gap-3">
      <div class="rounded-xl border border-border/40 bg-muted/10 p-4">
        <div class="text-xs text-muted-foreground mb-1">This Week</div>
        <div class="text-2xl font-bold text-foreground">{{ logsThisWeek.length }}</div>
        <div class="text-xs text-muted-foreground">workouts</div>
      </div>
      <div class="rounded-xl border border-border/40 bg-muted/10 p-4">
        <div class="text-xs text-muted-foreground mb-1">Total Time</div>
        <div class="text-2xl font-bold text-foreground">{{ formatDuration(totalMinutesThisWeek) }}</div>
        <div class="text-xs text-muted-foreground">this week</div>
      </div>
      <div class="rounded-xl border border-border/40 bg-muted/10 p-4">
        <div class="text-xs text-muted-foreground mb-1">Calories</div>
        <div class="text-2xl font-bold text-foreground">{{ totalCaloriesThisWeek > 0 ? totalCaloriesThisWeek + ' kcal' : '—' }}</div>
        <div class="text-xs text-muted-foreground">this week</div>
      </div>
    </div>

    <!-- Main Grid -->
    <div class="grid grid-cols-5 gap-4">
      <!-- Templates Panel (2 cols) -->
      <div class="col-span-2 space-y-3">
        <div class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Templates</div>
        <div v-if="templates.length === 0" class="text-center py-8 text-muted-foreground text-sm">
          No templates yet
        </div>
        <div
          v-for="tmpl in templates"
          :key="tmpl.id"
          class="rounded-xl border border-border/40 bg-muted/10 p-3 hover:border-border/60 transition-colors group"
        >
          <div class="flex items-center gap-2">
            <span class="text-lg">{{ categoryIcon(tmpl.category) }}</span>
            <div class="flex-1 min-w-0">
              <div class="text-sm font-semibold text-foreground truncate">{{ tmpl.name }}</div>
              <div v-if="tmpl.category" class="text-xs text-muted-foreground capitalize">{{ tmpl.category }}</div>
            </div>
            <div class="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
              <button
                @click="openLog(tmpl)"
                class="h-7 px-2 flex items-center gap-1 rounded-lg bg-primary/10 text-primary text-xs font-semibold cursor-pointer hover:bg-primary/20 transition-colors"
              >
                <ChevronRight class="h-3 w-3" />
                Log
              </button>
              <button
                @click="workoutStore.deleteTemplate(tmpl.id)"
                class="h-7 w-7 flex items-center justify-center rounded-lg text-muted-foreground hover:text-red-400 hover:bg-red-500/10 cursor-pointer transition-all"
              >
                <Trash2 class="h-3 w-3" />
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Recent Logs (3 cols) -->
      <div class="col-span-3 space-y-3">
        <div class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Recent Workouts</div>
        <div v-if="recentLogs.length === 0" class="text-center py-8 text-muted-foreground text-sm">
          No workouts logged yet
        </div>
        <div
          v-for="log in recentLogs"
          :key="log.id"
          class="rounded-xl border border-border/40 bg-muted/10 p-3 hover:border-border/60 transition-colors group"
        >
          <div class="flex items-start justify-between gap-2">
            <div class="flex-1 min-w-0">
              <div class="text-sm font-semibold text-foreground truncate">{{ log.title }}</div>
              <div class="flex items-center gap-3 mt-1">
                <span class="text-xs text-muted-foreground">{{ formatDate(log.performed_at) }}</span>
                <span v-if="log.duration_minutes" class="flex items-center gap-1 text-xs text-muted-foreground">
                  <Clock class="h-3 w-3" />
                  {{ formatDuration(log.duration_minutes) }}
                </span>
                <span v-if="log.calories" class="flex items-center gap-1 text-xs text-muted-foreground">
                  <Flame class="h-3 w-3" />
                  {{ log.calories }} kcal
                </span>
              </div>
              <div v-if="log.notes" class="text-xs text-muted-foreground/70 mt-1 truncate">{{ log.notes }}</div>
            </div>
            <button
              @click="workoutStore.deleteLog(log.id)"
              class="h-7 w-7 flex items-center justify-center rounded-lg text-muted-foreground hover:text-red-400 hover:bg-red-500/10 opacity-0 group-hover:opacity-100 cursor-pointer transition-all"
            >
              <Trash2 class="h-3 w-3" />
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Add Template Dialog -->
    <Teleport to="body">
      <div v-if="showTemplateDialog" class="fixed inset-0 z-50 flex items-center justify-center">
        <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="showTemplateDialog = false"></div>
        <div class="relative w-full max-w-sm bg-background border border-border/50 rounded-2xl shadow-2xl p-6 space-y-4 z-10">
          <div class="flex items-center justify-between">
            <h2 class="text-base font-bold">New Template</h2>
            <button @click="showTemplateDialog = false" class="h-7 w-7 flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted cursor-pointer"><X class="h-4 w-4" /></button>
          </div>
          <div class="space-y-3">
            <input v-model="templateForm.name" placeholder="Template name *" class="w-full px-3 py-2 bg-muted/30 border border-border/40 rounded-lg text-sm text-foreground placeholder:text-muted-foreground/50 focus:outline-none focus:border-primary/50 transition-colors" />
            <input v-model="templateForm.description" placeholder="Description (optional)" class="w-full px-3 py-2 bg-muted/30 border border-border/40 rounded-lg text-sm text-foreground placeholder:text-muted-foreground/50 focus:outline-none focus:border-primary/50 transition-colors" />
            <select v-model="templateForm.category" class="w-full px-3 py-2 bg-muted/30 border border-border/40 rounded-lg text-sm text-foreground focus:outline-none focus:border-primary/50 cursor-pointer">
              <option v-for="c in categoryOptions" :key="c" :value="c" class="capitalize">{{ c }}</option>
            </select>
          </div>
          <div class="flex gap-2">
            <button @click="showTemplateDialog = false" class="flex-1 py-2 rounded-lg border border-border/40 text-sm font-semibold text-muted-foreground hover:bg-muted cursor-pointer">Cancel</button>
            <button @click="submitTemplate" :disabled="!templateForm.name.trim()" class="flex-1 py-2 rounded-lg bg-primary text-white text-sm font-semibold cursor-pointer hover:bg-primary/90 disabled:opacity-50">Create</button>
          </div>
        </div>
      </div>
    </Teleport>

    <!-- Log Workout Dialog -->
    <Teleport to="body">
      <div v-if="showLogDialog" class="fixed inset-0 z-50 flex items-center justify-center">
        <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="showLogDialog = false"></div>
        <div class="relative w-full max-w-sm bg-background border border-border/50 rounded-2xl shadow-2xl p-6 space-y-4 z-10">
          <div class="flex items-center justify-between">
            <h2 class="text-base font-bold">Log Workout</h2>
            <button @click="showLogDialog = false" class="h-7 w-7 flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted cursor-pointer"><X class="h-4 w-4" /></button>
          </div>
          <div v-if="selectedTemplate" class="text-xs text-muted-foreground px-2 py-1 bg-muted/30 rounded-lg">
            Template: <span class="text-foreground font-medium">{{ selectedTemplate.name }}</span>
          </div>
          <div class="space-y-3">
            <input v-model="logForm.title" placeholder="Workout title *" class="w-full px-3 py-2 bg-muted/30 border border-border/40 rounded-lg text-sm text-foreground placeholder:text-muted-foreground/50 focus:outline-none focus:border-primary/50 transition-colors" />
            <div class="flex gap-2">
              <div class="flex-1">
                <label class="text-xs text-muted-foreground block mb-1">Duration (min)</label>
                <input v-model.number="logForm.duration_minutes" type="number" min="0" placeholder="e.g. 60" class="w-full px-3 py-2 bg-muted/30 border border-border/40 rounded-lg text-sm text-foreground placeholder:text-muted-foreground/50 focus:outline-none focus:border-primary/50 transition-colors" />
              </div>
              <div class="flex-1">
                <label class="text-xs text-muted-foreground block mb-1">Calories</label>
                <input v-model.number="logForm.calories" type="number" min="0" placeholder="e.g. 400" class="w-full px-3 py-2 bg-muted/30 border border-border/40 rounded-lg text-sm text-foreground placeholder:text-muted-foreground/50 focus:outline-none focus:border-primary/50 transition-colors" />
              </div>
            </div>
            <input v-model="logForm.notes" placeholder="Notes (optional)" class="w-full px-3 py-2 bg-muted/30 border border-border/40 rounded-lg text-sm text-foreground placeholder:text-muted-foreground/50 focus:outline-none focus:border-primary/50 transition-colors" />
          </div>
          <div class="flex gap-2">
            <button @click="showLogDialog = false" class="flex-1 py-2 rounded-lg border border-border/40 text-sm font-semibold text-muted-foreground hover:bg-muted cursor-pointer">Cancel</button>
            <button @click="submitLog" :disabled="!logForm.title.trim()" class="flex-1 py-2 rounded-lg bg-primary text-white text-sm font-semibold cursor-pointer hover:bg-primary/90 disabled:opacity-50">Log Workout</button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>
