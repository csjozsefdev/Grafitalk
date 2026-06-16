# GrafiTalk — RC Cleanup Report

**Date:** 2026-06-15  
**Scope:** Core Release Candidate cleanup + documentation pass  
**Rule:** No new features; cleanup and docs only.

---

## PART 1 — Repository cleanup

### Removed files

| File | Reason |
|------|--------|
| `debug-e471c8.log` | Stale agent debug log at repo root; `*.log` already in `.gitignore`; not part of the product |

### Removed code blocks

None. No clearly dead application code was identified safe to delete without risk.

### Reviewed but kept (uncertain or in use)

| Item | Decision | Reason |
|------|----------|--------|
| `frontend/public/icons.svg` | **Kept** | No references found; may be reserved asset — uncertain |
| `formatLastUsed.ts` vs `formatRelativeShort.ts` | **Kept** | Different UX: sidebar uses locale datetime; review row uses relative time |
| `AppShell.tsx` (~1,059 lines) | **Kept** | Active orchestrator; partial extractions already done — refactor deferred |
| Duplicate audit docs (`RC_AUDIT_RESULTS.md`, `RC_1_0_AUDIT_RESULTS.md`, `POST_RC_AUDIT.md`) | **Kept** | Serve different validation phases; consolidate post-sign-off |

### No action taken

- No unused React components found (all components in `frontend/src/components/` are imported)
- No obsolete TODO/FIXME comments in application source
- `examples/` and `Grafid Importok/` retained (samples / reference imports)
- `target/` and `node_modules/` already gitignored

---

## PART 2 — Documentation audit (drift identified and addressed)

| Document | Outdated section | Required update | Status |
|----------|------------------|-----------------|--------|
| `README.md` | Pro Pre-AI dual scope; workflow missing Template step; no splash | Full Core RC rewrite | **Updated** |
| `DEVELOPER_GUIDE.md` | "19 commands"; restore UI out of scope; no splash; outdated post-RC list | Architecture, 23 IPC, splash boundary, pipelines | **Updated** |
| `DECISIONS.md` | Pro Pre-AI framing; missing philosophy line, splash, rename/restore | Core RC decisions | **Updated** |
| `QA_CHECKLIST.md` | Pro Pre-AI title; no startup/splash; export not per-format | Core RC checklist with splash + export sections | **Updated** |
| `CORE_STATUS.md` | "no archive restore UI"; 129+ tests | Splash, rename/restore, 29 frontend tests | **Updated** |
| `LIMITATIONS.md` | "Pro Pre-AI" title | Core RC title | **Updated** |
| `USER_GUIDE.md` | No startup splash mention | Opening section added | **Updated** |
| `ARCHITECTURE.md` | Pro Pre-AI header; missing lifecycle + app IPC | Header + command table | **Updated** |
| `docs/README.md` | MVP wording; missing RC docs | Index refresh | **Updated** |
| `PROGRESS.md` | No splash milestone | Minor — RC complete statement sufficient | **Partial** (tracking doc; low priority) |
| `RC_AUDIT_RESULTS.md` | "No archive restore UI" | Historical audit snapshot | **Kept** (point-in-time record) |
| `POST_RC_AUDIT.md` | References pre-splash state | Procedure still valid | **Kept** |
| ROADMAP documents | N/A in repo | Roadmap lives in planning context, not `docs/` | **Noted** |

### Remaining minor drift (acceptable for RC)

- `PROGRESS.md` still lists Pro Pre-AI milestone table (historical record)
- `RC_AUDIT_RESULTS.md` / `POST_RC_AUDIT.md` reflect earlier CONDITIONAL GO state
- No screenshots in docs (never existed — not outdated assets)

---

## PART 7 — Structure review (recommendations only)

### Oversized files

