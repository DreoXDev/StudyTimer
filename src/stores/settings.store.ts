import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

export type Theme = 'dark' | 'light'
export type AccentColor = 'red' | 'green' | 'blue' | 'amber' | 'violet' | 'neutral'

export const useSettingsStore = defineStore('settings', () => {
  // Load initial values from localStorage or defaults
  const theme = ref<Theme>((localStorage.getItem('st_theme') as Theme) || 'dark')
  const accentColor = ref<AccentColor>((localStorage.getItem('st_accent_color') as AccentColor) || 'red')
  const defaultFocusDuration = ref<number>(parseInt(localStorage.getItem('st_default_focus_duration') || '1500', 10)) // 25 min default
  const autoStart = ref<boolean>(localStorage.getItem('st_auto_start') === 'true')
  
  const spaceStartStop = ref<boolean>(localStorage.getItem('st_space_start_stop') !== 'false') // default true
  const rReset = ref<boolean>(localStorage.getItem('st_r_reset') !== 'false') // default true
  const autoSync = ref<boolean>(localStorage.getItem('st_auto_sync') !== 'false') // default true

  // Apply theme and accent color
  function applyTheme(t: Theme) {
    const root = document.documentElement
    if (t === 'dark') {
      root.classList.add('dark')
    } else {
      root.classList.remove('dark')
    }
  }

  function applyAccent(color: AccentColor) {
    const root = document.documentElement
    
    // Define accent mappings
    const accents: Record<AccentColor, { primary: string; ring: string; accent: string }> = {
      red: {
        primary: 'hsl(0 72% 51%)',
        ring: 'hsl(0 72% 51%)',
        accent: 'hsl(0 55% 22%)'
      },
      green: {
        primary: 'hsl(142 72% 45%)',
        ring: 'hsl(142 72% 45%)',
        accent: 'hsl(142 55% 20%)'
      },
      blue: {
        primary: 'hsl(217 91% 60%)',
        ring: 'hsl(217 91% 60%)',
        accent: 'hsl(217 55% 25%)'
      },
      amber: {
        primary: 'hsl(35 92% 50%)',
        ring: 'hsl(35 92% 50%)',
        accent: 'hsl(35 55% 22%)'
      },
      violet: {
        primary: 'hsl(262 83% 58%)',
        ring: 'hsl(262 83% 58%)',
        accent: 'hsl(262 55% 24%)'
      },
      neutral: {
        primary: 'hsl(0 0% 90%)',
        ring: 'hsl(0 0% 90%)',
        accent: 'hsl(0 0% 25%)'
      }
    }

    const val = accents[color] || accents.red
    root.style.setProperty('--primary', val.primary)
    root.style.setProperty('--ring', val.ring)
    root.style.setProperty('--accent', val.accent)
    root.style.setProperty('--sidebar-primary', val.primary)
    root.style.setProperty('--sidebar-ring', val.ring)
  }

  // Initialize
  function init() {
    applyTheme(theme.value)
    applyAccent(accentColor.value)
  }

  // Watchers to persist and apply settings
  watch(theme, (newTheme) => {
    localStorage.setItem('st_theme', newTheme)
    applyTheme(newTheme)
  })

  watch(accentColor, (newColor) => {
    localStorage.setItem('st_accent_color', newColor)
    applyAccent(newColor)
  })

  watch(defaultFocusDuration, (newDuration) => {
    localStorage.setItem('st_default_focus_duration', newDuration.toString())
  })

  watch(autoStart, (newVal) => {
    localStorage.setItem('st_auto_start', newVal ? 'true' : 'false')
  })

  watch(spaceStartStop, (newVal) => {
    localStorage.setItem('st_space_start_stop', newVal ? 'true' : 'false')
  })

  watch(rReset, (newVal) => {
    localStorage.setItem('st_r_reset', newVal ? 'true' : 'false')
  })

  watch(autoSync, (newVal) => {
    localStorage.setItem('st_auto_sync', newVal ? 'true' : 'false')
  })

  return {
    theme,
    accentColor,
    defaultFocusDuration,
    autoStart,
    spaceStartStop,
    rReset,
    autoSync,
    init,
    applyTheme,
    applyAccent
  }
})
