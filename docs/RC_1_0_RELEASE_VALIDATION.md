# GrafiTalk Core RC 1.0 — Release validation package

Human and environment-dependent steps required to move from **CONDITIONAL GO** to **GO**. Cursor can prepare documentation and repository artifacts; a human must execute and sign off.

---

## 1. Full manual QA

Use [QA_CHECKLIST.md](QA_CHECKLIST.md) including Core RC 1.0 sections:

- Project rename
- Archive and restore
- All-archived empty state
- App-styled confirm dialogs (archive, template dirty)
- Read-aloud on WebView2
- Diagnostics preferences (all five Grafi/read-aloud toggles)
- Backup export

### Evidence per check

| Field | Record |
|-------|--------|
| Result | Pass / Fail / Blocked / N/A |
| Tester initials | |
| Date | |
| Build | App version + git commit or installer hash |
| Notes | Screenshot path or short description for failures |

### Sign-off

Complete the sign-off table at the bottom of [QA_CHECKLIST.md](QA_CHECKLIST.md).

---

## 2. Clean-machine installer smoke test

Run on a Windows 10 or 11 x64 VM **without** Rust, Node, or dev dependencies installed.

| Step | MSI | NSIS | Notes |
|------|-----|------|-------|
| Install from release bundle | | | |
| SmartScreen / warning observed | | | Record exact message |
| First launch | | | |
| WebView2 / read-aloud spot check | | | |
| Create project | | | |
| Add context → generate → copy | | | |
| Rename project | | | |
| Archive → restore | | | |
| Export draft | | | |
| Database backup | | | |
| Restart app — data persists | | | |
| Uninstall — note data retention | | | |

### Record

- Windows version and architecture
- WebView2 version (if available)
- Installer file name and SHA256
- Tester and date
- Official installer format chosen by release owner (MSI **or** NSIS if only one is tested)

Artifacts path: `target/release/bundle/`

---

## 3. Unsigned installer communication

For trusted tester distribution:

- Document that Windows SmartScreen may warn on unsigned builds.
- Publish SHA256 hashes of installer files with release notes.
- Download only from a trusted origin (your repo releases page or internal share).
- Do **not** instruct users to disable security controls without verifying file hash and source.

See [INSTALLING.md](INSTALLING.md) (if present) or release notes template below.

---

## 4. Code signing (external prerequisite)

| Item | Owner |
|------|-------|
| Code signing certificate (EV/OV) | Organization |
| `tauri.conf.json` signing configuration | Developer after cert is available |
| Signed build verification | Release owner |

**Classification:**

- **Trusted testers:** Unsigned RC may be acceptable (P2).
- **Public distribution:** Signing required for defensible GO (P0/P1).

Repository placeholder for future signing (do not commit secrets):

```json
"bundle": {
  "windows": {
    "certificateThumbprint": "<thumbprint-from-cert-store>"
  }
}
```

---

## 5. Automated gate (run before human QA)

```powershell
cargo test -p backend
cargo check -p grafitalk
npm run build --prefix frontend
npm run test --prefix frontend
npm run lint --prefix frontend
.\scripts\release.ps1
```

---

## 6. Definition of GO

All of the following:

- [ ] Track A repository milestones merged (lifecycle, dialogs, read-aloud, tests, backup docs)
- [ ] Automated gates green
- [ ] Manual QA checklist signed with evidence
- [ ] Clean-machine install Pass on chosen installer format
- [ ] Unsigned install stance documented **or** signed build verified
- [ ] Post-implementation audit recorded in [RC_1_0_AUDIT_RESULTS.md](RC_1_0_AUDIT_RESULTS.md)

---

## 7. Blocking failure criteria

- Data loss on rename, archive, restore, or project switch
- Installer fails on clean VM
- Core workflow broken (context → generate → review → copy/export)
- Product boundary violation (auto-send, AI, cloud dependency in core path)

---

## 8. Post-implementation audit

After human validation, run the procedure in [POST_RC_AUDIT.md](POST_RC_AUDIT.md) and record results in [RC_1_0_AUDIT_RESULTS.md](RC_1_0_AUDIT_RESULTS.md).

**Do not mark GO until evidence is attached.**
