# GrafiTalk — Core status



**Last updated:** Core Release Candidate  

**Statement:** **GrafiTalk Core RC** — Pro Pre-AI baseline plus lifecycle, workflow, Grafi, read-aloud, diagnostics, startup splash, and release packaging.



---



## Release gate (Pro Pre-AI)



| Check | Status |

|-------|--------|

| Pretty Print on every generation path | ✓ |

| Graf-ID contract v0.1 stable | ✓ [GRAF_ID_EXPORT_SCHEMA.md](GRAF_ID_EXPORT_SCHEMA.md) |

| Contract samples pass (reference serializers) | ✓ |

| Import JSON / TXT / MD | ✓ |

| All 5 templates | ✓ |

| Export TXT / MD / JSON / PDF | ✓ |

| Review metadata in SQLite | ✓ |

| Project isolation verified | ✓ hardening matrix |

| DB migration verified (001–005) | ✓ |

| Hardening matrix passes | ✓ `backend/src/hardening/matrix.rs` |

| Legacy `context_parser` removed | ✓ PR 8 |

| Docs match code | ✓ PR 9 |

---

## Core RC release gate

| Check | Status |
|-------|--------|
| Dynamic workflow indicator | ✓ `deriveWorkflowStep` |
| Archive project UI + IPC (Remove via `⋯` menu) | ✓ |
| Template dirty confirm | ✓ |
| Grafi advisor (local) | ✓ Upstream Grafi @ `23fd1c3` via body portal host |
| Read-aloud (Web Speech) | ✓ |
| Custom title bar (Windows) | ✓ |
| Diagnostics + DB backup | ✓ Settings → About / Diagnostics |
| `scripts/release.ps1` | ✓ |
| Project rename + archive restore | ✓ |
| GrafiTalk startup splash (light theme) | ✓ |
| Frontend unit tests | ✓ Vitest (51) |
| Post-RC audit procedure | [POST_RC_AUDIT.md](POST_RC_AUDIT.md) |
| Post-RC audit results | [RC_AUDIT_RESULTS.md](RC_AUDIT_RESULTS.md) — **CONDITIONAL GO** |

Run post-implementation sign-off per [POST_RC_AUDIT.md](POST_RC_AUDIT.md) before shipping the installer.

**Note:** Graf-ID production export is verified in the separate Graf-ID repo against contract v0.1. GrafiTalk proves contract + import compatibility.



---



## Pro Pre-AI baseline (PR 0)



| Check | Result |

|-------|--------|

| Regression fixtures | [backend/src/template/fixtures.rs](../backend/src/template/fixtures.rs) |

| Fixture tests | [backend/src/template/regression_fixtures.rs](../backend/src/template/regression_fixtures.rs) |

| Project isolation | Verified in service + hardening matrix |

| DB reopen persistence | Verified in service + hardening matrix |



### Parser history (resolved PR 1 / PR 8)



Before PR 1, Graf-ID-style labeled context collapsed into a single **Focus Area** block. PR 1 added [pretty_print.rs](../backend/src/template/pretty_print.rs); PR 8 removed the legacy `context_parser.rs` path.



---



## Completed features



### Workbench UI (accepted — do not redesign)



- Three-panel layout: Projects | Current Preview | Context

- Top workflow indicator: Context → Template → Draft → Review → Copy

- Communication Studio visual direction (light industrial graphite palette)

- Status badges and DocumentPanel review metadata row



### Projects

- Create, list, select, **rename**, **remove (archive)**, and **restore** active projects via sidebar `⋯` menu

- `last_used_at` updated on selection



### Context



- Manual context per project; debounced autosave (500 ms)

- Flush before project switch, Generate Draft, and Import

- **Graf-ID import** (JSON, TXT, MD) via Context panel Import button



### Draft



- Deterministic generation via **PrettyPrintContext** + **TemplateKind** registry

- Five templates: Status Update, Client Update, Debug Report, Handover, Weekly Summary

- Template selector in Action Bar; `last_template_kind` persisted per project

- Compact-note rules fill structured fields (no full-draft bypass)

- One latest draft per project; debounced draft autosave



### Current Preview



- Editable draft textarea with dynamic subject header

- Generate / Copy / Export (TXT, MD, JSON, PDF)



### Copy



- Clipboard copy with user feedback; disabled when preview empty



### Review metadata (PR 6)



- SQLite: `last_generated_at`, `last_exported_at`, `last_export_format`, `last_template_kind`

- IPC: `get_project_review_metadata`, `record_draft_export`, `migrate_last_generated_at_if_empty`

- localStorage migration fallback when DB empty



### Draft export (PR 5)



- `backend/src/export/` — TXT, MD, JSON, PDF

- IPC: `prepare_draft_export`, `record_draft_export`

- Schema: [DRAFT_EXPORT_SCHEMA.md](DRAFT_EXPORT_SCHEMA.md)



### Graf-ID import (PR 3)



- `backend/src/import/` — format detection, validation, parse → `PrettyPrintContext`

- IPC: `import_graf_id_handoff`

- Samples: `examples/graf_id_project_context.sample.{json,txt,md}`

- Contract: [GRAF_ID_EXPORT_SCHEMA.md](GRAF_ID_EXPORT_SCHEMA.md)



### Core RC UX

- Dynamic workflow pill (context → template → draft → review → copy)
- Grafi advisor via upstream Grafi (`createPortal` + `.gt-grafi-host`)
- Read-aloud via Web Speech API
- Custom title bar with window controls (`decorations: false`)
- **Settings workspace** (Grafi, read-aloud, export placeholder, About / Diagnostics)
- GrafiTalk startup splash (light theme; `GrafiTalkSplash` wrapper)
- Project rename, remove (archive), restore with app-styled dialogs

---

### Hardening (PR 7)



- Integration test matrix: import, templates, export, workflow, isolation, migration

- Updated [QA checklist](QA_CHECKLIST.md)



---



## Reliability status



| Area | Status |

|------|--------|

| Project switching isolation | Verified (tests + guards + matrix) |

| Context/draft persistence | Verified (SQLite + reopen tests) |

| Generate uses current context | Verified (flush before generate) |

| Import failure safety | Does not overwrite existing context |

| Export failure safety | Does not mutate draft or export metadata |

| Automated tests | 134 backend tests; 51 frontend Vitest tests |

Run manual QA: [QA checklist](QA_CHECKLIST.md). Release: `.\scripts\release.ps1`



---



## Known limitations



See [LIMITATIONS.md](LIMITATIONS.md). Highlights: no AI, no auto-send, no cloud sync, no draft history, no permanent delete.



---



## What NOT to touch before next milestone



- Panel layout proportions and three-column structure

- Copy-only outbound policy

- Workflow step labels and order

- Communication Studio color system (unless explicit design milestone)



---



## Recommended next milestone

After Core RC sign-off:

1. Human QA + clean-machine install validation ([RC_1_0_RELEASE_VALIDATION.md](RC_1_0_RELEASE_VALIDATION.md))
2. Windows code signing / store pipeline
3. Permanent project delete (if product requires)
4. AI / Mentor Mode (separate milestone — out of Pre-AI scope)



---



## Validation commands



```powershell
cargo test -p backend
cargo check -p grafitalk
npm run build --prefix frontend
npm run test --prefix frontend
npm run lint --prefix frontend
.\scripts\release.ps1
npx @tauri-apps/cli dev
```



---



## Related documents



- [README](../README.md)

- [Architecture](ARCHITECTURE.md)

- [Decisions](DECISIONS.md)

- [Progress](PROGRESS.md)

- [Graf-ID export schema](GRAF_ID_EXPORT_SCHEMA.md)


