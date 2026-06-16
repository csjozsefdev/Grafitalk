# Graf-ID → GrafiTalk handoff guide

**Version 0.1 · Import works in GrafiTalk today**

Graf-ID helps you remember **where you left off** on a project. GrafiTalk helps you decide **what to tell the client**. This guide describes the file you pass between them.

Nothing is sent automatically. You export from Graf-ID, import into GrafiTalk, generate a draft, review it, and copy it yourself.

---

## The idea in one minute

You finish a coding session in Graf-ID. Before writing a client update, you export a small handoff file — a snapshot of status, changes, blockers, and next steps.

In GrafiTalk you open the matching project, click **Import** in the Context panel, and pick that file. Your notes appear as labeled sections you can edit. Then you generate a draft the way you always would.

The two apps never share a database. They only exchange a plain file you can open, read, and fix by hand if needed.

---

## Step by step

### In Graf-ID

1. Open the project you were working on.
2. Export a handoff file (JSON is best; plain text or Markdown also work).
3. Save it somewhere you can find — Desktop, project folder, etc.

### In GrafiTalk

1. Select or create the project for that client.
2. Open the **Context** panel on the right.
3. Click **Import** and choose the file (`.json`, `.txt`, or `.md`).
4. Read the imported text. Remove anything the client should not see.
5. Pick a template, click **Generate Draft**, review, then **Copy** or **Export**.

The name inside the file (`project_name`) shows up under **Project:** in context. It does **not** rename the GrafiTalk project you already picked in the sidebar.

---

## What a handoff file looks like

Whether you use plain text, Markdown, or JSON, the **meaning** is the same. Here is the shape GrafiTalk expects:

```text
Project:
Mesencsi webshop

Current status:
Deployment is nearly ready.

What changed:
- Manual QA completed.
- Barion sandbox flow tested.
- Production deployment is waiting for Barion production credentials.

Current blocker:
- Barion production key / merchant approval.

Next step:
- Deploy once Barion credentials are available.

Estimated time:
Approximately 1 day after credentials are received.

Notes:
This context is for draft generation only. GrafiTalk must not send messages automatically.

Files updated:
- src/checkout.rs
- README.md
```

You do not need every section. Skip anything that is empty — GrafiTalk will not create blank blocks.

### What each section is for

**Project** — Which workstream this handoff belongs to. Usually the client or product name.

**Current status** — One short summary: where things stand right now.

**What changed** — Bullet list of work done since the last update. In JSON this field is called `changes`.

**Current blocker** — What is stopping progress. Waiting on the client, missing credentials, an open bug.

**Next step** — What you plan to do next, in order.

**Estimated time** — Rough timing in plain language (“about two days”, “waiting on client review”).

**Notes** — Anything else: reminders, links, caveats for yourself before generating.

**Files updated** — Optional list of paths touched (`src/App.tsx`, `README.md`). Useful for technical templates; omit for non-technical clients.

---

## Three file formats

| Format | When to use |
|--------|-------------|
| **JSON** | Best for Graf-ID export — structured, easy to validate |
| **TXT** | Easy to read and edit in any editor |
| **MD** | Same as TXT, but with `#` headings instead of `Project:` labels |

GrafiTalk accepts all three. A good handoff in JSON, TXT, and MD should import to the same context.

**Markdown tip:** headings like `# Project` and `## Current status` are converted automatically. You can also write `**Notes**` as a section title.

**Label aliases:** GrafiTalk understands small variations — for example `Completed work:` instead of `What changed:`, or `Timeline:` instead of `Estimated time:`.

**Size limit:** files must be under **256 KB**. Empty files are rejected.

Sample files to try: [`examples/graf_id_project_context.sample.json`](../examples/graf_id_project_context.sample.json) (and matching `.txt` / `.md` in the same folder).

---

## JSON export (recommended for Graf-ID)

If you are building export in Graf-ID, use this flat JSON shape:

```json
{
  "source": "graf-id",
  "schema_version": "0.1",
  "project_name": "Mesencsi webshop",
  "current_status": "Deployment is nearly ready.",
  "changes": [
    "Manual QA completed.",
    "Barion sandbox flow tested.",
    "Production deployment is waiting for Barion production credentials."
  ],
  "blockers": [
    "Barion production key / merchant approval."
  ],
  "next_steps": [
    "Deploy once Barion credentials are available."
  ],
  "estimated_time": "Approximately 1 day after credentials are received.",
  "notes": "This context is for draft generation only.",
  "files": [
    "src/checkout.rs",
    "README.md"
  ]
}
```

**Required fields**

