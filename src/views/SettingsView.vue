<script lang="ts" setup>
import { ref } from 'vue'
import { useSyncStore } from '@/stores/sync.store'
import { useSettingsStore } from '@/stores/settings.store'
import { storeToRefs } from 'pinia'
import {
  LogIn, LogOut, RefreshCw, Wifi, WifiOff,
  Keyboard, Sliders, Database, Cloud, Download
} from 'lucide-vue-next'
import { Switch } from '@/components/ui/switch'
import { Button } from '@/components/ui/button'
import { toast } from 'vue-sonner'
import { api } from '@/lib/tauri'
import { supabaseUrl, supabaseAnonKey, configureSupabase } from '@/lib/supabase'

const syncStore = useSyncStore()
const { isConfigured, isAuthenticated, userEmail, syncing, error, lastSyncedAt } = storeToRefs(syncStore)
const settingsStore = useSettingsStore()

// ── Supabase UI Configuration ──────────────────────────────────
const supabaseUrlInput = ref(supabaseUrl.value)
const supabaseKeyInput = ref(supabaseAnonKey.value)

async function saveConnection() {
  if (!supabaseUrlInput.value || !supabaseKeyInput.value) {
    toast.error('Inserisci sia l\'URL che la chiave Anon.')
    return
  }
  try {
    configureSupabase(supabaseUrlInput.value, supabaseKeyInput.value)
    await syncStore.init() // Re-inizializza lo store per usare il nuovo client
    toast.success('Connessione a Supabase configurata con successo!')
  } catch (e: any) {
    toast.error('Errore durante la configurazione: ' + e.message)
  }
}

function clearConnection() {
  configureSupabase('', '')
  supabaseUrlInput.value = ''
  supabaseKeyInput.value = ''
  syncStore.init()
  toast.info('Connessione a Supabase rimossa.')
}

// ── Auth Form ──────────────────────────────────────────────────
const authMode = ref<'login' | 'signup'>('login')
const authForm = ref({ email: '', password: '' })
const authLoading = ref(false)
const authError = ref<string | null>(null)
const authSuccess = ref<string | null>(null)

async function submitAuth() {
  authLoading.value = true
  authError.value = null
  authSuccess.value = null
  try {
    if (authMode.value === 'login') {
      await syncStore.login(authForm.value.email, authForm.value.password)
    } else {
      await syncStore.signup(authForm.value.email, authForm.value.password)
      authSuccess.value = 'Account creato! Controlla la tua email per verificare.'
    }
    authForm.value = { email: '', password: '' }
  } catch (e: any) {
    authError.value = e.message || e.toString()
  } finally {
    authLoading.value = false
  }
}

async function handleLogout() {
  await syncStore.logout()
}

async function syncNow() {
  await syncStore.sync()
}

function formatLastSync(iso: string | null): string {
  if (!iso) return 'Never'
  const d = new Date(iso)
  return d.toLocaleString()
}

// ── Keybinds reference ────────────────────────────────────────
const keybinds = [
  { key: 'Space', action: 'Avvia / Pausa timer (quando non in input)' },
  { key: 'F', action: 'Naviga a Focus' },
  { key: 'S', action: 'Naviga a Statistiche' },
  { key: ', / Ctrl+,', action: 'Naviga a Impostazioni' },
  { key: 'F11', action: 'Attiva/Disattiva Schermo Intero / Full Focus' },
  { key: 'Escape', action: 'Esci da Schermo Intero' },
]

const activeSection = ref<'preferences' | 'keybinds' | 'cloud' | 'data'>('preferences')

