<script setup lang="ts">
import AppShell from './app/AppShell.vue'
import Toaster from '@/components/ui/sonner/Sonner.vue'
</script>

<template>
  <main class="dark min-h-screen bg-background text-foreground">
    <AppShell />
    <Toaster />
  </main>
</template>