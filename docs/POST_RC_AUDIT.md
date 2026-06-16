# GrafiTalk Core RC — Post-Implementation GO/NO-GO Procedure

Run this audit **after** RC implementation is complete. Do not treat planning documents as evidence.

## 1. Evidence collection

1. Run `cargo test -p backend`
2. Run `npm run test --prefix frontend`
3. Run `npm run build --prefix frontend`
4. Run `npm run lint --prefix frontend`
5. Run `npx @tauri-apps/cli build` on a Windows machine
6. Install the produced bundle on a clean or low-trust VM
7. Execute [QA_CHECKLIST.md](QA_CHECKLIST.md) including RC sections

## 2. Original audit gap review

| Gap | Evidence required |
|-----|-------------------|
| Installer / release pipeline | `scripts/release.ps1` + successful `tauri build` artifact |
| Project lifecycle | Archive UI + `archive_project` IPC + backend tests |
| Dynamic workflow indicator | Manual walkthrough of workflow pill states |
| Settings / backup / diagnostics | Diagnostics modal + DB backup command |
| Frontend tests | `npm run test` green |
| Documentation drift | Spot-check README, DEVELOPER_GUIDE, DECISIONS, USER_GUIDE |
| Grafi advisor | Bottom-left contextual messages verified |
| Visual polish | Diagonal background + custom title bar controls |

## 3. Product boundary review

Confirm the RC diff does **not** add:

- AI / LLM generation
- Cloud sync or remote APIs for core workflow
- CRM or multi-user features
- Auto-send, email, or chat integrations

## 4. Data-loss risk review

- Template change with dirty draft requires confirmation
- Invalid Graf-ID import does not overwrite context
- Failed export does not mutate draft or export metadata
- Archive hides project but retains SQLite row
- Autosave flush on project switch still works

## 5. Scoring template

| Area | Weight | Score 0-10 | Notes |
|------|--------|------------|-------|
| Core workflow | 25% | | |
| Release packaging | 20% | | |
| Reliability / data safety | 20% | | |
| RC UX features | 15% | | |
| Docs + tests | 10% | | |
| Product boundary compliance | 10% | | |

**Completion %** = weighted average × 10

## 6. Decision matrix

| Decision | Criteria |
|----------|----------|
| **GO** | Installer works on clean machine; no P0 bugs; QA signed off; docs accurate |
| **CONDITIONAL GO** | Core workflow solid; installer or minor UX issues remain; no data-loss bugs |
| **NO-GO** | Installer fails; data loss; broken import/export/generate; scope violations |

## 7. Required output

- Evidence-based completion %
- Remaining risks (P0 / P1 / P2)
- GO / CONDITIONAL GO / NO-GO
- Blocking issues list
- Recommended next milestone

## 8. Sign-off

Tester: ____________________  
Date: ____________________  
Decision: ____________________
