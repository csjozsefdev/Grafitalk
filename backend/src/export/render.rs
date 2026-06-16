use std::io::BufWriter;

use chrono::Utc;
use printpdf::{BuiltinFont, Mm, PdfDocument, PdfLayerIndex, PdfPageIndex};
use serde::Serialize;

use crate::model::Project;
use crate::template::TemplateKind;

use super::error::ExportError;
use super::format::ExportFormat;
use super::model::{DraftExportDocument, SCHEMA_VERSION, SOURCE_GRAFITALK};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PreparedExport {
    pub suggested_filename: String,
    pub text: Option<String>,
    pub bytes: Option<Vec<u8>>,
}

pub fn build_export_document(
    project: &Project,
    template_kind: &str,
    draft_text: &str,
) -> Result<DraftExportDocument, ExportError> {
    if draft_text.trim().is_empty() {
        return Err(ExportError::EmptyDraft);
    }

    let kind = TemplateKind::parse(template_kind).map_err(ExportError::InvalidTemplateKind)?;
    let subject = format!("{} — {}", kind.title(), project.name.trim());

    Ok(DraftExportDocument {
        source: SOURCE_GRAFITALK.to_string(),
        schema_version: SCHEMA_VERSION.to_string(),
        project_name: project.name.trim().to_string(),
        template_kind: kind.as_str().to_string(),
        subject,
        body: draft_text.to_string(),
        exported_at: Utc::now().to_rfc3339(),
    })
}

pub fn prepare_export(
    project: &Project,
    template_kind: &str,
    draft_text: &str,
    format: ExportFormat,
) -> Result<PreparedExport, ExportError> {
    let document = build_export_document(project, template_kind, draft_text)?;
    let suggested_filename = suggested_filename(&document, format);

    match format {
        ExportFormat::Txt => Ok(PreparedExport {
            suggested_filename,
            text: Some(export_txt(&document)),
            bytes: None,
        }),
        ExportFormat::Md => Ok(PreparedExport {
            suggested_filename,
            text: Some(export_md(&document)),
            bytes: None,
        }),
        ExportFormat::Json => Ok(PreparedExport {
            suggested_filename,
            text: Some(export_json(&document)?),
            bytes: None,
        }),
        ExportFormat::Pdf => Ok(PreparedExport {
            suggested_filename,
            text: None,
            bytes: Some(export_pdf(&document)?),
        }),
    }
}

pub fn suggested_filename(document: &DraftExportDocument, format: ExportFormat) -> String {
    let project_slug = slugify(&document.project_name);
    let template_slug = slugify(&document.template_kind.replace('_', "-"));
    format!("grafitalk_{project_slug}_{template_slug}.{}", format.extension())
}

fn export_txt(document: &DraftExportDocument) -> String {
    document.body.clone()
}

fn export_md(document: &DraftExportDocument) -> String {
    format!(
        "# {}\n\n**Project:** {}\n\n**Template:** {}\n\n**Exported at:** {}\n\n---\n\n{}",
        document.subject,
        document.project_name,
        document.template_kind,
        document.exported_at,
        document.body
    )
}

fn export_json(document: &DraftExportDocument) -> Result<String, ExportError> {
    serde_json::to_string_pretty(document)
        .map_err(|err| ExportError::Serialization(err.to_string()))
}

fn export_pdf(document: &DraftExportDocument) -> Result<Vec<u8>, ExportError> {
    let (pdf, page1, layer1) = PdfDocument::new(
        &document.subject,
        Mm(210.0),
        Mm(297.0),
        "Draft export",
    );
    let font = pdf
        .add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|err| ExportError::PdfGeneration(err.to_string()))?;
    let font_bold = pdf
        .add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|err| ExportError::PdfGeneration(err.to_string()))?;

    let mut page = page1;
    let mut layer = layer1;
    let mut y = 280.0;
    let left = 20.0;
    let line_height = 5.0;
    let bottom_margin = 20.0;

    let mut current_layer = pdf.get_page(page).get_layer(layer);
    current_layer.use_text(&document.subject, 14.0, Mm(left), Mm(y), &font_bold);
    y -= line_height * 2.0;

    for meta in [
        format!("Project: {}", document.project_name),
        format!("Template: {}", document.template_kind),
        format!("Exported at: {}", document.exported_at),
    ] {
        if y < bottom_margin {
            (page, layer) = add_page(&pdf);
            current_layer = pdf.get_page(page).get_layer(layer);
            y = 280.0;
        }
        current_layer.use_text(&meta, 10.0, Mm(left), Mm(y), &font);
        y -= line_height;
    }

    y -= line_height;

    for line in wrap_pdf_lines(&document.body, 92) {
        if y < bottom_margin {
            (page, layer) = add_page(&pdf);
            current_layer = pdf.get_page(page).get_layer(layer);
            y = 280.0;
        }
        current_layer.use_text(&line, 10.0, Mm(left), Mm(y), &font);
        y -= line_height;
    }

    let mut buffer = Vec::new();
    pdf.save(&mut BufWriter::new(&mut buffer))
        .map_err(|err| ExportError::PdfGeneration(err.to_string()))?;

    Ok(buffer)
}

