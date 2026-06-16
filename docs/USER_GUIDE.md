# GrafiTalk — User guide

GrafiTalk helps you prepare client or team updates **before** you send them. You gather context, generate a draft from a fixed template, review and edit it, then copy it to your email or chat app yourself.

**GrafiTalk does not send messages.** Nothing leaves your machine unless you paste it somewhere.

---

## What GrafiTalk is (and is not)

| GrafiTalk is | GrafiTalk is not |
|--------------|------------------|
| A local preparation workbench | A chat app or inbox |
| A place to store notes per project | A CRM or contact manager |
| A deterministic draft generator | An AI writer |
| A review step before copy-paste | An auto-sender |

All projects, context notes, and drafts are stored in a **local SQLite database** on your computer. Restarting the app does not erase your work.

---

## Opening the app

On cold start, GrafiTalk shows a brief **light-themed splash screen** with startup status text, then fades into the main workbench. The splash stays visible until your project list finishes loading (minimum ~1 second).

---

## The workbench layout

When you open GrafiTalk you see three main areas:

```
┌─────────────┬──────────────────────────┬─────────────┐
│  Projects   │   Document (draft)       │   Context   │
│  (sidebar)  │   Review & edit here     │   Notes     │
└─────────────┴──────────────────────────┴─────────────┘
```

### Left — Projects

Your recent projects appear here. Each project is a separate workspace with its own context and draft.

- **New project** — type a name and click *New project*. A name is required.
- **Select a project** — click a row to load its context and draft.
- **Last used** — updates when you select a project.

### Center — Current Preview

This is your draft message (the **Current Preview**). You can:

- **Generate Draft** — build a Status Update from saved context (requires context notes)
- **Edit freely** — change any wording before you send
- **Copy** — put the full draft on your clipboard (enabled when draft has text)

Status indicators above the preview:

- **Context saved** — context is present and saved
- **Draft ready** — preview has content
- **Last generated** — when you last generated for this project

Changes save automatically after a short pause (about half a second). You do not need to press Save.

### Right — Context

Paste or type background information for the current project: handoff notes, meeting bullets, open tasks, links, reminders.

You can also click **Import** to load a Graf-ID handoff file (`.json`, `.txt`, or `.md`). GrafiTalk converts it into the same labeled sections as manual context. See [Import from Graf-ID](#import-from-graf-id) below.

Context is saved per project and used when you generate a draft.

---

## Recommended workflow

Follow these steps for a typical status update:

### 1. Create or open a project

If this is a new client or topic, create a project with a clear name (for example `Mesencsi platform`).

If you already have the project, select it from the sidebar.

### 2. Add context

In the right panel, write what the update should be based on. Examples:

- What you finished this week
- What is blocked or waiting on the client
- What you plan to do next
- Anything you must not forget in the message

You can paste from other tools. Keep it rough — you will polish the final message in the draft panel.

### 3. Generate a draft

Click **Generate Draft** in the center toolbar.

Choose one of **five templates** in the action bar (Status Update, Client Update, Debug Report, Handover, Weekly Summary).

GrafiTalk uses a **deterministic template pipeline** (not AI). It:

- Reads your **saved context** for the selected project
- Structures or transforms notes into client-readable sections
- Saves the result as the latest draft for that project

**Generate Draft is disabled** until you add context notes.

### 4. Review and edit

Read the draft in the center panel. Change tone, add detail, remove sections, fix names.

This step is intentional. GrafiTalk assumes a human always reviews before anything goes out.

### 5. Copy and send elsewhere

Click **Copy**, then paste into email, Slack, Teams, or wherever you normally communicate.

GrafiTalk shows brief feedback when copy succeeds or fails.

---

## What the generated draft looks like

The Status Update pipeline produces structured output without email boilerplate. Examples:

**Structured context (prose + files):**

```text
Status update

Project:
Mesencsi platform

Focus Area:
QA handoff complete. README aligned with deployment steps.

Review Notes:
The above information was generated from the provided project context and should be reviewed before sending.
```

**Compact deployment note (transformed):**

```text
Status update

The webshop is ready for deployment, but the final release is currently waiting on the Barion production key.

Once the Barion key is available, the remaining deployment work should take approximately one day.

Current blocker:
Barion production approval / key access.
```

Output does **not** include `Hi team,`, `Please review before sending.`, or `Best regards` by default.

---

## Import from Graf-ID

If you use **Graf-ID** to track where you left off on a project, you can import its handoff into GrafiTalk instead of retyping context.

1. Select the GrafiTalk project for that client or workstream
2. In the **Context** panel, click **Import**
3. Choose a file exported from Graf-ID:
   - **JSON** (recommended) — structured handoff
   - **TXT** or **MD** — same information as labeled sections
4. GrafiTalk fills the context panel with sections like `Project:`, `Current status:`, `What changed:`, etc.
5. Edit anything that should not go to the client, then **Generate Draft** as usual

**Notes:**

- Import replaces the current context text for that project (autosaves immediately)
- The imported project name appears under `Project:` in context — it does not rename your GrafiTalk project
- Maximum file size is 256 KB
- Invalid or unsupported files show an error under the context panel

Sample files for testing: `examples/graf_id_project_context.sample.json` (in the repo). Full specification: [Graf-ID handoff](GRAF_ID_HANDOFF.md).

---

## Saving and persistence

| Data | Where it lives | When it saves |
|------|----------------|---------------|
| Project list | Local database | On create |
| Context notes | Per project in database | Auto-save while typing |
| Draft text | Per project in database | Auto-save while editing; also saved on Generate |

Switching projects saves any pending edits first, so you should not lose work when moving between clients.

---

## Buttons and features

- **Import** (Context panel) — load Graf-ID handoff JSON, TXT, or MD
- **Export** (toolbar) — save Current Preview as TXT, MD, JSON, or PDF
- **Archive** (sidebar) — hide a project from the active list without deleting its data
- **Restore** (archived section) — return an archived project to the active list
- **Rename** (sidebar) — change the display name of an active project
- **Read aloud** (speaker icon near Current Preview) — local text-to-speech when supported
- **About / diagnostics** (title bar) — version, database path, backup, read-aloud preference

Database backup and manual restore: [BACKUP_RESTORE.md](BACKUP_RESTORE.md)

Changing the template after editing a draft may ask for confirmation before replacing your manual changes.

---

## Tips

- **One project per client or workstream** keeps context and drafts separate.
- **Generate is safe to run again** — it replaces the draft for that project. Review before copying if you had manual edits you want to keep.
- **Empty context** — Generate Draft stays disabled until you add context notes.
- **Copy requires text** — if the draft area is empty, copy does nothing.

---

## Privacy

- Data stays on your machine in the app data folder (`grafitalk.db`).
- No cloud sync, no telemetry, no automatic sending in the MVP.
- Uninstalling or deleting app data would remove your projects; back up important drafts externally if needed.

---

## Getting help with the app

If something does not save, does not generate, or shows an error under the context or document panels, note the exact step and message. For development issues, see the [Developer guide](DEVELOPER_GUIDE.md).
