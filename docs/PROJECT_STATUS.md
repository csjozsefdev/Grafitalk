# GrafiTalk — Project status

**Last updated:** v1.0.0 automated gate pass  
**Product line:** GrafiTalk Core  
**Philosophy:** GrafiTalk prepares; humans send.

---

## Summary

GrafiTalk is a **local-first desktop communication preparation tool** (Tauri 2 + React + SQLite). It helps freelancers turn per-project **context** into deterministic **drafts**, **review** them, then **copy** or **export** — without AI, cloud services, or automatic sending.

```text
Project  →  Context  →  Template  →  Draft  →  Review  →  Copy / Export
```

---

## Completed (Core RC)

| Area | Status | Notes |
|------|--------|-------|
| **Core MVP** | Done | Three-panel workbench, project isolation, SQLite persistence |
| **Project management** | Done | Create, select, rename, remove (archive), restore via sidebar `⋯` menu |
| **Context editor** | Done | Manual notes; 500 ms debounced autosave |
| **Graf-ID import** | Done | JSON, TXT, MD handoff files; failure-safe |
| **Templates** | Done | Five deterministic templates |
| **Draft generation** | Done | Rule-based pipeline from saved context |
| **Review / edit** | Done | Editable Current Preview with metadata badges |
| **Copy** | Done | Clipboard only |
| **Export** | Done | TXT, MD, JSON, PDF |
| **Grafi Advisor** | Done | Upstream Grafi via bottom-left portal host; toggles in Settings |
| **Settings** | Done | Dedicated workspace: Grafi, read-aloud, export placeholder, About / Diagnostics |
| **Startup splash** | Done | Light GrafiTalk overlay (`white-grafi-splash.jpg`) |
| **Custom title bar** | Done | Windows — brand, workflow pill, window controls |
| **Diagnostics + backup** | Done | Settings → About / Diagnostics |
| **Hardening matrix** | Done | 134 backend integration tests |
| **Frontend tests** | Done | 51 Vitest tests |
| **Release packaging** | Done | `scripts/release.ps1` (unsigned Windows MSI + NSIS) |

---

## Remaining / future

| Item | Priority | Notes |
|------|----------|-------|
| **Read-aloud polish** | Medium | Basic Web Speech works; voice selector is placeholder (“coming later”) |
| **Windows code signing** | Release | SmartScreen warnings on unsigned RC builds |
| **Clean-machine validation** | Release | [RC_1_0_RELEASE_VALIDATION.md](RC_1_0_RELEASE_VALIDATION.md) |
| **Permanent project delete** | Product | Archive (remove) retains SQLite row today |
| **Draft history / versions** | Future | One latest draft per project |
| **AI / Mentor Mode** | Out of scope | Separate milestone — not Core RC |
| **Cloud sync / accounts** | Out of scope | Local-first by design |

---

## Known limitations

See [LIMITATIONS.md](LIMITATIONS.md). Core constraints:

- **Manual review required** — nothing is sent automatically
- **No auto-send** — copy and export only; user pastes into email/chat externally
- **No AI required** — deterministic templates only
- **Local-first** — all data in `{app_data_dir}/grafitalk.db`; no cloud dependency
- **No permanent delete** — Remove hides project (archive); data retained for restore
- **One draft per project** — regenerate overwrites previous draft

---

## Validation status

| Check | Result |
|-------|--------|
| Backend tests (`cargo test -p backend`) | 134 passed |
| Frontend tests (`npm run test --prefix frontend`) | 51 passed |
| Frontend lint (`npm run lint --prefix frontend`) | Pass |
| npm audit high (`npm audit --audit-level=high --prefix frontend`) | 0 vulnerabilities |
| Production build (`npm run build --prefix frontend`) | Pass |
| `cargo check -p grafitalk` | Pass |
| Manual QA | Pending — [QA_CHECKLIST.md](QA_CHECKLIST.md) |
| Clean-machine installer | Pending — [RC_1_0_RELEASE_VALIDATION.md](RC_1_0_RELEASE_VALIDATION.md) |

---

## Related documents

| Document | Purpose |
|----------|---------|
| [README](../README.md) | Product overview and quick start |
| [Core status](CORE_STATUS.md) | Release gate checklist |
| [Architecture](ARCHITECTURE.md) | Data flow, IPC, persistence |
| [Decisions](DECISIONS.md) | Product policy |
| [Progress](PROGRESS.md) | Milestone history |