fn add_page(pdf: &printpdf::PdfDocumentReference) -> (PdfPageIndex, PdfLayerIndex) {
    pdf.add_page(Mm(210.0), Mm(297.0), "Draft export")
}

fn wrap_pdf_lines(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();

    for paragraph in text.lines() {
        if paragraph.trim().is_empty() {
            lines.push(String::new());
            continue;
        }

        let mut current = String::new();
        for word in paragraph.split_whitespace() {
            if current.is_empty() {
                current.push_str(word);
                continue;
            }

            if current.len() + 1 + word.len() > max_chars {
                lines.push(current);
                current = word.to_string();
            } else {
                current.push(' ');
                current.push_str(word);
            }
        }

        if !current.is_empty() {
            lines.push(current);
        }
    }

    lines
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut last_was_sep = false;

    for ch in value.trim().chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_was_sep = false;
        } else if !last_was_sep && !slug.is_empty() {
            slug.push('_');
            last_was_sep = true;
        }
    }

    let trimmed = slug.trim_matches('_').to_string();
    if trimmed.is_empty() {
        "draft".to_string()
    } else {
        trimmed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Project;

    fn sample_project() -> Project {
        Project {
            id: "p1".to_string(),
            name: "Mesencsi Webshop".to_string(),
            client_label: Some("Acme".to_string()),
            status: "active".to_string(),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
            last_used_at: None,
            archived_at: None,
        }
    }

    const SAMPLE_DRAFT: &str = "Status update\n\nProject:\nMesencsi\n\nCurrent status:\nReady.";

    #[test]
    fn rejects_empty_draft() {
        let err = build_export_document(&sample_project(), "status_update", "   ")
            .expect_err("empty");
        assert_eq!(err, ExportError::EmptyDraft);
    }

    #[test]
    fn exports_txt_as_body_only() {
        let prepared = prepare_export(
            &sample_project(),
            "status_update",
            SAMPLE_DRAFT,
            ExportFormat::Txt,
        )
        .expect("txt");

        assert_eq!(prepared.text.as_deref(), Some(SAMPLE_DRAFT));
        assert!(prepared
            .suggested_filename
            .ends_with("_status_update.txt"));
    }

    #[test]
    fn exports_md_with_subject_header() {
        let prepared = prepare_export(
            &sample_project(),
            "handover",
            SAMPLE_DRAFT,
            ExportFormat::Md,
        )
        .expect("md");

        let text = prepared.text.expect("text");
        assert!(text.starts_with("# Handover — Mesencsi Webshop"));
        assert!(text.contains(SAMPLE_DRAFT));
    }

    #[test]
    fn exports_json_with_schema_fields() {
        let prepared = prepare_export(
            &sample_project(),
            "debug_report",
            SAMPLE_DRAFT,
            ExportFormat::Json,
        )
        .expect("json");

        let text = prepared.text.expect("text");
        assert!(text.contains("\"source\": \"grafitalk\""));
        assert!(text.contains("\"schema_version\": \"0.1\""));
        assert!(text.contains("\"template_kind\": \"debug_report\""));
        assert!(text.contains("\"body\":"));
    }

    #[test]
    fn exports_pdf_bytes() {
        let prepared = prepare_export(
            &sample_project(),
            "status_update",
            SAMPLE_DRAFT,
            ExportFormat::Pdf,
        )
        .expect("pdf");

        let bytes = prepared.bytes.expect("bytes");
        assert!(bytes.starts_with(b"%PDF"));
        assert!(bytes.len() > 100);
    }

    #[test]
    fn unicode_and_long_lines_are_supported() {
        let draft = format!(
            "Status update\n\nNotes:\nUnicode — β\n\n{}",
            "Long line ".repeat(40)
        );
        let prepared = prepare_export(
            &sample_project(),
            "weekly_summary",
            &draft,
            ExportFormat::Pdf,
        )
        .expect("unicode pdf");

        assert!(prepared.bytes.unwrap().starts_with(b"%PDF"));
    }

    #[test]
    fn wrap_pdf_lines_breaks_long_text() {
        let lines = wrap_pdf_lines("word ".repeat(30).trim(), 20);
        assert!(lines.len() > 1);
    }
}
