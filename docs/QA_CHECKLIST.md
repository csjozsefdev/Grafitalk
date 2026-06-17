# GrafiTalk — QA checklist

Manual verification for **GrafiTalk Core RC**. Run after meaningful changes to startup, import, templates, export, persistence, lifecycle, or project switching.

**Environment:** `npx @tauri-apps/cli dev` from repo root (or packaged installer for release validation).

---

## Startup

- [ ] App launches without crash
- [ ] Light GrafiTalk splash appears on cold start (not dark Graf-ID black background)
- [ ] Splash shows rotating English startup copy
- [ ] Splash stays visible at least ~1 second
- [ ] Splash fades out smoothly into main UI
- [ ] No white flash before splash (background matches light theme)
- [ ] No layout jump when splash disappears
- [ ] Main workbench loads normally after splash
- [ ] No console errors on startup

---

## Layout and workflow

- [ ] Empty state shows “No projects yet” when database is fresh
- [ ] Three-panel layout: sidebar | Current Preview | Context
- [ ] Context panel height capped — preview editor remains visible
- [ ] Copy and Export buttons visible in action bar
- [ ] No unwanted horizontal scrollbar at normal or maximized window width
- [ ] Diagonal ambient background does not reduce text readability
- [ ] Workflow bar shows Context → Template → Draft → Review → Copy
- [ ] Dynamic workflow pill updates with project state

---

## Project creation

- [ ] Create project **A** with a clear name
- [ ] Create project **B** with a different name
- [ ] Empty name rejected on create
- [ ] No error banners on normal create

---

## Project selection

- [ ] Selecting a project loads its context and draft panels
- [ ] Sidebar shows **Last used** updating on selection
- [ ] Rapid switching does not mix content between projects

---

## Context editing

- [ ] Type context in project **A** — wait ~1 s — switch away and back — text persists
- [ ] **Context saved** indicator turns green when context is present
- [ ] Empty context shows idle indicator (not green)
- [ ] Edit context, switch project within 500 ms — previous project edits flushed

---

## Graf-ID import

- [ ] **Import** button visible in Context panel
- [ ] Import sample JSON (`examples/graf_id_project_context.sample.json`) — context fills with labeled sections
- [ ] Import sample TXT and MD — equivalent structured context
- [ ] Invalid JSON shows clear error; existing context unchanged
- [ ] Empty file rejected with clear error
- [ ] Import on **A** does not affect **B**

---

## Template switching

- [ ] Template dropdown shows all five templates (Status Update, Client Update, Debug Report, Handover, Weekly Summary)
- [ ] Selected template persists when switching away and back to same project
- [ ] Generate uses currently selected template (subject line and sections change)
- [ ] Edit draft manually, change template — app-styled confirmation appears
- [ ] Cancel keeps edited draft; confirm regenerates draft

---

## Draft generation

- [ ] **Generate Draft** disabled when no project selected
- [ ] **Generate Draft** disabled when context is empty
- [ ] With context present, generate produces structured output
- [ ] Compact deployment note transforms to client-readable paragraphs
- [ ] Graf-ID labeled context maps to structured sections
- [ ] Output does **not** contain `Hi team,`, `Please review before sending.`, or `Best regards`
- [ ] **Draft ready** indicator turns green after generation
- [ ] **Last generated** (relative time) shown in DocumentPanel status row

---

## Review editing

- [ ] Draft textarea is editable after generation
- [ ] Manual edits autosave (~500 ms debounce)
- [ ] Subject header and body reflect edits
- [ ] Review metadata (template, last generated) visible in status row

---

## Copy

- [ ] **Copy** disabled when preview is empty
- [ ] **Copy** enabled when draft has text
- [ ] Copy puts full draft on clipboard
- [ ] Brief workflow highlight on **Copy** step after success
- [ ] Toast or feedback confirms success or failure

---

## Export TXT

- [ ] **Export** disabled when preview is empty
- [ ] Save dialog offers TXT filter
- [ ] Exported TXT matches Current Preview content
- [ ] Failed export shows error; draft unchanged

---

## Export MD

- [ ] Save dialog offers MD filter
- [ ] Exported MD matches documented format ([DRAFT_EXPORT_SCHEMA.md](DRAFT_EXPORT_SCHEMA.md))

