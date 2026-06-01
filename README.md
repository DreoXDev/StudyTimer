# Study Timer

An immersive, calm Focus Home desktop application built with Tauri 2, Vue 3, TailwindCSS, and shadcn-vue. Designed to serve as a secondary screen companion during study sessions, featuring a dark minimalist aesthetic with glassmorphic elements and soft red accents.

## Features

- **Immersive Focus Home**: Clean layout containing only essential info when studying to minimize distractions.
- **Large Central Timer**: A cardless, minimalist central timer with support for manual minute editing (click the digits to edit).
- **Smoking Tracker Dropdown**: A discrete title bar widget for tracking today's cigarette counts, offering manual `+` and `-` actions, along with healthy reminders ("Il fumo uccide").
- **Minimize-to-Tray Mode**: Intercepts standard close commands to hide the application in the system tray, keeping background timers and active widgets alive. Exit is cleanly accessed through the tray menu.
- **Dedicated Stats Dashboard**: A second view featuring aggregated Metrics cards, visual CSS charts for study time and fumo tracking, and an interactive raw log timeline with delete functionality.
- **Local Exporter**: Allows exporting tracking logs to CSV, JSON, or formatted Obsidian Markdown.
- **Offline-First Cloud Sync (Supabase)**: Connects SQLite storage with Supabase cloud databases using user authentication and Row Level Security (RLS) policies without exposing private backend credentials.
- **Collapsible Sidebars**:
  - **Left Sidebar**: Displays study session history, daily/weekly stats, and a form to manually log study sessions done outside the app.
  - **Right Sidebar**: A checklist for tasks that acts as a quick-glance panel.
- **Custom App Titlebar**: Replaces native OS window decorations with a custom borderless design supporting window dragging, minimization, maximization, and closing.
- **Native System Now Playing**:
  - Integrates with the **Windows Global System Media Transport Controls (GSMTC)**.
  - Automatically fetches metadata (title, artist, app source, play state, progress, and duration) from active system audio sessions (e.g. Spotify Desktop, Chrome/YouTube, Edge, VLC).
  - Allows direct play/pause, next, and previous actions on the active media session.
  - **No OAuth Required**: Works out of the box without requiring API keys or Spotify login.
  - **Cross-Platform Mock Fallback**: Automatically falls back to an interactive mock track on non-Windows platforms.
- **SQLite Persistence**: Rust-powered backend running migration-managed local SQLite storage in the user's AppData directory.

## Development

Prerequisites:
- [Node.js](https://nodejs.org/) & [pnpm](https://pnpm.io/)
- [Rust toolchain](https://www.rust-lang.org/tools/install) (cargo/rustc)

Start in development mode:
```bash
pnpm install
pnpm tauri dev
```

## Build

To compile a production-ready Windows desktop app:
```bash
pnpm tauri build
```

## Tech Stack

- **Desktop Framework**: Tauri 2 + Rust (utilizing the `windows` crate for native GSMTC integration)
- **Frontend Framework**: Vue 3 + TypeScript
- **State Management**: Pinia
- **CSS Utility**: TailwindCSS v4
- **UI Components**: shadcn-vue
- **Database**: SQLite via `sqlx` (Rust backend)

