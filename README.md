# Study Timer

A calm dark-themed desktop study dashboard built with Tauri 2, Vue 3, TailwindCSS and shadcn-vue.

## Features

- **Aesthetic Focus Timer**: Circular progress, customizable intervals, and custom durations with a relaxing dark/red pulsing glow during execution.
- **Persistent Session Logging**: Automated recording of finished or interrupted study sessions to a local SQLite database.
- **Interactive Task List**: CRUD functionality for task tracking (completed items move to the bottom).
- **Current Clock & Local Date**: Glancable digital clock formatting.
- **Spotify Now Playing Widget**: Interactive mockup demonstrating future OAuth integration with music progress and play/pause controls.
- **Local SQLite Persistence**: Powered by Rust, `sqlx` migrations, and local application data path storage.

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

- **Desktop Framework**: Tauri 2 + Rust
- **Frontend Framework**: Vue 3 + TypeScript
- **State Management**: Pinia
- **CSS Utility**: TailwindCSS v4
- **UI Components**: shadcn-vue
- **Database**: SQLite via `sqlx` (Rust backend)
