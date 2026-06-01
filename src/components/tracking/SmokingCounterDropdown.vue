<script lang="ts" setup>
import { onMounted } from 'vue'
import { storeToRefs } from 'pinia'
import { Cigarette, Plus, Minus, Skull } from 'lucide-vue-next'
import { useSmokingStore } from '@/stores/smoking.store'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'

const smokingStore = useSmokingStore()
const { countToday, loading } = storeToRefs(smokingStore)

onMounted(async () => {
  await smokingStore.loadSmokingTodayCount()
})
</script>

<template>
  <DropdownMenu>
    <DropdownMenuTrigger as-child>
      <Button
        variant="outline"
        size="icon"
        class="h-8 w-8 rounded-lg border-primary/20 bg-background/50 hover:bg-accent hover:text-accent-foreground relative cursor-pointer"
        :class="{ 'border-primary/50 text-primary shadow-[0_0_8px_rgba(239,68,68,0.2)]': countToday > 0 }"
      >
        <Cigarette class="h-4 w-4" />
        <span
          v-if="countToday > 0"
          class="absolute -top-1 -right-1 flex h-4 w-4 items-center justify-center rounded-full bg-primary text-[9px] font-bold text-white shadow-sm font-mono"
        >
          {{ countToday }}
        </span>
      </Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent
      align="start"
      :side-offset="8"
      class="w-72 border-border bg-background/95 backdrop-blur-md p-4 text-foreground rounded-xl shadow-2xl z-50"
    >
      <div class="space-y-4">
        <!-- Warnings / Headers -->
        <div class="text-center space-y-1">
          <div class="inline-flex items-center gap-1.5 px-2 py-0.5 rounded bg-primary/10 border border-primary/20 text-primary text-[10px] font-bold uppercase tracking-wider">
            <Skull class="h-3 w-3 animate-pulse" />
            Il fumo uccide
          </div>
          <p class="text-[9px] text-muted-foreground italic max-w-[240px] mx-auto mt-1 leading-normal">
            Tracciare serve a smettere, non a normalizzare.
          </p>
        </div>

        <!-- Today count -->
        <div class="flex flex-col items-center justify-center py-2 bg-muted/30 rounded-lg border border-border/30">
          <span class="text-3xl font-extrabold tracking-tight font-mono text-foreground">
            {{ countToday }}
          </span>
          <span class="text-[10px] text-muted-foreground uppercase tracking-widest mt-1">
            Sigarette oggi
          </span>
        </div>

        <!-- Controls -->
        <div class="flex gap-2">
          <Button
            variant="outline"
            class="flex-1 h-9 gap-1.5 border-border hover:bg-muted/80 hover:text-foreground text-xs cursor-pointer"
            :disabled="loading || countToday === 0"
            @click="smokingStore.removeLastCigaretteToday()"
          >
            <Minus class="h-3.5 w-3.5" />
            <span>Rimuovi</span>
          </Button>

          <Button
            variant="default"
            class="flex-1 h-9 gap-1.5 bg-primary hover:bg-primary/95 text-white font-semibold text-xs cursor-pointer shadow-[0_0_10px_rgba(239,68,68,0.2)]"
            :disabled="loading"
            @click="smokingStore.addCigarette()"
          >
            <Plus class="h-3.5 w-3.5" />
            <span>Aggiungi</span>
          </Button>
        </div>
      </div>
    </DropdownMenuContent>
  </DropdownMenu>
</template>
