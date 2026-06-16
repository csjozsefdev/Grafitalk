# Graf-ID export schema — technical reference (v0.1)

**Start with the human guide:** [GRAF_ID_HANDOFF.md](GRAF_ID_HANDOFF.md)

This page is for implementers — field names, parsers, and validation. GrafiTalk owns the contract; Graf-ID implements export in its own repo.

---

## Flat JSON envelope (canonical)

```json
{
  "source": "graf-id",
  "schema_version": "0.1",
  "project_name": "Mesencsi webshop",
  "current_status": "Deployment is nearly ready.",
  "changes": ["Manual QA completed."],
  "blockers": ["Barion production key / merchant approval."],
  "next_steps": ["Deploy once Barion credentials are available."],
  "estimated_time": "Approximately 1 day after credentials are received.",
  "notes": "Optional free-form notes.",
  "files": ["src/checkout.rs", "README.md"]
}
```

---

## Fields

| Field | Required | Type | Context panel |
|-------|----------|------|---------------|
| `source` | yes | string | — (must be `"graf-id"`) |
| `schema_version` | yes | string | — (must be `"0.1"`) |
| `project_name` | yes | string | `Project:` |
| `current_status` | no | string | `Current status:` |
| `changes` | no | string[] | `What changed:` |
| `blockers` | no | string[] | `Current blocker:` |
| `next_steps` | no | string[] | `Next step:` |
| `estimated_time` | no | string | `Estimated time:` |
| `notes` | no | string | `Notes:` |
| `files` | no | string[] | `Files updated:` |

Unknown keys ignored. Empty optional fields omitted in output. `project_name` does not rename the GrafiTalk SQLite project.

---

## Legacy aliases (import only)

| Alias | Maps to |
|-------|---------|
| `spec_version`, `version` | `schema_version` |
| `completed_work` | `changes` |
| `timeline` | `estimated_time` |
| `project_focus` | see [handoff guide](GRAF_ID_HANDOFF.md#older-graf-id-exports-still-work) |
| missing `source` | `"graf-id"` when handoff-shaped |

All version fields present must match. Numbers coerced to strings.

---

## Text / Markdown labels

Case-insensitive; trailing `:` optional.

| Label | Aliases |
|-------|---------|
| Project | Project focus |
| Current status | — |
| What changed | Completed work |
| Current blocker | Blocker, Blockers |
| Next step | Next steps |
| Estimated time | Timeline |
| Notes | — |
| Files updated | Files |

`.md`: `#` / `##` / `###` → labels. `.txt` / `.md`: parsed by `parse_pretty_print`.

---

## JSON detection order

1. UTF-8 BOM stripped; must be JSON object
2. Handoff-shaped → flat v0.1 parser
3. Full project export → normalizer (legacy warning)
4. Else `project` key → full export parser
5. Else strict flat parser

Handoff-shaped: `source == "graf-id"`, or non-empty `project_name`, or legacy fields without wrong `source`.

---

## Full project export (compatibility)

Not recommended for new exports. Detected when `project` contains `name`, `summary_preview`, `latest_session`, or `resume_panel.startup_summary`.

Extracts: name, headline, sanitized summary, blocker, next step, relative file paths.  
Strips: absolute paths, IDs, categories, session metadata.

Sample: [graf_id_full_project_export.sample.json](../examples/graf_id_full_project_export.sample.json)

---

## Limits and errors

| Constraint | Value |
|------------|-------|
| Max size | 256 KB |
| Supported version | `"0.1"` only |

Errors surface in UI as import failures; legacy warnings go to `[grafitalk]` stderr.

---

## Code and samples

| Piece | Path |
|-------|------|
| Flat JSON + TXT/MD serializers | `backend/src/template/graf_id_handoff.rs` |
| Full export normalizer | `backend/src/template/graf_id_full_export.rs` |
| Import entry | `backend/src/import/parser.rs` |
| IPC | `import_graf_id_handoff` |
| Samples | `examples/graf_id_project_context.sample.*` |

Draft exports from GrafiTalk use `source: "grafitalk"` — different schema: [DRAFT_EXPORT_SCHEMA.md](DRAFT_EXPORT_SCHEMA.md).