async function exportData(format: 'json' | 'csv' | 'markdown') {
  const start = new Date(2020, 0, 1).toISOString()
  const end = new Date(2030, 0, 1).toISOString()
  const types = ['study_session'] // esporta solo le sessioni di studio dopo la semplificazione
  
  try {
    const rawData = await api.export.exportData(format, start, end, types)
    if (!rawData) {
      toast.info('Nessun dato da esportare.')
      return
    }
    const blob = new Blob([rawData], { type: 'text/plain;charset=utf-8' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    
    let ext = 'csv'
    if (format === 'json') ext = 'json'
    else if (format === 'markdown') ext = 'md'
    
    a.download = `studytimer_export_${new Date().toISOString().slice(0, 10)}.${ext}`
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
  <div class="h-full w-full overflow-y-auto p-6">
    <div class="max-w-2xl mx-auto space-y-6">
      <!-- Header -->
      <div>
        <h1 class="text-xl font-bold text-foreground tracking-tight">Settings</h1>
        <p class="text-xs text-muted-foreground mt-0.5">Configure your study environment</p>
      </div>

      <!-- Section Tabs -->
      <div class="flex gap-1 bg-muted/30 p-0.5 rounded-lg border border-border/30 w-fit">
        <button
          v-for="s in [
            { id: 'preferences', label: 'Preferences', icon: Sliders },
            { id: 'keybinds', label: 'Keybinds', icon: Keyboard },
            { id: 'cloud', label: 'Cloud Sync', icon: Cloud },
            { id: 'data', label: 'Data', icon: Database },
          ]"
          :key="s.id"
          @click="activeSection = s.id as any"
          class="flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-semibold transition-all cursor-pointer"
          :class="activeSection === s.id ? 'bg-background text-foreground shadow-sm border border-border/30' : 'text-muted-foreground hover:text-foreground'"
        >
          <component :is="s.icon" class="h-3.5 w-3.5" />
          {{ s.label }}
        </button>
      </div>

      <!-- ── Preferences ─────────────────────────────────────── -->
      <div v-if="activeSection === 'preferences'" class="space-y-3">
        <div class="rounded-xl border border-border/40 bg-muted/10 p-4 space-y-4">
          <div class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Timer Settings</div>
          
          <div class="space-y-3">
            <!-- Default Duration Select -->
            <div class="flex items-center justify-between py-1">
              <div>
                <div class="text-sm text-foreground">Default Focus Duration</div>
                <div class="text-xs text-muted-foreground">Initial duration for new focus sessions</div>
              </div>
              <select
                v-model="settingsStore.defaultFocusDuration"
                class="bg-card border border-border text-foreground text-xs p-1.5 rounded-lg font-semibold focus:outline-none focus:border-primary font-mono cursor-pointer"
              >
                <option :value="900">15 minutes</option>
                <option :value="1500">25 minutes</option>
                <option :value="1800">30 minutes</option>
                <option :value="2700">45 minutes</option>
                <option :value="3600">60 minutes</option>
              </select>
            </div>

            <!-- Auto-start switch -->
            <div class="flex items-center justify-between py-1 border-t border-border/20">
              <div>
                <div class="text-sm text-foreground">Auto-start Pomodoro</div>
                <div class="text-xs text-muted-foreground">Automatically start the next focus/break timer interval</div>
              </div>
              <Switch v-model:checked="settingsStore.autoStart" />
            </div>
          </div>
        </div>

        <!-- Appearance Card -->
        <div class="rounded-xl border border-border/40 bg-muted/10 p-4 space-y-4">
          <div class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Appearance</div>
          
          <div class="space-y-3">
            <!-- Theme Toggle -->
            <div class="flex items-center justify-between py-1">
              <div>
                <div class="text-sm text-foreground">Theme</div>
                <div class="text-xs text-muted-foreground">Toggle application theme</div>
              </div>
              <div class="flex gap-1 bg-muted/30 p-0.5 rounded-lg border border-border/30 w-fit">
                <button
                  @click="settingsStore.theme = 'dark'"
                  class="px-2.5 py-1 rounded-md text-xs font-semibold transition-all cursor-pointer"
                  :class="settingsStore.theme === 'dark' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'"
                >Dark</button>
                <button
                  @click="settingsStore.theme = 'light'"
                  class="px-2.5 py-1 rounded-md text-xs font-semibold transition-all cursor-pointer"
                  :class="settingsStore.theme === 'light' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'"
                >Light</button>
              </div>
            </div>

            <!-- Accent Color Picker -->
            <div class="flex items-center justify-between py-1 border-t border-border/20">
              <div>
                <div class="text-sm text-foreground">Accent Color</div>
                <div class="text-xs text-muted-foreground">Customize color accents across the UI</div>
              </div>
              <div class="flex gap-2">
                <button
                  v-for="c in [
                    { id: 'red', color: 'bg-red-500' },
                    { id: 'green', color: 'bg-emerald-500' },
                    { id: 'blue', color: 'bg-blue-500' },
                    { id: 'amber', color: 'bg-amber-500' },
                    { id: 'violet', color: 'bg-violet-500' },
                    { id: 'neutral', color: 'bg-neutral-400' }
                  ]"
                  :key="c.id"
                  @click="settingsStore.accentColor = c.id as any"
                  class="h-5 w-5 rounded-full cursor-pointer transition-transform hover:scale-110 relative"
                  :class="[c.color, settingsStore.accentColor === c.id ? 'ring-2 ring-offset-2 ring-primary scale-110' : '']"
                  :title="c.id"
                ></button>
              </div>
            </div>
            
            <div class="flex items-center justify-between py-1 border-t border-border/20">
              <div>
                <div class="text-sm text-foreground">Close Button Action</div>
                <div class="text-xs text-muted-foreground">Closes to system tray instead of quitting</div>
              </div>
              <span class="text-xs text-green-400 bg-green-500/10 px-2 py-1 rounded border border-green-500/20">Minimize to Tray</span>
            </div>
          </div>
        </div>

        <div class="rounded-xl border border-border/40 bg-muted/10 p-4 space-y-2">
          <div class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-3">About</div>
          <div class="text-sm text-foreground">StudyTimer — Focused Study Timer</div>
          <div class="text-xs text-muted-foreground">Offline-first desktop app. Powered by Tauri 2, Vue 3, and SQLite.</div>
        </div>
      </div>

      <!-- ── Keybinds ────────────────────────────────────────── -->
      <div v-if="activeSection === 'keybinds'" class="space-y-3">
        <!-- Keybind toggles -->
        <div class="rounded-xl border border-border/40 bg-muted/10 p-4 space-y-4">
          <div class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Configure Keybinds</div>
          <div class="space-y-3">
            <div class="flex items-center justify-between py-1">
              <div>
                <div class="text-sm text-foreground">Space Key</div>
                <div class="text-xs text-muted-foreground">Press Space to Start / Pause timer (when not focused on input)</div>
              </div>
              <Switch v-model:checked="settingsStore.spaceStartStop" />
            </div>
            
            <div class="flex items-center justify-between py-1 border-t border-border/20">
              <div>
                <div class="text-sm text-foreground">R Key</div>
                <div class="text-xs text-muted-foreground">Press R to Reset the active timer (when not focused on input)</div>
              </div>
              <Switch v-model:checked="settingsStore.rReset" />
            </div>
          </div>
        </div>

        <!-- Shortcuts reference -->
        <div class="rounded-xl border border-border/40 bg-muted/10 p-4 space-y-1">
          <div class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-3">Keyboard Shortcuts Reference</div>
          <div
            v-for="kb in keybinds"
            :key="kb.key"
            class="flex items-center justify-between py-2 border-b border-border/20 last:border-0"
          >
            <span class="text-sm text-muted-foreground">{{ kb.action }}</span>
            <kbd class="px-2 py-0.5 bg-muted/40 border border-border/40 rounded text-xs font-mono text-foreground">{{ kb.key }}</kbd>
          </div>
        </div>
        <div class="text-xs text-muted-foreground px-1">
          Note: Navigation shortcuts only work when not typing in an input field.
        </div>
      </div>

      <!-- ── Cloud Sync ──────────────────────────────────────── -->
      <div v-if="activeSection === 'cloud'" class="space-y-4">
        <!-- Supabase Connection Config Card -->
        <div class="rounded-xl border border-border/40 bg-muted/10 p-4 space-y-4">
          <div class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Supabase Connection Settings</div>
          
          <div class="space-y-3">
            <div class="space-y-1.5">
              <label class="text-xs text-muted-foreground">Supabase URL</label>
              <input
                v-model="supabaseUrlInput"
                type="text"
                placeholder="https://your-project.supabase.co"
                class="w-full px-3 py-1.5 bg-card border border-border rounded-lg text-xs font-semibold focus:outline-none focus:border-primary font-mono text-foreground"
              />
            </div>
            
            <div class="space-y-1.5">
              <label class="text-xs text-muted-foreground">Supabase Anon Key</label>
              <input
                v-model="supabaseKeyInput"
                type="password"
                placeholder="your-anon-key"
                class="w-full px-3 py-1.5 bg-card border border-border rounded-lg text-xs font-semibold focus:outline-none focus:border-primary font-mono text-foreground"
              />
            </div>

            <div class="flex gap-2 pt-1">
              <Button
                size="sm"
                class="bg-primary hover:bg-primary/95 text-white font-semibold text-xs cursor-pointer px-4 h-8.5 rounded-lg"
                @click="saveConnection"
              >
                Salva Connessione
              </Button>
              <Button
                size="sm"
                variant="outline"
                class="border-border hover:bg-muted text-muted-foreground hover:text-foreground font-semibold text-xs cursor-pointer px-4 h-8.5 rounded-lg border"
                @click="clearConnection"
                v-if="isConfigured"
              >
                Rimuovi Connessione
              </Button>
            </div>
          </div>
        </div>

        <!-- Status Card -->
        <div v-if="isConfigured" class="rounded-xl border border-border/40 bg-muted/10 p-4 space-y-3">
          <div class="flex items-center justify-between pb-1 border-b border-border/20">
            <div class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Supabase Status</div>
            <div class="flex items-center gap-2">
              <span class="text-xs text-muted-foreground">Auto-Sync</span>
              <Switch v-model:checked="settingsStore.autoSync" />
            </div>
          </div>

          <!-- Auth Status -->
          <div class="flex items-center gap-2">
            <component
              :is="isAuthenticated ? Wifi : WifiOff"
              class="h-4 w-4"
              :class="isAuthenticated ? 'text-green-400' : 'text-muted-foreground'"
            />
            <span class="text-sm" :class="isAuthenticated ? 'text-green-400' : 'text-muted-foreground'">
              {{ isAuthenticated ? `Signed in as ${userEmail}` : 'Not signed in' }}
            </span>
          </div>

          <!-- Last Sync -->
          <div class="text-xs text-muted-foreground font-mono">
            Last sync: {{ formatLastSync(lastSyncedAt) }}
          </div>

          <!-- Error -->
          <div v-if="error" class="text-xs text-red-400 bg-red-500/10 border border-red-500/20 rounded-lg px-3 py-2">
            {{ error }}
          </div>

          <!-- Sync Now -->
          <button
            v-if="isAuthenticated"
            @click="syncNow"
            :disabled="syncing"
            class="flex items-center gap-2 px-3 py-1.5 bg-primary/10 hover:bg-primary/20 text-primary border border-primary/20 rounded-lg text-xs font-semibold transition-all cursor-pointer disabled:opacity-50"
          >
            <RefreshCw class="h-3.5 w-3.5" :class="{ 'animate-spin': syncing }" />
            {{ syncing ? 'Syncing...' : 'Sync Now' }}
          </button>
        </div>

        <!-- Auth Form -->
        <div v-if="isConfigured && !isAuthenticated" class="rounded-xl border border-border/40 bg-muted/10 p-4 space-y-3">
          <div class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Sign In</div>

          <!-- Mode Toggle -->
          <div class="flex gap-1 bg-muted/30 p-0.5 rounded-lg border border-border/30 w-fit">
            <button
              @click="authMode = 'login'"
              class="px-3 py-1 rounded-md text-xs font-semibold transition-all cursor-pointer"
              :class="authMode === 'login' ? 'bg-background text-foreground shadow-sm border border-border/30' : 'text-muted-foreground hover:text-foreground'"
            >Login</button>
            <button
              @click="authMode = 'signup'"
              class="px-3 py-1 rounded-md text-xs font-semibold transition-all cursor-pointer"
              :class="authMode === 'signup' ? 'bg-background text-foreground shadow-sm border border-border/30' : 'text-muted-foreground hover:text-foreground'"
            >Sign Up</button>
          </div>

          <div class="space-y-2">
            <input
              v-model="authForm.email"
              type="email"
              placeholder="Email"
              class="w-full px-3 py-2 bg-muted/30 border border-border/40 rounded-lg text-sm text-foreground placeholder:text-muted-foreground/50 focus:outline-none focus:border-primary/50 transition-colors"
            />
            <input
              v-model="authForm.password"
              type="password"
              placeholder="Password"
              @keydown.enter="submitAuth"
              class="w-full px-3 py-2 bg-muted/30 border border-border/40 rounded-lg text-sm text-foreground placeholder:text-muted-foreground/50 focus:outline-none focus:border-primary/50 transition-colors"
            />
          </div>

          <div v-if="authError" class="text-xs text-red-400 bg-red-500/10 border border-red-500/20 rounded-lg px-3 py-2">{{ authError }}</div>
          <div v-if="authSuccess" class="text-xs text-green-400 bg-green-500/10 border border-green-500/20 rounded-lg px-3 py-2">{{ authSuccess }}</div>

          <button
            @click="submitAuth"
            :disabled="authLoading || !authForm.email || !authForm.password"
            class="flex items-center gap-2 px-4 py-2 bg-primary text-white rounded-lg text-sm font-semibold cursor-pointer hover:bg-primary/90 disabled:opacity-50 transition-colors"
          >
            <LogIn class="h-3.5 w-3.5" />
            {{ authLoading ? 'Loading...' : authMode === 'login' ? 'Login' : 'Create Account' }}
          </button>
        </div>

        <!-- Logout -->
        <div v-if="isConfigured && isAuthenticated" class="rounded-xl border border-border/40 bg-muted/10 p-4">
          <button
            @click="handleLogout"
            class="flex items-center gap-2 px-3 py-1.5 border border-border/40 hover:bg-muted text-muted-foreground hover:text-foreground rounded-lg text-xs font-semibold transition-all cursor-pointer"
          >
            <LogOut class="h-3.5 w-3.5" />
            Sign Out
          </button>
        </div>
      </div>

      <!-- ── Data ───────────────────────────────────────────── -->
      <div v-if="activeSection === 'data'" class="space-y-3">
        <div class="rounded-xl border border-border/40 bg-muted/10 p-4 space-y-3">
          <div class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Local Storage</div>
          <div class="text-sm text-muted-foreground">All data is stored locally in SQLite at your system's app data directory.</div>
        </div>

        <!-- Export Card -->
        <div class="rounded-xl border border-border/40 bg-muted/10 p-4 space-y-4">
          <div class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Export Data</div>
          <div class="text-xs text-muted-foreground">Export your history of study sessions.</div>
          
          <div class="flex flex-wrap gap-2 pt-1">
            <Button
              size="sm"
              variant="outline"
              class="flex items-center gap-1.5 cursor-pointer font-semibold border"
              @click="exportData('json')"
            >
              <Download class="h-3.5 w-3.5" />
              <span>JSON</span>
            </Button>
            <Button
              size="sm"
              variant="outline"
              class="flex items-center gap-1.5 cursor-pointer font-semibold border"
              @click="exportData('csv')"
            >
              <Download class="h-3.5 w-3.5" />
              <span>CSV</span>
            </Button>
            <Button
              size="sm"
              variant="outline"
              class="flex items-center gap-1.5 cursor-pointer font-semibold border"
              @click="exportData('markdown')"
            >
              <Download class="h-3.5 w-3.5" />
              <span>Markdown</span>
            </Button>
          </div>
        </div>

        <div class="rounded-xl border border-red-500/20 bg-red-500/5 p-4 space-y-2">
          <div class="text-xs font-semibold text-red-400">Danger Zone</div>
          <div class="text-xs text-muted-foreground">To reset all local data, delete the SQLite database from your app data directory and restart the app.</div>
        </div>
      </div>
    </div>
  </div>
</template>
