<script lang="ts" setup>
import { ref, onMounted } from 'vue'
import { useHabitStore } from '@/stores/habit.store'
import { storeToRefs } from 'pinia'
import { Plus, Flame, Target, Minus, Check, X, Edit2, Archive, Trash2 } from 'lucide-vue-next'
import type { Habit, CreateHabitPayload, HabitTrackingType } from '@/types/habit'

const habitStore = useHabitStore()
const { activeHabits, loading } = storeToRefs(habitStore)

onMounted(() => habitStore.init())

// ── Add Habit Dialog ──────────────────────────────────────────
const showAddDialog = ref(false)
const editingHabit = ref<Habit | null>(null)
const form = ref<CreateHabitPayload>({
  name: '',
  description: null,
  icon: null,
  color: 'red',
  tracking_type: 'boolean',
  unit: null,
  daily_goal: null,
  direction: 'build',
})

const colorOptions = [
  { label: 'Red', value: 'red', cls: 'bg-red-500' },
  { label: 'Blue', value: 'blue', cls: 'bg-blue-500' },
  { label: 'Green', value: 'green', cls: 'bg-green-500' },
  { label: 'Purple', value: 'purple', cls: 'bg-purple-500' },
  { label: 'Orange', value: 'orange', cls: 'bg-orange-500' },
  { label: 'Teal', value: 'teal', cls: 'bg-teal-500' },
]

const typeOptions: { label: string; value: HabitTrackingType }[] = [
  { label: '✓ Boolean (done/not done)', value: 'boolean' },
  { label: '# Counter (count things)', value: 'counter' },
  { label: '⏱ Duration (time)', value: 'duration' },
  { label: '📏 Quantity (pages, km...)', value: 'quantity' },
  { label: '⭐ Rating (1-5)', value: 'rating' },
]

function openAdd() {
  editingHabit.value = null
  form.value = { name: '', description: null, icon: null, color: 'red', tracking_type: 'boolean', unit: null, daily_goal: null, direction: 'build' }
  showAddDialog.value = true
}

function openEdit(habit: Habit) {
  editingHabit.value = habit
  form.value = {
    name: habit.name,
    description: habit.description,
    icon: habit.icon,
    color: habit.color,
    tracking_type: habit.tracking_type as HabitTrackingType,
    unit: habit.unit,
    daily_goal: habit.daily_goal,
    direction: habit.direction as any,
  }
  showAddDialog.value = true
}

async function submitHabit() {
  if (!form.value.name.trim()) return
  if (editingHabit.value) {
    await habitStore.updateHabit({ id: editingHabit.value.id, ...form.value })
  } else {
    await habitStore.createHabit(form.value)
  }
  showAddDialog.value = false
}

async function archiveHabit(id: string) {
  await habitStore.updateHabit({ id, is_archived: true })
}

async function deleteHabit(id: string) {
  if (confirm('Delete this habit? All entries will be lost.')) {
    await habitStore.deleteHabit(id)
  }
}

// ── Grid (contribution-style) ─────────────────────────────────
const GRID_DAYS = 84  // 12 weeks

function getGridDates(): string[] {
  const dates: string[] = []
  const today = new Date()
  for (let i = GRID_DAYS - 1; i >= 0; i--) {
    const d = new Date(today)
    d.setDate(today.getDate() - i)
    dates.push(`${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`)
  }
  return dates
}

const gridDates = getGridDates()

function cellIntensity(habit: Habit, date: string): number {
  const entry = habitStore.getEntryForDate(habit.id, date)
  if (!entry) return 0
  const v = entry.value
  if (habit.tracking_type === 'boolean') return v > 0 ? 4 : 0
  if (!habit.daily_goal) return Math.min(4, Math.ceil(v))
  const ratio = v / habit.daily_goal
  if (habit.direction === 'limit') {
    // For limit habits: 0 cigs = best (4), goal+ = worst (0)
    if (v === 0) return 4
    if (ratio <= 0.25) return 3
    if (ratio <= 0.5) return 2
    if (ratio <= 1) return 1
    return 0
  }
  // build habit
  if (ratio >= 1) return 4
  if (ratio >= 0.75) return 3
  if (ratio >= 0.5) return 2
  if (ratio >= 0.25) return 1
  return 0
}

