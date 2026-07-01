# StudyTimer ⏱️

[![Tauri v2](https://img.shields.io/badge/Tauri-v2.0-FFC107?logo=tauri&logoColor=white&style=flat-flat)](https://tauri.app/)
[![Vue 3](https://img.shields.io/badge/Vue-3.x-4FC08D?logo=vue.js&logoColor=white&style=flat-flat)](https://vuejs.org/)
[![Rust](https://img.shields.io/badge/Rust-Backend-000000?logo=rust&logoColor=white&style=flat-flat)](https://www.rust-lang.org/)
[![Supabase](https://img.shields.io/badge/Supabase-Database-3ECF8E?logo=supabase&logoColor=white&style=flat-flat)](https://supabase.com/)

**StudyTimer** è un'applicazione desktop immersiva e minimalista per la gestione del tempo di studio, progettata specificamente per fungere da **companion screen** secondario durante le sessioni di concentrazione. Caratterizzata da un'estetica scura rilassante con elementi vetrosi (glassmorphism) e delicati accenti rossi, l'applicazione riduce al minimo le distrazioni aumentando la produttività.

---

## ✨ Caratteristiche Principali

- ⏱️ **Focus Dashboard Immersiva**: Un'interfaccia ultra-pulita priva di elementi superflui che mostra unicamente il timer digitale in riproduzione per massimizzare la concentrazione.
- ✏️ **Timer Digitale Interattivo**: Consente di modificare i minuti direttamente facendo click sulle cifre del timer (quando non attivo).
- 🎵 **Integrazione Media di Sistema Nativa (Windows GSMTC)**:
  - Si connette direttamente ai controlli audio globali di Windows (**Global System Media Transport Controls**).
  - Mostra titolo, artista, sorgente e progresso dell'audio di sistema in tempo reale (es. da Spotify, Chrome/YouTube, VLC).
  - Pulsanti di controllo nativi (Play/Pause, Successivo, Precedente) senza richiedere account web o chiavi API esterne.
  - Fallback automatico su mock-track interattivo su piattaforme non-Windows.
- ☁️ **Cloud Sync Offline-First (Supabase)**:
  - Archiviazione locale istantanea su database SQLite crittografato tramite il backend Rust.
  - Sincronizzazione bidirezionale asincrona che sincronizza i dati con Supabase.
  - Gestione dei conflitti deterministica (**Last-Write-Wins**) e supporto completo per la cancellazione logica (**Soft Delete**).
  - Configurazione interattiva delle credenziali (URL e Anon Key) direttamente dall'interfaccia utente.
- 📋 **Sidebar Globali ad Accesso Rapido**:
  - **Sinistra (Sessioni)**: Cronologia delle sessioni completate/interrotte e form per l'aggiunta manuale rapida.
  - **Destra (Task)**: Checklist di cose da fare con feedback di completamento immediato.
  - Le barre laterali sono disponibili in sovrimpressione su qualsiasi pagina dell'applicazione.
- 📊 **Statistiche Avanzate**: Pannello grafico con riepiloghi dettagliati del tempo di studio, tassi di completamento delle sessioni, conteggi delle task aggiunte e completate, corredato da grafici storici a barre.
- 📥 **Modalità System Tray**: Riducendo ad icona l'applicazione, questa si sposta nella barra dei menu di sistema, continuando a far scorrere i timer e a gestire i media in background.

---

## 🛠️ Tech Stack & Architettura

L'applicazione sfrutta un'architettura ibrida per unire la sicurezza ed efficienza del codice nativo alla flessibilità delle moderne UI web:

- **Desktop Runtime**: [Tauri v2](https://tauri.app/) (motore leggero e performante in Rust)
- **Frontend**: [Vue 3](https://vuejs.org/) + [TypeScript](https://www.typescriptlang.org/)
- **Gestione Stato**: [Pinia](https://pinia.vuejs.org/)
- **UI & Stile**: [Tailwind CSS v4](https://tailwindcss.com/) + [shadcn-vue](https://www.shadcn-vue.com/)
- **Database Locale**: [SQLite](https://www.sqlite.org/) interfacciato tramite query asincrone compilate con [SQLx](https://github.com/launchbadge/sqlx) in Rust
- **Database Cloud**: [Supabase](https://supabase.com/) con politiche Row Level Security (RLS) attive
- **OS API**: Crate Rust `windows` per l'aggancio diretto alle API GSMTC di Windows

---

## 🚀 Guida all'Installazione ed Esecuzione

### Prerequisiti
Assicurati di avere installato sul sistema:
- [Node.js](https://nodejs.org/) (versione 18+)
- [pnpm](https://pnpm.io/)
- [Rust Toolchain](https://www.rust-lang.org/tools/install) (cargo e rustc)

### Avvio in Sviluppo
Installa le dipendenze frontend ed avvia l'ambiente Tauri dev:
```bash
# Installa le dipendenze
pnpm install

# Avvia l'applicazione in modalità sviluppo (Hot Reloading frontend e backend)
pnpm tauri dev
```

### Compilazione Build di Produzione
Per generare l'eseguibile ottimizzato `.exe` per Windows:
```bash
pnpm tauri build
```

---

## 📚 Documentazione Tecnica di Dettaglio

Per comprendere approfonditamente le scelte architetturali e i protocolli implementati nell'applicazione, consulta la documentazione ufficiale:
- 📖 [Motore di Sincronizzazione Supabase (Offline-First)](docs/SUPABASE_SYNC.md)
- 📖 [Architettura ad Eventi Estensibili (Tracking System)](docs/TRACKING_ARCHITECTURE.md)
