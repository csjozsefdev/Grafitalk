# GrafiTalk — Developer guide

This guide explains how GrafiTalk is built, how data flows through the app, and how to work on it safely.

For milestone status and IPC inventory, see [Project status](PROJECT_STATUS.md), [Progress](PROGRESS.md), and [Core status](CORE_STATUS.md).

---

## Product boundary (Core RC)

GrafiTalk is a **local-first communication preparation tool**:

```text
Context  →  Template  →  Draft  →  Review  →  Copy / Export
```

**GrafiTalk prepares; humans send.**

The Core Release Candidate ships:

- Project create / list / select / **rename** / **remove (archive)** / **restore** via sidebar `⋯` menu
- Manual `context_text` per project
- **Graf-ID import** (JSON, TXT, MD)
- **Five deterministic templates** (no AI)
- Editable draft with autosave
- Copy to clipboard
- **Draft export** (TXT, MD, JSON, PDF)
- **Dynamic workflow indicator**
- **Template-change confirmation** when manual draft edits would be overwritten
- **Read-aloud** for Current Preview (local Web Speech API when supported)
- **Settings workspace** (Grafi, read-aloud, export placeholder, About / Diagnostics)
- **Diagnostics + database backup** (Settings → About / Diagnostics)
- **Custom title bar** with window controls (Windows)
- **GrafiTalk startup splash** (light theme, `GrafiTalkSplash` wrapper)

Still out of scope: AI, auto-send, cloud sync, CRM, unlimited draft history, advanced settings (custom export defaults, accounts), permanent delete, code signing (org-dependent).

---

## Technology stack

| Layer | Technology |
|-------|------------|
| **Frontend** | React 19, TypeScript, Vite |
| **Desktop shell** | Tauri 2 |
| **Backend** | Rust (`backend` crate) |
| **Storage** | SQLite (`grafitalk.db` in app data dir) |

---

## Repository layout

```text
Grafitalk/
├── backend/              Rust library — business logic and SQL
│   ├── migrations/       SQL schema versions (001–005)
│   └── src/
│       ├── db/           Connection, migrations, errors
│       ├── model/        Domain structs (Project, AppDiagnostics)
│       ├── repo/         SQL queries only
│       ├── service/      Validation, orchestration
│       ├── import/       Graf-ID handoff import pipeline
│       ├── export/       Draft export formats (TXT/MD/JSON/PDF)
│       ├── template/     Deterministic draft templates
│       └── hardening/    Integration test matrix
├── src-tauri/            Tauri 2 desktop shell
│   └── src/
│       ├── commands/     Thin IPC handlers (one file per domain)
│       ├── lib.rs        App setup + invoke registration
│       └── state.rs      AppState (service, db path, version)
├── frontend/             React + TypeScript UI
│   └── src/
│       ├── api/          invoke() wrappers
│       ├── components/   Workbench UI + grafi-splash/
│       ├── hooks/        useReadAloud, useStartupSplash, …
│       ├── styles/       tokens.css, app.css, ambient.css
│       ├── types/        Project, template, preferences
│       └── utils/        Workflow, dirty state, formatting
├── scripts/              release.ps1
├── docs/                 Documentation
├── examples/             Sample Graf-ID handoff files
└── Cargo.toml            Rust workspace (backend + src-tauri)
```

There is **no root `package.json`**. npm lives in `frontend/package.json`.

---

## Architecture

### Layer rules

```text
React UI
    │  invoke('command_name', { ... })
    ▼
src-tauri/commands/*.rs     ← parse args, call service, map errors to String
    ▼
backend::service            ← validation, IDs, timestamps, template calls
    ▼
backend::repo               ← SQL only
    ▼
SQLite (grafitalk.db)
```