function cellColorClass(habit: Habit, intensity: number): string {
  if (intensity === 0) return 'bg-muted/40 border border-border/20'
  const c = habit.color
  const alphas = ['', '/25', '/45', '/65', '/85']
  const alpha = alphas[intensity] ?? '/85'
  return `bg-${c}-500${alpha}`
}

function getStreak(habit: Habit): number {
  let streak = 0
  const today = habitStore.todayLocal()
  for (let i = 0; i < 365; i++) {
    const d = new Date()
    d.setDate(d.getDate() - i)
    const dateStr = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`
    const entry = habitStore.getEntryForDate(habit.id, dateStr)
    const done = entry && entry.value > 0
    if (dateStr === today && !done) continue  // today not counted yet if not done
    if (done) {
      streak++
    } else {
      break
    }
  }
  return streak
}
</script>

<template>
  <div class="h-full w-full overflow-y-auto p-6 space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-xl font-bold text-foreground tracking-tight">Habits</h1>
        <p class="text-xs text-muted-foreground mt-0.5">Track your daily habits and build consistency</p>
      </div>
      <button
        @click="openAdd"
        class="flex items-center gap-2 px-3 py-1.5 bg-primary/10 hover:bg-primary/20 text-primary border border-primary/20 rounded-lg text-xs font-semibold transition-all cursor-pointer"
      >
        <Plus class="h-3.5 w-3.5" />
        Add Habit
      </button>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="flex items-center justify-center h-40 text-muted-foreground text-sm">
      Loading habits...
    </div>

    <!-- Empty State -->
    <div v-else-if="activeHabits.length === 0" class="flex flex-col items-center justify-center h-60 gap-3">
      <div class="text-4xl">🌱</div>
      <p class="text-muted-foreground text-sm">No habits yet. Add your first one!</p>
      <button @click="openAdd" class="px-4 py-2 bg-primary text-white rounded-lg text-sm font-semibold cursor-pointer hover:bg-primary/90 transition-colors">
        Add Habit
      </button>
    </div>

    <!-- Habit Cards -->
    <div v-else class="grid grid-cols-1 gap-4">
      <div
        v-for="habit in activeHabits"
        :key="habit.id"
        class="rounded-xl border border-border/40 bg-muted/10 p-4 space-y-3 hover:border-border/60 transition-colors"
      >
        <!-- Card Header -->
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2.5">
            <span v-if="habit.icon" class="text-lg leading-none">{{ habit.icon }}</span>
            <div
              v-else
              class="h-3 w-3 rounded-full"
              :class="`bg-${habit.color}-500`"
            ></div>
            <div>
              <div class="text-sm font-semibold text-foreground">{{ habit.name }}</div>
              <div v-if="habit.description" class="text-xs text-muted-foreground">{{ habit.description }}</div>
            </div>
          </div>

          <div class="flex items-center gap-1.5">
            <!-- Streak -->
            <div class="flex items-center gap-1 px-2 py-0.5 rounded-md bg-orange-500/10 text-orange-400 text-xs font-bold">
              <Flame class="h-3 w-3" />
              {{ getStreak(habit) }}
            </div>

            <!-- Today value & quick log -->
            <div class="flex items-center gap-1">
              <template v-if="habit.tracking_type === 'boolean'">
                <button
                  @click="habitStore.upsertEntry({ habit_id: habit.id, entry_date: habitStore.todayLocal(), value: habitStore.getTodayValue(habit.id) > 0 ? 0 : 1 })"
                  class="h-7 w-7 flex items-center justify-center rounded-lg border transition-all cursor-pointer"
                  :class="habitStore.getTodayValue(habit.id) > 0 ? 'bg-green-500/20 border-green-500/40 text-green-400' : 'border-border/40 text-muted-foreground hover:border-border/60'"
                >
                  <Check class="h-3.5 w-3.5" />
                </button>
              </template>
              <template v-else-if="habit.tracking_type === 'counter'">
                <button
                  @click="habitStore.decrement(habit.id)"
                  class="h-7 w-7 flex items-center justify-center rounded-lg border border-border/40 text-muted-foreground hover:bg-muted cursor-pointer transition-all"
                >
                  <Minus class="h-3.5 w-3.5" />
                </button>
                <span class="text-sm font-bold text-foreground w-6 text-center">{{ habitStore.getTodayValue(habit.id) }}</span>
                <button
                  @click="habitStore.increment(habit.id)"
                  class="h-7 w-7 flex items-center justify-center rounded-lg border border-border/40 text-muted-foreground hover:bg-muted cursor-pointer transition-all"
                >
                  <Plus class="h-3.5 w-3.5" />
                </button>
              </template>
              <template v-else>
                <span class="text-sm font-bold text-foreground px-2">
                  {{ habitStore.getTodayValue(habit.id) }}
                  <span v-if="habit.unit" class="text-xs text-muted-foreground font-normal">{{ habit.unit }}</span>
                </span>
              </template>
            </div>

            <!-- Goal indicator -->
            <div v-if="habit.daily_goal" class="flex items-center gap-1 text-xs text-muted-foreground">
              <Target class="h-3 w-3" />
              <span>{{ habit.daily_goal }}{{ habit.unit ? ' ' + habit.unit : '' }}</span>
            </div>

            <!-- Actions -->
            <button @click="openEdit(habit)" class="h-7 w-7 flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted cursor-pointer transition-all">
              <Edit2 class="h-3.5 w-3.5" />
            </button>
            <button @click="archiveHabit(habit.id)" class="h-7 w-7 flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted cursor-pointer transition-all">
              <Archive class="h-3.5 w-3.5" />
            </button>
            <button @click="deleteHabit(habit.id)" class="h-7 w-7 flex items-center justify-center rounded-lg text-muted-foreground hover:text-red-400 hover:bg-red-500/10 cursor-pointer transition-all">
              <Trash2 class="h-3.5 w-3.5" />
            </button>
          </div>
        </div>

        <!-- Contribution Grid -->
        <div class="overflow-x-auto">
          <div class="flex gap-0.5" style="min-width: max-content;">
            <div
              v-for="date in gridDates"
              :key="date"
              :title="`${date}: ${habitStore.getEntryForDate(habit.id, date)?.value ?? 0} ${habit.unit ?? ''}`"
              class="h-3.5 w-3.5 rounded-sm flex-shrink-0 transition-colors"
              :class="cellColorClass(habit, cellIntensity(habit, date))"
            ></div>
          </div>
          <div class="flex justify-between mt-1 text-[9px] text-muted-foreground/50">
            <span>12 weeks ago</span>
            <span>Today</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Add/Edit Dialog -->
    <Teleport to="body">
      <div v-if="showAddDialog" class="fixed inset-0 z-50 flex items-center justify-center">
        <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="showAddDialog = false"></div>
        <div class="relative w-full max-w-md bg-background border border-border/50 rounded-2xl shadow-2xl p-6 space-y-4 z-10">
          <!-- Dialog Header -->
          <div class="flex items-center justify-between">
            <h2 class="text-base font-bold text-foreground">{{ editingHabit ? 'Edit Habit' : 'New Habit' }}</h2>
            <button @click="showAddDialog = false" class="h-7 w-7 flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted cursor-pointer transition-all">
              <X class="h-4 w-4" />
            </button>
          </div>

          <!-- Form -->
          <div class="space-y-3">
            <!-- Name -->
            <div>
              <label class="text-xs text-muted-foreground block mb-1">Name *</label>
              <input
                v-model="form.name"
                type="text"
                placeholder="e.g. Read 30 min"
                class="w-full px-3 py-2 bg-muted/30 border border-border/40 rounded-lg text-sm text-foreground placeholder:text-muted-foreground/50 focus:outline-none focus:border-primary/50 transition-colors"
              />
            </div>

            <!-- Icon + Color Row -->
            <div class="flex gap-3">
              <div class="flex-1">
                <label class="text-xs text-muted-foreground block mb-1">Icon (emoji)</label>
                <input
                  v-model="form.icon"
                  type="text"
                  placeholder="📚"
                  class="w-full px-3 py-2 bg-muted/30 border border-border/40 rounded-lg text-sm text-foreground placeholder:text-muted-foreground/50 focus:outline-none focus:border-primary/50 transition-colors"
                />
              </div>
              <div class="flex-1">
                <label class="text-xs text-muted-foreground block mb-1">Color</label>
                <div class="flex gap-1.5 flex-wrap">
                  <button
                    v-for="c in colorOptions"
                    :key="c.value"
                    @click="form.color = c.value"
                    class="h-6 w-6 rounded-full border-2 transition-all cursor-pointer"
                    :class="[c.cls, form.color === c.value ? 'border-foreground scale-110' : 'border-transparent opacity-60 hover:opacity-100']"
                    :title="c.label"
                  ></button>
                </div>
              </div>
            </div>

            <!-- Description -->
            <div>
              <label class="text-xs text-muted-foreground block mb-1">Description</label>
              <input
                v-model="form.description"
                type="text"
                placeholder="Optional description"
                class="w-full px-3 py-2 bg-muted/30 border border-border/40 rounded-lg text-sm text-foreground placeholder:text-muted-foreground/50 focus:outline-none focus:border-primary/50 transition-colors"
              />
            </div>

            <!-- Type -->
            <div v-if="!editingHabit">
              <label class="text-xs text-muted-foreground block mb-1">Tracking Type</label>
              <select
                v-model="form.tracking_type"
                class="w-full px-3 py-2 bg-muted/30 border border-border/40 rounded-lg text-sm text-foreground focus:outline-none focus:border-primary/50 transition-colors cursor-pointer"
              >
                <option v-for="t in typeOptions" :key="t.value" :value="t.value">{{ t.label }}</option>
              </select>
            </div>

            <!-- Unit + Goal -->
            <div class="flex gap-3" v-if="['counter', 'duration', 'quantity'].includes(form.tracking_type)">
              <div class="flex-1">
                <label class="text-xs text-muted-foreground block mb-1">Unit</label>
                <input
                  v-model="form.unit"
                  type="text"
                  placeholder="cigarettes, pages, min..."
                  class="w-full px-3 py-2 bg-muted/30 border border-border/40 rounded-lg text-sm text-foreground placeholder:text-muted-foreground/50 focus:outline-none focus:border-primary/50 transition-colors"
                />
              </div>
              <div class="flex-1">
                <label class="text-xs text-muted-foreground block mb-1">Daily Goal</label>
                <input
                  v-model.number="form.daily_goal"
                  type="number"
                  min="0"
                  placeholder="Optional"
                  class="w-full px-3 py-2 bg-muted/30 border border-border/40 rounded-lg text-sm text-foreground placeholder:text-muted-foreground/50 focus:outline-none focus:border-primary/50 transition-colors"
                />
              </div>
            </div>

            <!-- Direction -->
            <div>
              <label class="text-xs text-muted-foreground block mb-1">Direction</label>
              <div class="flex gap-2">
                <button
                  @click="form.direction = 'build'"
                  class="flex-1 py-1.5 rounded-lg text-xs font-semibold border transition-all cursor-pointer"
                  :class="form.direction === 'build' ? 'bg-green-500/20 border-green-500/40 text-green-400' : 'border-border/40 text-muted-foreground hover:bg-muted'"
                >
                  Build (more = better)
                </button>
                <button
                  @click="form.direction = 'limit'"
                  class="flex-1 py-1.5 rounded-lg text-xs font-semibold border transition-all cursor-pointer"
                  :class="form.direction === 'limit' ? 'bg-red-500/20 border-red-500/40 text-red-400' : 'border-border/40 text-muted-foreground hover:bg-muted'"
                >
                  Limit (less = better)
                </button>
              </div>
            </div>
          </div>

          <!-- Actions -->
          <div class="flex gap-2 pt-1">
            <button
              @click="showAddDialog = false"
              class="flex-1 py-2 rounded-lg border border-border/40 text-sm font-semibold text-muted-foreground hover:bg-muted cursor-pointer transition-colors"
            >
              Cancel
            </button>
            <button
              @click="submitHabit"
              :disabled="!form.name.trim()"
              class="flex-1 py-2 rounded-lg bg-primary text-white text-sm font-semibold cursor-pointer hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {{ editingHabit ? 'Save Changes' : 'Create Habit' }}
            </button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>
