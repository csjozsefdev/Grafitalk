# GrafiTalk — Progress



Local-first **Context → Template → Generate Draft → Review → Copy / Export** tool. Not AI, not CRM, not auto-send.



**Status:** **GrafiTalk Core RC** (see [Core status](CORE_STATUS.md)).



---



## Milestone status



| Milestone | Status | Scope |

|-----------|--------|-------|

| **B.1** | Done | Cargo workspace, `backend` module tree |

| **B.2** | Done | Tauri 2 + React workbench shell |

| **M1** | Done | SQLite open, idempotent migrations |

| **M2** | Done | Project create/list, IPC, sidebar |

| **M3** | Done | Context per project |

| **M4** | Done | Status Update template |

| **M5** | Done | Draft persistence, review/edit/copy |

| **M5.1** | Done | Core hardening (flush, isolation) |

| **M5.2** | Done | MVP documentation + Graf-ID handoff spec |

| **P1** | Done | Pretty Print foundation (PR 1) |

| **P2** | Done | Graf-ID export contract + serializers (PR 2) |

| **P3** | Done | Graf-ID import UI + IPC (PR 3) |

| **P4** | Done | Five templates + selector (PR 4) |

| **P5** | Done | Draft export TXT/MD/JSON/PDF (PR 5) |

| **P6** | Done | Review metadata in SQLite (PR 6) |

| **P7** | Done | Hardening test matrix + QA checklist (PR 7) |

| **P8** | Done | Legacy parser cleanup (PR 8) |

| **P9** | Done | Documentation sync + release gate (PR 9) |

| **RC1** | Done | Dynamic workflow indicator |
| **RC2** | Done | Archive project UI + IPC |
| **RC3** | Done | Template dirty confirm |
| **RC4** | Done | Grafi advisor + read-aloud |
| **RC5** | Done | Custom title bar + visual polish |
| **RC6** | Done | Diagnostics + DB backup |
| **RC7** | Done | Release script + frontend Vitest |
| **RC8** | Done | Documentation sync + POST_RC_AUDIT procedure |
| **RC9** | Done | GrafiTalk startup splash (light theme) |
| **RC10** | Done | Project rename + archive restore + app-styled dialogs |
| **RC11** | Done | RC cleanup + documentation pass |

---



## Architecture



```text

frontend  --invoke-->  src-tauri/commands  -->  backend::service  -->  repo  -->  SQLite

                              │                      │

                              │                      ├── import/

                              │                      ├── export/

                              │                      └── template/ (Pretty Print → render)

```



Details: [Architecture](ARCHITECTURE.md)



---



## Migrations



| Version | File | Adds |

|---------|------|------|

| 1 | `001_projects.sql` | `schema_migrations`, `projects` |

| 2 | `002_context_text.sql` | `context_text` |

| 3 | `003_draft_text.sql` | `draft_text` |

| 4 | `004_template_kind.sql` | `last_template_kind` |

| 5 | `005_review_metadata.sql` | `last_generated_at`, `last_exported_at`, `last_export_format` |



---



## IPC commands (22)

| Command | Purpose |
|---------|---------|
| `list_projects` | Active projects |
| `list_archived_projects` | Archived projects |
| `create_project` | Create project |
| `rename_project` | Rename active project |
| `archive_project` | Soft-hide project |
| `restore_project` | Unarchive project |
| `touch_project_used` | Update `last_used_at` |
| `get_project_context` / `save_project_context` | Context |
| `get_project_draft` / `save_project_draft` | Draft |
| `generate_status_draft` | Status Update alias |
| `generate_draft` | Generate with template kind |
| `get_project_template_kind` / `save_project_template_kind` | Template persistence |
| `import_graf_id_handoff` | Graf-ID import |
| `prepare_draft_export` | Export bytes |
| `get_project_review_metadata` | Review metadata read |
| `record_draft_export` | Export metadata write |
| `migrate_last_generated_at_if_empty` | localStorage migration |
| `get_app_diagnostics` | App + DB diagnostics |
| `export_database_backup` | Copy `grafitalk.db` |
| `open_data_folder` | Reveal data directory |



---



## Pro Pre-AI feature checklist



- [x] Pretty Print on all generation paths

- [x] Graf-ID contract v0.1 + samples + reference serializers

- [x] Import JSON / TXT / MD

- [x] Five deterministic templates

- [x] Export TXT / MD / JSON / PDF

- [x] Review metadata in SQLite

- [x] Project isolation + DB migration hardening matrix

- [x] Legacy `context_parser` removed

- [x] Documentation synced

### Core RC 1.0 additions

- [x] Project rename
- [x] Archived project view + restore
- [x] App-styled confirm dialogs
- [x] Read-aloud hardening
- [x] Frontend regression tests (Vitest + RTL)
- [x] Backup/restore documentation
- [x] Release validation package (human QA)

**Out of Core RC 1.0 scope:** permanent delete, code signing, E2E CI, AI.



---



## Tests



```powershell
cargo test -p backend
cargo check -p grafitalk
npm run build --prefix frontend
npm run test --prefix frontend
npm run lint --prefix frontend
```

Release: `.\scripts\release.ps1`

Manual QA: [QA checklist](QA_CHECKLIST.md) · Post-RC audit: [POST_RC_AUDIT.md](POST_RC_AUDIT.md)



---



## Dev



```powershell

cd C:\Projektek\Grafitalk

npx @tauri-apps/cli dev

```



---



## Key paths



| Area | Path |

|------|------|

| Pretty Print | `backend/src/template/pretty_print.rs` |

| Templates | `backend/src/template/templates/` |

| Import | `backend/src/import/` |

| Export | `backend/src/export/` |

| Hardening matrix | `backend/src/hardening/matrix.rs` |

| Graf-ID contract | `docs/GRAF_ID_EXPORT_SCHEMA.md` |

| Samples | `examples/graf_id_project_context.sample.*` |


