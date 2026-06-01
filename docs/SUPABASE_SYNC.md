# Supabase Synchronization Engine

Questo documento illustra il funzionamento del motore di sincronizzazione cloud tra il database SQLite locale dell'app e la piattaforma cloud Supabase.

## Filosofia Progettuale: Offline-First

StudyTimer è un'applicazione **offline-first**. Tutte le letture e le scritture avvengono istantaneamente sul database SQLite locale dell'utente. Il processo di sincronizzazione avviene in modo asincrono, evitando di bloccare o rallentare l'esperienza d'uso dell'utente in caso di assenza di rete o problemi di connessione col cloud.

---

## Sicurezza e Credenziali

> [!CAUTION]
> **Non utilizzare mai la chiave `service_role` o secret keys all'interno dell'applicazione desktop.**
> Trattandosi di un'app compilata ed eseguita sul client dell'utente, qualsiasi codice o risorsa contenuta nel pacchetto finale può essere estratta e ispezionata. L'esposizione di una chiave di servizio darebbe controllo completo sul database a terzi.

### Linee Guida di Sicurezza:
1. Usare esclusivamente la **Anon Public Key** (`VITE_SUPABASE_ANON_KEY`) e l'URL del progetto (`VITE_SUPABASE_URL`).
2. Configurare e abilitare le politiche di sicurezza **RLS (Row Level Security)** su tutte le tabelle cloud di Supabase.
3. Filtrare l'accesso ai dati basandosi sull'ID dell'utente autenticato (`auth.uid() = user_id`).

---

## Configurazione di Supabase

Per attivare il cloud sync, segui questi passaggi:

1. Crea un progetto su [Supabase](https://supabase.com/).
2. Esegui lo script SQL contenuto in [supabase_schema.sql](file:///d:/Projects/StudyTimer/docs/supabase_schema.sql) nel SQL Editor del pannello di Supabase.
3. Copia il file `.env.example` in `.env.local` nella cartella root del progetto:
   ```bash
   cp .env.example .env.local
   ```
4. Inserisci l'URL del progetto e la Anon Key ricavati dalle impostazioni API del pannello di Supabase (`Settings` -> `API`).

---

## Algoritmo di Sincronizzazione (Bidirezionale)

Il flusso di sincronizzazione si articola in tre fasi distinte coordinate da `sync.store.ts`:

```text
       [ SQLite Locale ]                         [ Supabase Cloud ]
               │                                         │
 1. Carica eventi non sincronizzati                      │
    (sync_status != 'synced')                            │
               ├────────────────────────────────────────>│  (Upsert remoto)
               │                                         │
 2. Aggiorna stato locale degli eventi caricati           │
    (status = 'synced', synced_at = now)                 │
               │                                         │
 3. Scarica modifiche remote                             │
    (updated_at > ultimo_sync_locale)                     │
               │<────────────────────────────────────────┤  (Download delta)
               │                                         │
 4. Salva modifiche remote in SQLite                     │
    (upsert locale con status='synced')                   │
               │                                         │
```

### Fase 1: Caricamento Dati Locali (Upload)
1. L'app interroga SQLite tramite il comando `get_unsynced_events` per ottenere tutti gli eventi con `sync_status != 'synced'` (compresi i soft-delete `deleted_at IS NOT NULL`).
2. Per ciascun evento non sincronizzato:
   - Viene parsato l'eventuale campo `metadata_json` (stringa in SQLite) in un oggetto JSON per la colonna `jsonb` su Supabase.
   - Si invia un comando `upsert` a Supabase fornendo l'ID dell'evento e l'ID utente autenticato.
   - Se l'upload ha successo, l'app marca localmente l'evento come `synced` e imposta il timestamp `synced_at` a now.
   - Se l'upload fallisce (conflitto o errore), viene contrassegnato localmente come `conflict` o `error`.

### Fase 2: Download Dati Remoti (Pull)
1. L'app interroga Supabase scaricando tutti gli eventi modificati dopo l'ultimo timestamp di sincronizzazione memorizzato nel client (`localStorage.getItem('study_timer_last_synced_at')`).
2. Per ogni evento remoto ricevuto:
   - I dati vengono convertiti da `snake_case` (schema Supabase) a `camelCase` (schema locale).
   - Viene chiamato il comando Rust `upsert_synced_event` che scrive o aggiorna l'evento nel DB locale con `sync_status = 'synced'`.

### Fase 3: Conclusione
1. Viene aggiornato e persistito il timestamp locale `lastSyncedAt` per le future query di delta.

---

## Gestione dei Conflitti (MVP)

Per mantenere il sistema rapido ed efficiente, la risoluzione dei conflitti segue la regola **Last-Write-Wins** basandosi sul campo `updated_at`:
- SQLite e Supabase memorizzano data e ora di modifica di ciascun evento.
- In fase di sincronizzazione, l'evento con il timestamp `updated_at` più recente sovrascrive quello precedente.
- Le sigarette eliminate o le sessioni rimosse utilizzano la cancellazione logica (**Soft Delete**) inserendo la data corrente in `deleted_at`. Questo assicura che lo stato rimosso possa propagarsi correttamente tra tutti i dispositivi associati all'account.
