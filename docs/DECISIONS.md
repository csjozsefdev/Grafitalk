# GrafiTalk — Product decisions

Core decisions for **GrafiTalk Core RC**. These are intentional, not oversights.

**Philosophy:** **GrafiTalk prepares; humans send.**

---

## Local-first

- All project data lives in a local SQLite file on the user's machine
- No cloud accounts, sync, or remote API calls for draft generation
- Offline-capable by design
- Desktop application (Tauri 2); no backend server

**Why:** Freelancer workflow privacy, predictable behavior, no infrastructure dependency.

---

## Human review required

- Every outbound message is reviewed in **Current Preview** before copy or export
- Generate is a starting point, not a final send action
- Copy and export are the only outbound paths — user pastes or shares files manually

**Why:** Client communication quality and accountability stay with the freelancer.

---

## No auto-send

- GrafiTalk never sends email, chat, SMS, or push notifications
- Copy-to-clipboard and file export are the only outbound actions

**Why:** Prevents accidental client messages and keeps legal/compliance scope minimal.

---

## No CRM direction

- GrafiTalk is a per-project communication workbench, not a contact manager or pipeline tool
- No inbox, no deal stages, no team assignment

**Why:** Scope stays focused on draft preparation, not client relationship management.

---

## No AI dependency

- Draft generation is deterministic template + rule-based transformation
- No LLM, no external inference APIs, no ML models
- AI / Mentor Mode is explicitly deferred to a separate future milestone

**Why:** Reproducibility, offline use, no hallucinated client updates.

---

## One latest draft per project

- Each project stores a single `draft_text` column
- Generate and manual edits overwrite the current draft
- No version history table in Core RC

**Why:** Simplicity, clear mental model, matches “current message I’m preparing.”

---

## No unlimited draft history

- Previous generated versions are not retained automatically
- Users who need history should copy drafts elsewhere or export files

**Why:** Scope control and storage simplicity.

---

## Graph-ID JSON adapter architecture

- GrafiTalk must not read Graph-ID internal databases
- Integration uses a **versioned handoff contract** ([GRAF_ID_HANDOFF.md](GRAF_ID_HANDOFF.md), [GRAF_ID_EXPORT_SCHEMA.md](GRAF_ID_EXPORT_SCHEMA.md))
- GrafiTalk implements **import**; Graf-ID implements **export** in its own repo
- Import pipeline: format detection → validation → `PrettyPrintContext` → `context_text`

**Why:** Loose coupling, independent release cycles, clear ownership boundaries.

---

## English-only product and codebase text

- UI labels, errors, documentation, and code comments are English
- Generated drafts may contain any language present in user context

**Why:** Single-locale RC; localization deferred.

---

## Deterministic generation

- Same project + same saved context + same template → same generated draft
- No randomness, no time-based wording changes

**Why:** Trust, testability, and debuggability.

---

## UI stability

- Accepted layout: Projects sidebar | Current Preview | Context panel
- Workflow bar: Context → Template → Draft → Review → Copy
- Light “communication studio” visual identity (not dark terminal / cyberpunk)
- GrafiTalk startup splash uses light theme variant (related to Graf-ID splash family, adapted for GrafiTalk)

**Why:** Product identity as a communication studio, not admin software.

---

## Export (Core RC)

- Draft export is live: TXT, MD, JSON, PDF via the action bar **Export** button
- Export records `last_exported_at` and format in SQLite
- Failed export does not mutate draft text or export metadata

Schema: [DRAFT_EXPORT_SCHEMA.md](DRAFT_EXPORT_SCHEMA.md)

---

## Project lifecycle (Core RC)

- Projects can be **renamed**, **archived**, and **restored** from the sidebar
- Archive retains all data in SQLite; no permanent delete in RC
- Lifecycle prompts use app-styled dialogs (not native OS confirm)

---

## Read-aloud (Core RC)

- **Read-aloud:** optional Web Speech API; platform-dependent; stops on project switch

**Why:** Assist review without replacing human judgment or adding cloud services.

**Note:** Grafi advisor is integrated from upstream Grafi via a bottom-left portal host. Disable Grafi or adjust motion in Settings. The startup splash remains a separate light GrafiTalk overlay.

---

## Related documents

- [Limitations](LIMITATIONS.md) — what Core RC does not do yet
- [Architecture](ARCHITECTURE.md) — how decisions map to code
- [Core status](CORE_STATUS.md) — current completion state
- [Release notes (RC)](RELEASE_NOTES_RC.md) — shipped features summary
