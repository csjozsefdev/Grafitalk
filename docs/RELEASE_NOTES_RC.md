# GrafiTalk Core — Release Candidate Notes

**Version:** 1.0.0  
**Product line:** GrafiTalk Core  
**Philosophy:** GrafiTalk prepares; humans send.

---

## Summary

GrafiTalk Core RC is a local-first desktop communication workbench. It helps freelancers turn project context into client-ready drafts, review them, and copy or export — without AI, cloud services, or automatic sending.

---

## New in Core RC

### Startup experience

- **GrafiTalk splash screen** — light-themed startup overlay with Grafi green accents
- Rotating startup copy; minimum display ~1.2 s; fades into main UI when projects finish loading
- Anti-flash background in `index.html` for smoother cold start

### Project lifecycle

- **Rename** active projects from the sidebar
- **Remove** projects via sidebar `⋯` menu (data retained, hidden from active list — archived in SQLite)
- **Restore** archived projects via “Show archived” section
- App-styled **ConfirmDialog** and **NamePromptDialog** (no native `window.confirm`)

### Workflow and review

- **Dynamic workflow indicator** in the title bar (Context → Template → Draft → Review → Copy)
- **Template dirty confirmation** when switching templates after manual draft edits
- **Review metadata** in SQLite and DocumentPanel (last generated, template, last export)

### Graf-ID integration

- Import Graf-ID handoff files: **JSON**, **TXT**, **MD**
- Validated import pipeline; failure-safe (does not overwrite context on error)
- Contract: [GRAF_ID_HANDOFF.md](GRAF_ID_HANDOFF.md), [GRAF_ID_EXPORT_SCHEMA.md](GRAF_ID_EXPORT_SCHEMA.md)

### Export support

- Export Current Preview as **TXT**, **MD**, **JSON**, or **PDF**
- Export metadata recorded per project (`last_exported_at`, format)
- Failed export does not mutate draft or metadata

### Grafi Advisor

- Bottom-left contextual guidance (local only, no network)
- Preference toggles in **Settings → Grafi**: enabled, motion, silent mode, critical-only
- Template-change and workflow events trigger contextual reminders

### Read-aloud

- Local **Web Speech API** text-to-speech for Current Preview
- Disabled when unsupported; stops on project switch
- Toggle in **Settings** (Read-aloud category)

### Settings and reliability

- **Settings workspace** — Grafi, read-aloud, export placeholder, About / Diagnostics
- **About / Diagnostics** — version, DB path, project count, database backup
- **Custom title bar** (Windows) with window controls
- **Autosave** — 500 ms debounce for context and draft; flush on project switch
- **Hardening matrix** — backend integration tests for import, templates, export, isolation

### Packaging

- `scripts/release.ps1` — Windows MSI + NSIS (unsigned RC)
- Install guide: [INSTALLING.md](INSTALLING.md)
- Backup/restore guide: [BACKUP_RESTORE.md](BACKUP_RESTORE.md)

---

## Major improvements over Pro Pre-AI baseline

| Area | Improvement |
|------|-------------|
| Project lifecycle | Rename, remove (archive), restore via `⋯` menu (was create/list/select only) |
| UX polish | Custom title bar, Grafi advisor, Settings workspace, read-aloud, app-styled dialogs |
| Workflow clarity | Dynamic step indicator; template dirty guard |
| Release tooling | Release script, Vitest frontend tests, validation docs |
| Startup | GrafiTalk-branded light splash |

---

## Templates (unchanged count, full RC support)

1. Status Update  
2. Client Update  
3. Debug Report  
4. Handover  
5. Weekly Summary  

All use the deterministic Pretty Print → `render_draft` pipeline.

---

## Explicit non-features (by design)

- No AI / LLM generation  
- No auto-send (email, chat, SMS)  
- No cloud sync or accounts  
- No CRM or multi-user collaboration  
- No unlimited draft version history  
- No permanent project delete in RC  

---

## Upgrade / migration

- SQLite migrations **001–005** run automatically on startup
- Existing `localStorage` last-generated timestamps migrate once when DB field is empty
- No breaking changes to Graf-ID handoff contract v0.1

---

## Known RC limitations

See [LIMITATIONS.md](LIMITATIONS.md). Highlights:

- Unsigned Windows installers (SmartScreen may warn)
- Read-aloud depends on WebView2 / platform speech support
- No guided DB corruption repair wizard
- Human QA sign-off pending per [RC_1_0_RELEASE_VALIDATION.md](RC_1_0_RELEASE_VALIDATION.md)

---

## Validation before ship

```powershell
cargo test -p backend
cargo check -p grafitalk
npm run test --prefix frontend
npm run lint --prefix frontend
npm run build --prefix frontend
.\scripts\release.ps1
```

Manual: [QA_CHECKLIST.md](QA_CHECKLIST.md)

---

## Related documents

- [README](../README.md)
- [CORE_STATUS.md](CORE_STATUS.md)
- [RC_CLEANUP_REPORT.md](RC_CLEANUP_REPORT.md)