| File | Lines (approx.) | Risk |
|------|-----------------|------|
| `frontend/src/components/AppShell.tsx` | ~1,059 | **High** — central orchestrator |
| `frontend/src/styles/app.css` | ~1,250 | **Medium** — monolithic styles |
| `backend/src/service/project_service.rs` | Large | **Medium** — many responsibilities |

### Mixed responsibilities

| Area | Issue | Recommendation |
|------|-------|----------------|
| `AppShell` | Projects, context, draft, export, import, Grafi, diagnostics, lifecycle | Extract `useProjectWorkspace` hook for load/switch/flush; keep splash outside |
| `AppShell` | Inline handlers for archive/rename/export | Move to `hooks/useProjectLifecycle.ts` (partially done via dialogs) |
| Frontend API | Thin wrappers — good | Maintain pattern |
| Backend | service/repo split — good | Add service tests when touching lifecycle |

### AppShell growth risks

- Every new feature tends to add state + handlers to `AppShell`
- Testing orchestration without integration tests is fragile
- Grafi flash / confirm dialogs already extracted — continue that pattern

### Future maintenance recommendations (no refactor in this pass)

1. Split `AppShell` into domain hooks: `useProjectData`, `useDraftWorkflow`, `useDiagnostics`
2. Add mocked-IPC `AppShell` smoke test (Vitest)
3. Consider CSS modules or co-located styles per major component
4. Consolidate audit docs after human sign-off into single `RELEASE_VALIDATION.md`
5. Add Tauri smoke test in CI when runner supports WebView2

---

## PART 8 — Release notes

See [RELEASE_NOTES_RC.md](RELEASE_NOTES_RC.md).

---

## PART 9 — Final cleanup report

### Scores (1–10)

| Category | Score | Notes |
|----------|-------|-------|
| **Repository health** | **8** | Clean structure; one debug log removed; no dead app code; `AppShell` size is main debt |
| **Documentation** | **9** | Core docs aligned to Core RC; historical audit docs intentionally preserved |
| **Architecture** | **8** | Clear IPC → service → repo; splash boundary clean; no cloud/AI creep |
| **Maintainability** | **7** | Good conventions; `AppShell` concentration and no E2E are main risks |

### Top remaining risks

1. **Human QA not signed off** — `RC_1_0_AUDIT_RESULTS.md` still template
2. **Clean-machine install** — unsigned MSI/NSIS not verified on fresh VM
3. **`AppShell` size** — regression risk without integration tests
4. **Read-aloud / WebView2 variance** — platform-dependent behavior
5. **No code signing** — public distribution friction

### Recommended next milestone

**"Core RC 1.0 GO sign-off"** — execute [QA_CHECKLIST.md](QA_CHECKLIST.md) + [RC_1_0_RELEASE_VALIDATION.md](RC_1_0_RELEASE_VALIDATION.md), complete audit results, optional code signing decision.

Then **"Maintainability hardening"** — AppShell hook extraction + mocked IPC tests (no feature scope).

### Final verdict

## **READY FOR RC**

Repository cleanup is complete for safe items. Documentation reflects actual Core RC implementation including splash, lifecycle, Graf-ID import, four export formats, Grafi, and read-aloud. **Release ship still requires human validation track** — code and docs are RC-ready; operational sign-off is the remaining gate.

---

## Files changed in this pass

| File | Action |
|------|--------|
| `debug-e471c8.log` | Deleted |
| `README.md` | Rewritten |
| `docs/DEVELOPER_GUIDE.md` | Updated |
| `docs/DECISIONS.md` | Updated |
| `docs/QA_CHECKLIST.md` | Updated |
| `docs/CORE_STATUS.md` | Updated |
| `docs/LIMITATIONS.md` | Updated |
| `docs/USER_GUIDE.md` | Updated |
| `docs/ARCHITECTURE.md` | Updated |
| `docs/README.md` | Updated |
| `docs/RELEASE_NOTES_RC.md` | Created |
| `docs/RC_CLEANUP_REPORT.md` | Created (this file) |