| Layer | May do | Must not do |
|-------|--------|-------------|
| `frontend` | Display, user input, `invoke` | SQL, UUID generation, business rules |
| `src-tauri/commands` | Delegate to service | SQL, validation logic |
| `service` | Orchestration, rules | Raw SQL strings |
| `repo` | INSERT / SELECT / UPDATE | Name-empty checks, template logic |
| `template` | Pure string rendering | Database access |

### Startup boundary

```text
App.tsx
  ├── useStartupSplash()        ← splash timing only
  ├── AppShell                  ← loads projects; calls onStartupReady when done
  └── GrafiTalkSplash (overlay) ← fixed z-index; fades out after min duration
```

Splash logic is **not** mixed into `AppShell` business code. `AppShell` exposes optional `onStartupReady` when initial `list_projects` completes.

### Database location

On startup, Tauri opens:

```text
{app_data_dir}/grafitalk.db
```

`Database::open` creates parent directories and runs pending migrations idempotently.

### Schema (projects table)

After migration 005, each project row holds:

| Column | Purpose |
|--------|---------|
| `id` | UUID primary key |
| `name` | Required display name |
| `client_label` | Optional (API supports it; sidebar create uses name only) |
| `status` | `active` or `archived` |
| `context_text` | Manual context notes |
| `draft_text` | Current draft body |
| `last_template_kind` | Last selected template |
| `last_generated_at`, `last_exported_at`, `last_export_format` | Review metadata |
| `created_at`, `updated_at` | ISO 8601 timestamps |
| `last_used_at` | Updated when user selects project |
| `archived_at` | Set when project is archived |

---

## IPC commands (23 total)

Registered in `src-tauri/src/lib.rs`:

| Domain | Commands |
|--------|----------|
| **Projects** | `list_projects`, `create_project`, `touch_project_used`, `archive_project`, `list_archived_projects`, `restore_project`, `rename_project` |
| **Context** | `get_project_context`, `save_project_context` |
| **Draft** | `get_project_draft`, `save_project_draft`, `generate_draft`, `generate_status_draft`, `get_project_template_kind`, `save_project_template_kind` |
| **Import** | `import_graf_id_handoff` |
| **Export** | `prepare_draft_export` |
| **Review** | `get_project_review_metadata`, `migrate_last_generated_at_if_empty`, `record_draft_export` |
| **App** | `get_app_diagnostics`, `export_database_backup`, `open_data_folder` |

`generate_status_draft` remains a compatibility alias for Status Update.

