<script lang="ts" setup>
import { ref } from 'vue'
import { useSyncStore } from '@/stores/sync.store'
import { storeToRefs } from 'pinia'
import {
  LogIn, LogOut, RefreshCw, Wifi, WifiOff, CheckCircle2,
  AlertCircle, Keyboard, Sliders, Database, Cloud
} from 'lucide-vue-next'

const syncStore = useSyncStore()
const { isConfigured, isAuthenticated, userEmail, syncing, error, lastSyncedAt } = storeToRefs(syncStore)

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
      authSuccess.value = 'Account created! Check your email to verify.'
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
  { key: 'Space', action: 'Start / Pause timer (when not in input)' },
  { key: 'F', action: 'Navigate to Focus' },
  { key: 'S', action: 'Navigate to Stats' },
  { key: 'H', action: 'Navigate to Habits' },
  { key: 'W', action: 'Navigate to Workouts' },
  { key: ', / Ctrl+,', action: 'Navigate to Settings' },
  { key: 'F11', action: 'Toggle fullscreen / Full Focus mode' },
  { key: 'Escape', action: 'Exit fullscreen' },
]

const activeSection = ref<'preferences' | 'keybinds' | 'cloud' | 'data'>('preferences')
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
          <div class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Timer</div>
          <div class="space-y-3">
            <div class="flex items-center justify-between py-1">
              <div>
                <div class="text-sm text-foreground">Default Focus Duration</div>
                <div class="text-xs text-muted-foreground">Set in the timer presets on the Focus page</div>
              </div>
              <span class="text-xs text-muted-foreground bg-muted/40 px-2 py-1 rounded">Focus page</span>
            </div>
            <div class="flex items-center justify-between py-1 border-t border-border/20">
              <div>
                <div class="text-sm text-foreground">Theme</div>
                <div class="text-xs text-muted-foreground">Dark mode only (more themes coming)</div>
              </div>
              <span class="text-xs text-muted-foreground bg-muted/40 px-2 py-1 rounded">Dark</span>
            </div>
            <div class="flex items-center justify-between py-1 border-t border-border/20">
              <div>
                <div class="text-sm text-foreground">Font</div>
                <div class="text-xs text-muted-foreground">Monospace font for that terminal aesthetic</div>
              </div>
              <span class="text-xs text-muted-foreground bg-muted/40 px-2 py-1 rounded">JetBrains Mono</span>
            </div>
            <div class="flex items-center justify-between py-1 border-t border-border/20">
              <div>
                <div class="text-sm text-foreground">Close Button</div>
                <div class="text-xs text-muted-foreground">Closes to system tray instead of quitting</div>
              </div>
              <span class="text-xs text-green-400 bg-green-500/10 px-2 py-1 rounded border border-green-500/20">Minimize to Tray</span>
            </div>
          </div>
        </div>

        <div class="rounded-xl border border-border/40 bg-muted/10 p-4 space-y-2">
          <div class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-3">About</div>
          <div class="text-sm text-foreground">StudyTimer — Personal Life Tracker</div>
          <div class="text-xs text-muted-foreground">Offline-first desktop app. Powered by Tauri 2, Vue 3, and SQLite.</div>
        </div>
      </div>

      <!-- ── Keybinds ────────────────────────────────────────── -->
      <div v-if="activeSection === 'keybinds'" class="space-y-3">
        <div class="rounded-xl border border-border/40 bg-muted/10 p-4 space-y-1">
          <div class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-3">Keyboard Shortcuts</div>
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
      <div v-if="activeSection === 'cloud'" class="space-y-3">
        <!-- Status Card -->
        <div class="rounded-xl border border-border/40 bg-muted/10 p-4 space-y-3">
          <div class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Supabase Status</div>

          <!-- Env Status -->
          <div class="flex items-center gap-2">
            <component
              :is="isConfigured ? CheckCircle2 : AlertCircle"
              class="h-4 w-4"
              :class="isConfigured ? 'text-green-400' : 'text-yellow-400'"
            />
            <span class="text-sm" :class="isConfigured ? 'text-green-400' : 'text-yellow-400'">
              {{ isConfigured ? 'Environment configured' : 'Missing .env.local — VITE_SUPABASE_URL / VITE_SUPABASE_ANON_KEY' }}
            </span>
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
          <div class="text-xs text-muted-foreground">
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
              :class="authMode === 'login' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'"
            >Login</button>
            <button
              @click="authMode = 'signup'"
              class="px-3 py-1 rounded-md text-xs font-semibold transition-all cursor-pointer"
              :class="authMode === 'signup' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'"
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
        <div v-if="isAuthenticated" class="rounded-xl border border-border/40 bg-muted/10 p-4">
          <button
            @click="handleLogout"
            class="flex items-center gap-2 px-3 py-1.5 border border-border/40 hover:bg-muted text-muted-foreground hover:text-foreground rounded-lg text-xs font-semibold transition-all cursor-pointer"
          >
            <LogOut class="h-3.5 w-3.5" />
            Sign Out
          </button>
        </div>

        <!-- Setup Help -->
        <div v-if="!isConfigured" class="rounded-xl border border-yellow-500/20 bg-yellow-500/5 p-4 space-y-2">
          <div class="text-xs font-semibold text-yellow-400">Setup Required</div>
          <div class="text-xs text-muted-foreground space-y-1">
            <p>Create <code class="text-foreground bg-muted/40 px-1 rounded">.env.local</code> in the project root with:</p>
            <pre class="text-xs text-foreground bg-muted/40 rounded-lg p-2 mt-1 font-mono">VITE_SUPABASE_URL=https://your.supabase.co
VITE_SUPABASE_ANON_KEY=your-anon-key</pre>
            <p>Then restart the dev server.</p>
          </div>
        </div>
      </div>

      <!-- ── Data ───────────────────────────────────────────── -->
      <div v-if="activeSection === 'data'" class="space-y-3">
        <div class="rounded-xl border border-border/40 bg-muted/10 p-4 space-y-3">
          <div class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Local Storage</div>
          <div class="text-sm text-muted-foreground">All data is stored locally in SQLite at your system's app data directory.</div>
          <div class="text-xs text-muted-foreground">Export and import features are available in the Stats page.</div>
        </div>
        <div class="rounded-xl border border-red-500/20 bg-red-500/5 p-4 space-y-2">
          <div class="text-xs font-semibold text-red-400">Danger Zone</div>
          <div class="text-xs text-muted-foreground">To reset all local data, delete the SQLite database from your app data directory and restart the app.</div>
        </div>
      </div>
    </div>
  </div>
</template>