---

## Export JSON

- [ ] Save dialog offers JSON filter
- [ ] Exported JSON is valid and matches preview content

---

## Export PDF

- [ ] Save dialog offers PDF filter
- [ ] PDF opens and contains draft text

---

## Export metadata

- [ ] Successful export updates **Last exported** in DocumentPanel
- [ ] Failed export does not update export metadata
- [ ] Quit and relaunch — export metadata persists per project

---

## Project remove (archive)

- [ ] Each project card shows a compact `⋯` menu
- [ ] **Rename** opens name prompt with current project name
- [ ] **Remove** opens app-styled confirmation (not native OS dialog)
- [ ] Removed project disappears from active list (archived in SQLite — not hard-deleted)
- [ ] Removing active project selects another project or empty state
- [ ] When all projects removed/archived, guidance message appears

---

## Project rename

- [ ] **Rename** (via `⋯` menu) opens name prompt with current project name
- [ ] Empty name rejected
- [ ] Rename preserves context, draft, and metadata
- [ ] Rename failure shows sidebar error message

---

## Project restore

- [ ] **Show archived** lists archived projects
- [ ] **Restore** returns project to active list
- [ ] Restored project retains context, draft, and metadata

---

## Grafi Advisor

- [ ] Grafi figure visible bottom-left on workbench (when enabled in Settings)
- [ ] Settings → Grafi → disable hides Grafi entirely
- [ ] Bubble **X** closes bubble only — Grafi figure remains
- [ ] Clicking Grafi reopens bubble after dismiss
- [ ] Transient success messages auto-collapse (~5.5 s)
- [ ] Warning / error messages take priority over idle hints
- [ ] Grafi does not cover Copy / Export / Generate controls
- [ ] Grafi host does not introduce page scrollbars

---

## Settings

- [ ] **Settings** link in sidebar opens dedicated settings workspace (not a cramped modal)
- [ ] **Back to workbench** returns to three-panel layout
- [ ] Escape closes Settings
- [ ] Grafi toggles persist across restart (localStorage)
- [ ] Read-aloud toggle persists
- [ ] Voice selector shows disabled “coming later” placeholder
- [ ] About / Diagnostics loads version, DB path, project count
- [ ] Export database backup works from About / Diagnostics

---

## Read-aloud

- [ ] Speaker shows unavailable hint when speech API missing
- [ ] Speaker disabled when draft empty or read-aloud disabled in diagnostics
- [ ] Play/stop works when supported
- [ ] Speech stops on project switch

---

## Title bar and window

- [ ] Native OS title bar hidden; custom bar shows brand + workflow + window controls
- [ ] Title bar icon shows Grafi head only (transparent PNG — no white square plate)
- [ ] Minimize, maximize, close work; drag region moves window
- [ ] Settings (sidebar) opens diagnostics / backup — not a separate modal

---

## Persistence and isolation

- [ ] Project **A** / **B** context and drafts remain isolated after switching
- [ ] Quit and relaunch — projects, contexts, drafts, templates, metadata persist
- [ ] Only one draft per project — regenerating overwrites previous draft

---

## Error behavior

- [ ] Empty context + Generate — blocked with clear message
- [ ] Invalid project state does not white-screen the UI

---

## Automated tests (run before release)

```powershell
cargo test -p backend
cargo check -p grafitalk
npm run build --prefix frontend
npm run test --prefix frontend
npm run lint --prefix frontend
```

Expected: all backend tests pass (134), frontend tests pass (51), hardening matrix in `backend/src/hardening/matrix.rs` passes.

Release build: `.\scripts\release.ps1`

Human validation: [RC_1_0_RELEASE_VALIDATION.md](RC_1_0_RELEASE_VALIDATION.md)

---

## Sign-off

| Area | Pass | Notes |
|------|------|-------|
| Startup / splash | | |
| Projects (create / select) | | |
| Rename / remove / restore | | |
| Settings | | |
| Context | | |
| Import | | |
| Templates | | |
| Generate | | |
| Review / edit | | |
| Copy | | |
| Export (TXT/MD/JSON/PDF) | | |
| Grafi / read-aloud | | |
| Layout | | |
| Persistence | | |
| Errors | | |

Tester: _______________  Date: _______________