- `"source": "graf-id"` — identifies the file as a Graf-ID handoff
- `"schema_version": "0.1"` — contract version GrafiTalk supports today
- `"project_name"` — non-empty string

Everything else is optional. Empty lists can be omitted. Extra JSON keys GrafiTalk does not know about are ignored.

**JSON field → context section**

| In JSON | Appears in context as |
|---------|------------------------|
| `project_name` | Project |
| `current_status` | Current status |
| `changes` | What changed (bullets) |
| `blockers` | Current blocker (bullets) |
| `next_steps` | Next step (bullets) |
| `estimated_time` | Estimated time |
| `notes` | Notes |
| `files` | Files updated (bullets) |

---

## What GrafiTalk does with the file

1. Reads the file by extension (`.json`, `.txt`, `.md`).
2. Parses it into structured fields internally.
3. Writes **labeled plain text** into the Context panel — not raw JSON.
4. Autosaves like typed notes.

You always get editable text you can change before generating. GrafiTalk does not upload the file anywhere.

---

## Older Graf-ID exports still work

GrafiTalk tries to be forgiving during migration. You may see warnings in the developer console, but import should succeed.

**Old field names (import only — use the new names in new exports)**

| Old name | Use instead |
|----------|-------------|
| `spec_version` or `version` | `schema_version` |
| `completed_work` | `changes` |
| `timeline` | `estimated_time` |
| `project_focus` | split into `project_name` / `current_status` / `notes` depending on what else is present |

**Missing `source` field** — accepted if the file clearly looks like a Graf-ID handoff (version `0.1` plus real context fields). GrafiTalk fills in `"graf-id"` automatically.

**Full project dump** — Graf-ID can also export its internal JSON with `project` and `resume_panel` blocks. GrafiTalk can import this for compatibility, but it is **not recommended** for new work:

- No “what changed” list is pulled from history — only resume summary text
- Local paths, session IDs, and categories are stripped out
- Prefer the flat v0.1 JSON above for client-ready handoffs

Example of the old full format: [`examples/graf_id_full_project_export.sample.json`](../examples/graf_id_full_project_export.sample.json).

---

## Rules Graf-ID should follow when exporting

- Always set `"source": "graf-id"` and `"schema_version": "0.1"`.
- Always include a non-empty `project_name`.
- Prefer flat JSON over dumping the whole internal project.
- Do not put absolute paths like `C:\Users\...` in fields meant for the client.
- Optionally offer TXT/MD using the same section labels as the sample text file.

GrafiTalk includes reference TXT/MD serializers in the repo for parity testing against [`graf_id_project_context.sample.txt`](../examples/graf_id_project_context.sample.txt).

---

## When import fails

Common reasons and what they mean in plain language:

| Message (approx.) | What went wrong |
|-------------------|-----------------|
| Missing version field | JSON needs `schema_version: "0.1"` |
| Unsupported schema version | GrafiTalk only supports `0.1` today |
| Conflicting version fields | `schema_version` and `spec_version` disagree |
| Missing project name | `project_name` is empty or absent |
| Invalid source | `source` is set to something other than `graf-id` |
| Empty file / empty content | Nothing usable to import |
| File too large | Over 256 KB |
| Full export missing content | Old-style dump has no summary to import |

Fix the file or re-export from Graf-ID, then try Import again.

---

## Privacy

Handoff files can contain client names, blockers, and file paths. Treat them like local work notes.

- GrafiTalk stores imported text only in your local database.
- Nothing is sent to the cloud during import.
- Full-export import removes absolute paths and internal IDs on purpose.
- You are still responsible for what you copy to email or chat after review.

---

## Design choices (why it works this way)

**Human-readable first** — you should be able to open the file in Notepad and understand it.

**Files, not databases** — the tools stay independent; either side can evolve without breaking the other.

**No invented facts** — the handoff describes your dev reality; templates turn it into client language at generate time.

**Versioned contract** — when the format changes, `schema_version` bumps (e.g. to `0.2`). GrafiTalk will reject versions it does not support yet.

**Copy-only from GrafiTalk** — import fills context; sending always happens outside the app.

---

## For developers

Field tables, detection order, error codes, and code paths: [GRAF_ID_EXPORT_SCHEMA.md](GRAF_ID_EXPORT_SCHEMA.md)

Related docs:

- [User guide — Import from Graf-ID](USER_GUIDE.md#import-from-graf-id)
- [Architecture — Graf-ID integration](ARCHITECTURE.md#graf-id-integration)
- [QA checklist — Graf-ID import](QA_CHECKLIST.md#graf-id-import)
