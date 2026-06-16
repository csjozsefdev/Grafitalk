# GrafiTalk — Database backup and manual restore

GrafiTalk stores all projects, context, and drafts in a local SQLite file (`grafitalk.db`). The in-app **Export database backup** action copies that file to a location you choose. This guide explains how to back up and manually restore your data.

**Important:** GrafiTalk does not include a corruption-repair wizard or automated restore UI in Core RC 1.0. Restore is a manual file operation.

---

## Locate your database

1. Open GrafiTalk.
2. Click the **About / diagnostics** button in the title bar.
3. Note the **Database** path shown in the diagnostics panel, or click **Reveal data folder path**.

On Windows, the database is typically under the app data directory for `talk.grafi.app`, in a file named `grafitalk.db`.

---

## Create a backup

### In-app (recommended)

1. Open **About / diagnostics**.
2. Click **Export database backup**.
3. Choose a destination path (for example `grafitalk-backup-2026-06-14.db`).
4. Confirm the file was saved.

### Manual copy

1. **Quit GrafiTalk completely** (recommended before copying the live file).
2. Copy `grafitalk.db` to a safe location (external drive, cloud folder you control, etc.).

**Hot backup note:** The in-app backup copies the file while the app is running. For maximum safety before a risky operation, quit the app first and copy the file manually.

---

## Manual restore procedure

1. **Quit GrafiTalk completely.** The database must not be in use.
2. Locate your current `grafitalk.db` (see above).
3. **Rename** the current file to keep a rollback copy, for example:
   - `grafitalk.db` → `grafitalk.db.bak`
4. Copy your backup file into the same folder and name it **`grafitalk.db`**.
5. Start GrafiTalk and verify your projects load.

---

## Rollback if restore fails

If GrafiTalk fails to open or projects look wrong:

1. Quit GrafiTalk.
2. Remove or rename the restored `grafitalk.db`.
3. Rename `grafitalk.db.bak` back to `grafitalk.db`.
4. Start GrafiTalk again.

---

## Risks and warnings

| Risk | Mitigation |
|------|------------|
| Replacing the wrong file | Double-check the folder path in diagnostics before replacing |
| Partial or interrupted copy | Verify backup file size; copy again if unsure |
| Restoring an older schema | Use a backup from the same or older app version; migrations run on startup |
| Data loss | Always keep `grafitalk.db.bak` until you confirm the restore worked |

GrafiTalk **cannot** repair a corrupt SQLite file automatically. If a backup will not open, keep the original files and restore from an older backup if available.

---

## What is not included

- In-app restore wizard
- Automatic scheduled backups
- Cloud backup sync
- Merge of two database files

---

## Related documents

- [User guide](USER_GUIDE.md)
- [Developer guide](DEVELOPER_GUIDE.md)
- [QA checklist](QA_CHECKLIST.md)
