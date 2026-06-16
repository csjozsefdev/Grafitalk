# GrafiTalk — Draft export schema (v0.1)

GrafiTalk exports the **Current Preview** draft body using this schema for JSON exports. TXT and MD are derived views of the same document.

## JSON envelope

```json
{
  "source": "grafitalk",
  "schema_version": "0.1",
  "project_name": "Mesencsi webshop",
  "template_kind": "status_update",
  "subject": "Status update — Mesencsi webshop",
  "body": "Status update\n\nProject:\n...",
  "exported_at": "2026-06-07T12:34:56Z"
}
```

## Fields

| Field | Required | Description |
|-------|----------|-------------|
| `source` | yes | Always `"grafitalk"` |
| `schema_version` | yes | Contract version (`"0.1"`) |
| `project_name` | yes | Active GrafiTalk project name |
| `template_kind` | yes | One of `status_update`, `client_update`, `debug_report`, `handover`, `weekly_summary` |
| `subject` | yes | Human-readable title (`{Template title} — {Project name}`) |
| `body` | yes | Full draft text from Current Preview |
| `exported_at` | yes | RFC 3339 UTC timestamp at export time |

## Format mapping

| Export format | Content |
|---------------|---------|
| `.txt` | `body` only (UTF-8) |
| `.md` | Markdown wrapper with subject, metadata, horizontal rule, then `body` |
| `.json` | Schema envelope above (pretty-printed) |
| `.pdf` | Printable PDF of subject, metadata, and wrapped `body` |

## Validation rules

- Empty draft body is rejected (`Nothing to export yet.`).
- Export failure does **not** modify stored draft text.
- Unknown export extensions are rejected at prepare time.

## Versioning

- Increment `schema_version` for breaking JSON field changes.
- GrafiTalk must accept only supported schema versions when import/export symmetry is added later.

## Reference implementation

- Model: [`backend/src/export/model.rs`](../backend/src/export/model.rs)
- Renderers: [`backend/src/export/render.rs`](../backend/src/export/render.rs)