Full detail: [Architecture](ARCHITECTURE.md#tauri-commands).

---

## Pipelines

### Import pipeline (Graf-ID)

```text
User picks file (frontend)
    → import_graf_id_handoff IPC
    → backend/import (format detection, validation)
    → PrettyPrintContext → canonical context_text
    → save to projects.context_text
```

Failure does **not** overwrite existing context.

### Export pipeline

```text
User clicks Export (frontend)
    → prepare_draft_export IPC (builds file content in Rust)
    → Tauri save dialog (frontend)
    → record_draft_export IPC (metadata only, on success)
```

Failure does **not** mutate draft or export metadata.

### Template / generation pipeline

```text
context_text
    → parse_pretty_print (pretty_print.rs)
    → optional note_transform (compact notes)
    → PrettyPrintContext
    → render_draft(TemplateKind)
    → draft_text persisted
```

| Module | Role |
|--------|------|
| `pretty_print.rs` | Parse raw context → structured fields |
| `note_transform.rs` | Compact deployment/waiting notes → structured fields |
| `render.rs` | Template dispatch |
| `templates/` | Five template renderers |
| `graf_id_handoff.rs` | Graf-ID JSON + reference TXT/MD serializers |

Entry points: `render_draft`, `render_draft_from_context_text`, `render_status_update` (alias).

Generation requires non-empty context.

---

## Frontend structure

| Component / module | Role |
|------------------|------|
| `App.tsx` | Startup splash orchestration |
| `AppShell` | Workflow orchestration, autosave, Grafi, diagnostics |
| `ProjectSidebar` | List, create, rename, archive, restore |
| `DocumentPanel` | Draft textarea + read-aloud control |
| `ContextPanel` | Context textarea + Graf-ID import |
| `ActionBar` | Template selector, Generate, Copy, Export |
| `TitleBar` | Brand, dynamic workflow pill, window controls |
| `GrafiTalkAdvisor` | Bottom-left contextual guidance (portal host) |
| `SettingsView` | Settings workspace: Grafi, read-aloud, export placeholder, About / Diagnostics |
| `ConfirmDialog` / `NamePromptDialog` | App-styled lifecycle prompts |
| `grafi-splash/GrafiTalkSplash` | Light startup overlay |

### Auto-save behavior

Context and draft edits debounce at **500 ms**. `AppShell` flushes pending saves when:

- The user switches projects
- The component unmounts

Generate Draft persists via the backend (`generate_draft` writes to `draft_text`).

---

## Development setup

### Prerequisites

- Rust toolchain
- Node.js (for frontend)
- Windows (primary target for Core RC)

### Run the desktop app

```powershell
cd C:\Projektek\Grafitalk
npx @tauri-apps/cli dev
```

Tauri config (`src-tauri/tauri.conf.json`) runs `npm run dev --prefix ../frontend` before the window opens.

### Frontend only (no IPC)

```powershell
cd frontend
npm run dev
```

The UI loads but backend calls will fail without Tauri.

### Build and test

```powershell
cargo test -p backend
cargo check -p grafitalk
cd frontend
npm run build
npm run test
npm run lint
```

### Release build (Windows)

```powershell
.\scripts\release.ps1
```

Artifacts: `target/release/bundle/` (MSI and NSIS installers)

Validation: [RC_1_0_RELEASE_VALIDATION.md](RC_1_0_RELEASE_VALIDATION.md) · Backup: [BACKUP_RESTORE.md](BACKUP_RESTORE.md)

---

## Adding a feature (checklist)

1. **Decide the layer** — business rule? → `service`. SQL? → `repo`. UI only? → `frontend`.
2. **Schema change?** — add `00N_description.sql` in `backend/migrations/`. Never edit old migrations.
3. **Backend first** — model/repo/service + tests.
4. **Thin command** — one `#[tauri::command]` in `src-tauri/src/commands/`.
5. **Register** — add to `lib.rs` invoke handler.
6. **Frontend API** — small `invoke` wrapper in `frontend/src/api/`.
7. **Wire UI** — minimal change in existing components; avoid shell rewrites.

---

## Testing philosophy

- **Backend:** real temp SQLite files per test via `Database::open`; hardening matrix in `backend/src/hardening/matrix.rs`.
- **Frontend:** Vitest unit/component tests (`npm run test`) — workflow, dialogs, splash, Grafi, read-aloud.
- **Manual QA:** [QA checklist](QA_CHECKLIST.md) via `npx @tauri-apps/cli dev`.
- **No Tauri E2E** in CI yet — IPC wiring verified manually.

Expected counts (RC): 134 backend tests, 51 frontend tests.

---

## Common pitfalls

| Problem | Cause | Fix |
|---------|-------|-----|
| Build reads empty `Cargo.toml` | File not saved to disk | Save before building |
| `npx tauri dev` fails from root | Wrong package | Use `npx @tauri-apps/cli dev` |
| Dev command runs in wrong folder | cwd is `src-tauri/` | `--prefix ../frontend` in config |
| Rust `#` comments | Habit from Python/SQL | Use `//` |
| Business logic in React | Shortcut | Move to `backend/service` |
| Splash timing in AppShell | Coupling | Keep in `useStartupSplash` + `App.tsx` |

---

## Post-RC ideas (not implemented)

See [Core status](CORE_STATUS.md):

- Permanent project delete
- Windows code signing pipeline
- AppShell decomposition / integration tests
- AI / Mentor Mode (separate milestone)

Keep the **thin IPC → service → repo** pattern when extending.
