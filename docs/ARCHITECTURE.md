# GrafiTalk — Architecture



High-level view of the **GrafiTalk Core RC** desktop app: data flow, persistence, generation, import/export, and startup.



---



## System overview



```text

┌─────────────────────────────────────────────────────────────┐

│  React frontend (frontend/)                                  │

│  AppShell · ProjectSidebar · DocumentPanel · ContextPanel    │

└───────────────────────────┬─────────────────────────────────┘

                            │ Tauri invoke (camelCase payloads)

┌───────────────────────────▼─────────────────────────────────┐

│  Tauri shell (src-tauri/)                                    │

│  Thin #[tauri::command] handlers                            │

└───────────────────────────┬─────────────────────────────────┘

                            │

┌───────────────────────────▼─────────────────────────────────┐

│  Rust backend library (backend/)                             │

│  service → repo → SQLite                                     │

│  import · export · template (deterministic draft generation) │

└───────────────────────────┬─────────────────────────────────┘

                            │

                     {app_data_dir}/grafitalk.db

```



---



## Core generation pipeline



All draft generation uses one path:



```text

Raw context_text

    │

    ▼

parse_pretty_print (pretty_print.rs)

    │  optional compact-note fill (note_transform.rs)

    ▼

PrettyPrintContext

    │

    ▼

render_draft(TemplateKind) (render.rs + templates/)

    │

    ▼

Draft text → draft_text column

```



Import converges on the same model:



```text

Graf-ID file (JSON / TXT / MD)

    │

    ▼

parse_import (import/parser.rs)

    │

    ▼

PrettyPrintContext → canonical context text

    │

    ▼

context_text column

```



---



## Frontend components



| Component | Role |

|-----------|------|

| `AppShell` | Orchestrates projects, autosave, generate, copy, export |

| `ProjectSidebar` | Create/select projects, show last-used time |

| `DocumentPanel` | Current Preview — draft textarea, review metadata row |

| `ContextPanel` | Manual context editor + Import button |

| `ActionBar` | Template selector, Generate Draft, Copy, Export |



### API wrappers (`frontend/src/api/`)



- `projects.ts` — list, create, touch last used

- `context.ts` — get/save context

- `draft.ts` — get/save draft, generate, template kind

- `import.ts` — Graf-ID handoff import

- `export.ts` — prepare draft export

- `review.ts` — review metadata read/write



---



## Tauri commands



IPC commands return `Result<T, String>` and delegate to `ProjectService`.



| Command | Purpose |

|---------|---------|

| `list_projects` | Active projects for sidebar |
| `create_project` | Create project (name required) |
| `touch_project_used` | Update `last_used_at` on select |
| `archive_project` | Hide project from active list |
| `list_archived_projects` | Archived projects for restore UI |
| `restore_project` | Return archived project to active list |
| `rename_project` | Update project display name |
| `get_project_context` / `save_project_context` | Context text |

| `get_project_draft` / `save_project_draft` | Latest draft |

| `generate_status_draft` | **Compatibility alias** — Status Update template |

| `generate_draft` | Generate with selected `TemplateKind` |

| `get_project_template_kind` / `save_project_template_kind` | Per-project template |

| `import_graf_id_handoff` | Import Graf-ID JSON/TXT/MD |

| `prepare_draft_export` | Build export bytes (TXT/MD/JSON/PDF) |

| `get_project_review_metadata` | Last generated/exported/template |

| `record_draft_export` | Persist export metadata after successful save |

| `migrate_last_generated_at_if_empty` | One-time localStorage → SQLite migration |
| `get_app_diagnostics` | Version, DB path, migration info |
| `export_database_backup` | Copy `grafitalk.db` to user-chosen path |
| `open_data_folder` | Return app data directory path |

**Total:** 23 registered commands in `src-tauri/src/lib.rs`.

Handlers live in `src-tauri/src/commands/`.



---



## Local persistence



### Database



- Path: `{app_data_dir}/grafitalk.db`

- Engine: SQLite via `rusqlite` (bundled)

- Migrations: `backend/migrations/001–005`, applied idempotently on open



### Per-project columns (selected)



| Column | Purpose |

|--------|---------|

| `context_text` | Manual or imported context |

| `draft_text` | Latest draft only |

| `last_template_kind` | Last selected template |

| `last_generated_at` | ISO timestamp after generate |

| `last_exported_at` | ISO timestamp after successful export |

| `last_export_format` | Last export format label |

| `last_used_at` | Sidebar ordering |



---



## Draft generation flow



```text

User clicks Generate Draft

    │

    ▼

AppShell flushes pending context autosave

    │

    ▼

invoke('generate_draft', { projectId, templateKind })

    │  (generate_status_draft → same pipeline, Status Update kind)

    ▼

ProjectService::generate_draft

    ├── load project + context (must be non-empty)

    ├── render_draft_from_context_text(kind, project, context)

    ├── save draft_text + last_generated_at + last_template_kind

    └── return draft body

```



Generation is **deterministic**: same project + context + template → same output.



### Template modules (`backend/src/template/`)



| Module | Purpose |

|--------|---------|

| `pretty_print.rs` | Parse raw context → `PrettyPrintContext` |

| `note_transform.rs` | Compact deployment/waiting note → structured fields |

| `render.rs` | Template dispatch |

| `templates/` | Five template renderers |

| `graf_id_handoff.rs` | Graf-ID JSON + reference TXT/MD serializers |

| `status_update.rs` | Status Update entry (uses shared pipeline) |



Legacy `context_parser.rs` was removed in PR 8.



---



## Export flow



```text

User clicks Export → save dialog

    │

    ▼

prepare_draft_export(projectId, format, previewText)

    │

    ▼

User saves file via Tauri fs plugin

    │

    ▼

record_draft_export(projectId, format)  (on success only)

```



Export failure does not modify stored draft or export metadata.



---



## Project switching flow



```text

User selects different project

    │

    ▼

flushPendingSaves() — context + draft for previous project

    │

    ▼

Parallel load: context, draft, template kind, review metadata

    │

    ▼

Load generation guard — ignore stale responses if user switched again

```



---



## Autosave behavior



| Data | Debounce | Flush triggers |

|------|----------|----------------|

| Context | 500 ms | Switch, Generate, Import, unmount |

| Draft | 500 ms | Switch, unmount |

| Generate | Immediate | Backend persists draft |



One draft per project — saves overwrite `draft_text`.



---



## Graf-ID integration



GrafiTalk owns the export contract and import path. Graf-ID implements export in its own repo.



1. Graf-ID exports JSON, TXT, or MD per [GRAF_ID_EXPORT_SCHEMA.md](GRAF_ID_EXPORT_SCHEMA.md)

2. User imports file in GrafiTalk Context panel

3. Context stored as canonical labeled text

4. Normal Generate → Review → Copy / Export flow



Reference serializers: `handoff_to_plain_text`, `handoff_to_markdown` in `graf_id_handoff.rs`.



---



## Layer rules



| Layer | May do | Must not do |

|-------|--------|-------------|

| `frontend` | UI, invoke, debounce | SQL, business rules |

| `src-tauri/commands` | Delegate | SQL, validation |

| `service` | Orchestration, rules | Raw SQL |

| `repo` | SQL only | Template logic |

| `template` / `import` / `export` | Pure transforms | Database access |



See [Developer guide](DEVELOPER_GUIDE.md) for extension checklist.


