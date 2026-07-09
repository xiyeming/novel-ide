# Novel IDE

Professional novel writing IDE built with Tauri 2 + Vue 3 + SQLite.

## Quick Start

```bash
# Frontend dev server (port 5273)
npm run dev

# Full desktop app (frontend + Rust backend)
npm run tauri dev

# Build for production
npm run build
```

## Commands

| Command | Purpose |
|---------|---------|
| `npm run dev` | Start Vite dev server only |
| `npm run tauri dev` | Start full desktop app |
| `npm run build` | Type-check + build frontend |
| `npm run type-check` | TypeScript check only |
| `npm run lint` | ESLint check |
| `npm run test` | Run Vitest in watch mode |
| `npm run test:run` | Run Vitest once |
| `cargo test` | Run Rust tests |
| `cargo check` | Rust type check |

## Architecture

### Frontend (src/)
- `components/layout/` — IDE layout (ActivityBar, Sidebar, StatusBar, etc.)
- `components/project/` — Project management (ProjectList, NewProject, EditProject)
- `components/ai/` — AI Studio panel
- `stores/` — Pinia stores (project, chapter, ai, workspace, shortcuts)
- `styles/` — CSS variables and component styles
- `composables/` — Vue composables (useTauriIPC)

### Backend (src-tauri/)
- `src/commands/` — Tauri command handlers
- `src/db/` — SQLite database (SQLx)
- `src/models/` — Data models
- `src/services/` — Business logic
- `src/error.rs` — Error types

### Key Facts
- Tauri 2.11 with plugins: dialog, fs, shell, store, global-shortcut
- SQLite via SQLx 0.9 (runtime-tokio)
- Vue 3.5 + Pinia 3 + Monaco Editor
- Vite dev server runs on port 5273
- Rust edition 2024, rust-version 1.90

## Conventions

- **Language**: All UI text must be in Chinese (永中文). Component names in English.
- **CSS**: Use design tokens from `src/styles/variables.css`. Dark theme by default.
- **IPC**: Tauri IPC auto-converts snake_case (Rust) to camelCase (JS). Send camelCase from frontend.
- **Database**: SQLite migrations in `src-tauri/migrations/`
- **Components**: Use `<script setup lang="ts">` with Vue 3 Composition API

## Testing

- Frontend: Vitest with jsdom environment
- Backend: Rust tests with `cargo test`
- Tests live in `src/test/` (frontend) and inline in Rust files

## Gotchas

- `window.__TAURI__` is unreliable for detecting Tauri environment. Use `import("@tauri-apps/plugin-dialog")` with try/catch instead.
- Number input native controls don't respect CSS. Use custom NumberInput component.
- Native `<select>` dropdowns don't respect dark theme. Use custom CustomSelect component.
- Tauri dialog permissions must be declared in `src-tauri/capabilities/desktop.json`
