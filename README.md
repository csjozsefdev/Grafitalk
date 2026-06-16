# GrafiTalk

**GrafiTalk prepares; humans send.**

GrafiTalk is a **local-first desktop communication workbench** for freelancers. You gather project **context**, choose a **template**, **generate** a deterministic draft, **review and edit** it in Current Preview, then **copy** or **export** it into email or chat yourself.

GrafiTalk never sends messages for you.

---

## Purpose

Transform project context into human-readable communication drafts — with a mandatory human review step before anything leaves your machine.

---

## Core workflow

```text
Context  →  Template  →  Draft  →  Review / Edit  →  Copy / Export
```

1. Select or create a **project**
2. Add manual **context** notes or **import** a Graf-ID handoff file (JSON, TXT, or MD)
3. Choose a **template** in the action bar (five deterministic options)
4. Click **Generate Draft**
5. **Review and edit** the draft in **Current Preview**
6. **Copy** or **Export** (TXT, MD, JSON, PDF) before sending outside the app

The title bar workflow pill shows: Context → Template → Draft → Review → Copy.

---

## What GrafiTalk is

- A preparation studio for client-facing updates
- A per-project workspace (context + latest draft)
- A deterministic, local-only draft generator
- A human review step before anything leaves your machine
- A **desktop application** (Tauri 2 + React) with no cloud dependency

## What GrafiTalk is not

- Not a CRM, inbox, or team management tool
- Not an AI writer or cloud service — **no AI requirement**
- Not an auto-sender or scheduling tool — **no auto-send**
- Not a Graph-ID database client — structured handoff via import/export contract ([spec](docs/GRAF_ID_HANDOFF.md), [export schema](docs/GRAF_ID_EXPORT_SCHEMA.md))

---

## Current capabilities (Core RC)

| Area | Supported |
|------|-----------|
| **Project management** | Create, list, select, rename, archive, restore |
| **Context editing** | Manual notes per project; debounced autosave |
| **Graf-ID import** | JSON, TXT, MD handoff files |
| **Templates** | Five deterministic templates (Status Update, Client Update, Debug Report, Handover, Weekly Summary) |
| **Draft generation** | Rule-based pipeline from saved context |
| **Review / edit** | Editable Current Preview with metadata badges |
| **Copy** | Clipboard only |
| **Export** | TXT, MD, JSON, PDF |
| **Startup** | Light GrafiTalk splash on cold start |
| **Grafi advisor** | Upstream Grafi UI (bottom-left portal host) |
| **Read-aloud** | Web Speech API when supported |
| **Diagnostics** | Version, DB path, backup export, read-aloud preference |

**Grafi advisor:** Reintegrated from upstream [Grafi](https://github.com/csjozsefdev/Grafi) via a body portal host. The **startup splash** remains separate.

## Local-first philosophy

- All projects, context, and drafts live in a local **SQLite** database on your computer
- **No cloud sync**, no accounts, no network calls for generation
- Restarting the app does not erase your work
- You control when and where text is sent — always outside GrafiTalk

---

## Quick start (users)

1. Open GrafiTalk (startup splash appears briefly)
2. Create or select a project in the left sidebar
3. Add context in the right panel (or **Import** from Graf-ID)
4. Choose a **template**, then click **Generate Draft**
5. Review Current Preview, then **Copy** or **Export**

Details: [User guide](docs/USER_GUIDE.md)

---

## Quick start (developers)

```powershell
cd C:\Projektek\Grafitalk
npx @tauri-apps/cli dev
```

```powershell
cargo test -p backend
cargo check -p grafitalk
npm run build --prefix frontend
npm run test --prefix frontend
npm run lint --prefix frontend
```

### Release build (Windows)

```powershell
.\scripts\release.ps1
```

Details: [Developer guide](docs/DEVELOPER_GUIDE.md) · [Release notes (RC)](docs/RELEASE_NOTES_RC.md)

---

## Documentation index

| Document | Purpose |
|----------|---------|
| [User guide](docs/USER_GUIDE.md) | Daily use, workflow, UI areas |
| [Developer guide](docs/DEVELOPER_GUIDE.md) | Repo layout, dev setup, pipelines |
| [Architecture](docs/ARCHITECTURE.md) | Data flow, persistence, IPC |
| [QA checklist](docs/QA_CHECKLIST.md) | Manual verification steps |
| [Decisions](docs/DECISIONS.md) | Core product decisions |
| [Limitations](docs/LIMITATIONS.md) | Honest RC constraints |
| [Graf-ID handoff](docs/GRAF_ID_HANDOFF.md) | Import/export contract overview |
| [Graf-ID export schema](docs/GRAF_ID_EXPORT_SCHEMA.md) | v0.1 JSON/TXT/MD contract |
| [Draft export schema](docs/DRAFT_EXPORT_SCHEMA.md) | Current Preview export formats |
| [Core status](docs/CORE_STATUS.md) | Release gate checklist |
| [Release notes (RC)](docs/RELEASE_NOTES_RC.md) | Core RC feature summary |
| [RC cleanup report](docs/RC_CLEANUP_REPORT.md) | Cleanup + readiness audit |
| [Backup & restore](docs/BACKUP_RESTORE.md) | Manual database recovery |
| [Installing](docs/INSTALLING.md) | Windows installer notes |

---

## Core Release Candidate scope

| In scope | Out of scope |
|----------|--------------|
| Projects: create, list, select, rename, archive, restore | AI / LLM generation |
| Manual + imported context (Graf-ID) | Auto-send |
| Five deterministic templates | Cloud sync |
| Generate, review, edit, autosave | CRM / multi-user |
| Copy + Export (TXT / MD / JSON / PDF) | Unlimited draft history |
| Review metadata in SQLite | Permanent project delete |
| Dynamic workflow indicator | Code signing (org-dependent) |
| Read-aloud | |
| Diagnostics + DB backup | |
| Custom title bar (Windows) | |
| GrafiTalk startup splash | |
| Local SQLite persistence | |

Run [QA checklist](docs/QA_CHECKLIST.md) and [RC 1.0 validation](docs/RC_1_0_RELEASE_VALIDATION.md) before release sign-off.

---

## Project layout

```text
backend/       Rust domain (SQLite, templates, import/export)
src-tauri/     Tauri 2 shell + IPC commands
frontend/      React workbench UI
docs/          Product and technical documentation
examples/      Sample Graf-ID handoff files
scripts/       release.ps1 (Windows MSI + NSIS)
```

---

**GrafiTalk Core RC is local-first, deterministic, review-first, and copy-only.**
