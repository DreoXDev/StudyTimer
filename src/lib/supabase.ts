import { createClient, SupabaseClient } from '@supabase/supabase-js'
import { ref } from 'vue'

const getInitialUrl = () => {
  return localStorage.getItem('st_supabase_url') || import.meta.env.VITE_SUPABASE_URL || ''
}

const getInitialKey = () => {
  return localStorage.getItem('st_supabase_anon_key') || import.meta.env.VITE_SUPABASE_ANON_KEY || ''
}

export const supabaseUrl = ref(getInitialUrl())
export const supabaseAnonKey = ref(getInitialKey())

export const isSupabaseConfigured = ref(!!(supabaseUrl.value && supabaseAnonKey.value))

let supabaseInstance: SupabaseClient | null = isSupabaseConfigured.value
  ? createClient(supabaseUrl.value, supabaseAnonKey.value)
  : null

export function getSupabase(): SupabaseClient | null {
  return supabaseInstance
}

export function configureSupabase(url: string, key: string) {
  supabaseUrl.value = url.trim()
  supabaseAnonKey.value = key.trim()
  
  if (supabaseUrl.value && supabaseAnonKey.value) {
    localStorage.setItem('st_supabase_url', supabaseUrl.value)
    localStorage.setItem('st_supabase_anon_key', supabaseAnonKey.value)
    isSupabaseConfigured.value = true
    supabaseInstance = createClient(supabaseUrl.value, supabaseAnonKey.value)
  } else {
    localStorage.removeItem('st_supabase_url')
    localStorage.removeItem('st_supabase_anon_key')
    isSupabaseConfigured.value = false
    supabaseInstance = null
  }
}
