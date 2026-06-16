use super::graf_id_handoff::GrafIdHandoff;
use super::note_transform::try_parse_compact_note;

const FILE_EXTENSIONS: &[&str] = &[
    ".tsx", ".ts", ".jsx", ".js", ".rs", ".md", ".json", ".css", ".html", ".vue", ".py",
    ".go", ".sql", ".yaml", ".yml", ".toml", ".txt",
];

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PrettyPrintContext {
    pub project_focus: Option<String>,
    pub current_status: Option<String>,
    pub completed_work: Vec<String>,
    pub blockers: Vec<String>,
    pub next_steps: Vec<String>,
    pub timeline: Option<String>,
    pub notes: Option<String>,
    pub files: Vec<String>,
    pub raw_fallback: Option<String>,
}

impl PrettyPrintContext {
    pub fn is_empty(&self) -> bool {
        self.project_focus.is_none()
            && self.current_status.is_none()
            && self.completed_work.is_empty()
            && self.blockers.is_empty()
            && self.next_steps.is_empty()
            && self.timeline.is_none()
            && self.notes.is_none()
            && self.files.is_empty()
            && self.raw_fallback.is_none()
    }

    pub fn has_structured_content(&self) -> bool {
        self.current_status.is_some()
            || !self.completed_work.is_empty()
            || !self.blockers.is_empty()
            || !self.next_steps.is_empty()
            || self.timeline.is_some()
            || self.notes.is_some()
            || !self.files.is_empty()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SectionKind {
    None,
    CurrentStatus,
    CompletedWork,
    Blockers,
    NextSteps,
    Timeline,
    Notes,
    Files,
    LegacyFocus,
    Project,
}

pub fn parse_pretty_print(raw: &str) -> PrettyPrintContext {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return PrettyPrintContext::default();
    }

    if !trimmed.contains('\n') {
        if let Some(context) = try_parse_compact_note(trimmed) {
            return context;
        }
    }

    let mut context = PrettyPrintContext::default();
    let mut section = SectionKind::None;
    let mut current_status_lines: Vec<String> = Vec::new();
    let mut timeline_lines: Vec<String> = Vec::new();
    let mut notes_lines: Vec<String> = Vec::new();
    let mut project_focus_lines: Vec<String> = Vec::new();
    let mut legacy_lines: Vec<String> = Vec::new();

    for line in trimmed.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(next_section) = detect_section_header(line) {
            flush_current_status(&mut context, &mut current_status_lines);
            flush_timeline(&mut context, &mut timeline_lines);
            flush_notes(&mut context, &mut notes_lines);
            flush_project_focus(&mut context, &mut project_focus_lines);
            section = next_section;
            continue;
        }

        if should_apply_file_heuristics(section) {
            if let Some(path) = extract_bullet_path(line) {
                context.files.push(path);
                continue;
            }

            if looks_like_file_path(line) {
                context.files.push(line.to_string());
                continue;
            }
        }

        match section {
            SectionKind::CurrentStatus => current_status_lines.push(line.to_string()),
            SectionKind::CompletedWork => push_list_item(&mut context.completed_work, line),
            SectionKind::Blockers => push_list_item(&mut context.blockers, line),
            SectionKind::NextSteps => push_list_item(&mut context.next_steps, line),
            SectionKind::Timeline => timeline_lines.push(line.to_string()),
            SectionKind::Notes => notes_lines.push(line.to_string()),
            SectionKind::Files => {
                if looks_like_file_path(line) {
                    context.files.push(line.to_string());
                } else {
                    push_list_item(&mut context.files, line);
                }
            }
            SectionKind::LegacyFocus => legacy_lines.push(line.to_string()),
            SectionKind::Project => project_focus_lines.push(line.to_string()),
            SectionKind::None => legacy_lines.push(line.to_string()),
        }
    }

    flush_current_status(&mut context, &mut current_status_lines);
    flush_timeline(&mut context, &mut timeline_lines);
    flush_notes(&mut context, &mut notes_lines);
    flush_project_focus(&mut context, &mut project_focus_lines);

    if !legacy_lines.is_empty() {
        let legacy = legacy_lines.join("\n");
        if context.project_focus.is_none() && context.has_structured_content() {
            context.project_focus = Some(legacy);
        } else if context.raw_fallback.is_none() {
            context.raw_fallback = Some(legacy);
        }
    }

    if context.is_empty() && !trimmed.is_empty() {
        context.raw_fallback = Some(trimmed.to_string());
    }

    context.files.sort_unstable();
    context.files.dedup();
    context
}

pub fn pretty_print_to_context_text(context: &PrettyPrintContext) -> String {
    let mut sections = Vec::new();

    if let Some(focus) = context
        .project_focus
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        sections.push(format!("Project:\n{}", focus.trim()));
    }

    if let Some(status) = context
        .current_status
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        sections.push(format!("Current status:\n{}", status.trim()));
    }

    if !context.completed_work.is_empty() {
        sections.push(render_bullet_section("What changed", &context.completed_work));
    }

    if !context.blockers.is_empty() {
        sections.push(render_bullet_section("Current blocker", &context.blockers));
    }

    if !context.next_steps.is_empty() {
        sections.push(render_bullet_section("Next step", &context.next_steps));
    }

    if let Some(timeline) = context
        .timeline
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        sections.push(format!("Estimated time:\n{}", timeline.trim()));
    }

    if let Some(notes) = context
        .notes
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        sections.push(format!("Notes:\n{}", notes.trim()));
    }

    if !context.files.is_empty() {
        sections.push(render_bullet_section("Files updated", &context.files));
    }

    if let Some(fallback) = context
        .raw_fallback
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        if !context.has_structured_content() && context.project_focus.is_none() {
            sections.push(format!("Notes:\n{}", fallback.trim()));
        }
    }

    sections.join("\n\n")
}

pub fn from_graf_id_handoff(handoff: &GrafIdHandoff) -> PrettyPrintContext {
    let project_name = handoff.project_name.trim();
    PrettyPrintContext {
        project_focus: if project_name.is_empty() {
            None
        } else {
            Some(project_name.to_string())
        },
        current_status: handoff
            .current_status
            .as_ref()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty()),
        completed_work: handoff
            .changes
            .iter()
            .map(|item| item.trim().to_string())
            .filter(|item| !item.is_empty())
            .collect(),
        blockers: handoff
            .blockers
            .iter()
            .map(|item| item.trim().to_string())
            .filter(|item| !item.is_empty())
            .collect(),
        next_steps: handoff
            .next_steps
            .iter()
            .map(|item| item.trim().to_string())
            .filter(|item| !item.is_empty())
            .collect(),
        timeline: handoff
            .estimated_time
            .as_ref()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty()),
        notes: handoff
            .notes
            .as_ref()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty()),
        files: handoff
            .files
            .iter()
            .map(|item| item.trim().to_string())
            .filter(|item| !item.is_empty())
            .collect(),
        raw_fallback: None,
    }
}

fn flush_project_focus(context: &mut PrettyPrintContext, lines: &mut Vec<String>) {
    if lines.is_empty() {
        return;
    }

    context.project_focus = Some(lines.join("\n"));
    lines.clear();
}

fn flush_current_status(context: &mut PrettyPrintContext, lines: &mut Vec<String>) {
    if lines.is_empty() {
        return;
    }

    context.current_status = Some(lines.join("\n"));
    lines.clear();
}

fn flush_timeline(context: &mut PrettyPrintContext, lines: &mut Vec<String>) {
    if lines.is_empty() {
        return;
    }

    context.timeline = Some(lines.join("\n"));
    lines.clear();
}

fn flush_notes(context: &mut PrettyPrintContext, lines: &mut Vec<String>) {
    if lines.is_empty() {
        return;
    }

    context.notes = Some(lines.join("\n"));
    lines.clear();
}

fn push_list_item(items: &mut Vec<String>, line: &str) {
    let trimmed = line
        .trim()
        .trim_start_matches('*')
        .trim_start_matches('-')
        .trim_start_matches('•')
        .trim();
    if !trimmed.is_empty() {
        items.push(trimmed.to_string());
    }
}

fn render_bullet_section(title: &str, items: &[String]) -> String {
    let mut block = format!("{title}:\n");
    for item in items {
        block.push_str("- ");
        block.push_str(item.trim());
        block.push('\n');
    }
    block.trim_end().to_string()
}

fn should_apply_file_heuristics(section: SectionKind) -> bool {
    matches!(
        section,
        SectionKind::None | SectionKind::LegacyFocus | SectionKind::Files | SectionKind::Project
    )
}

fn detect_section_header(line: &str) -> Option<SectionKind> {
    let normalized = line.trim_end_matches(':').trim().to_ascii_lowercase();
    match normalized.as_str() {
        "current status" => Some(SectionKind::CurrentStatus),
        "what changed" | "completed work" => Some(SectionKind::CompletedWork),
        "current blocker" | "blocker" | "blockers" => Some(SectionKind::Blockers),
        "next step" | "next steps" => Some(SectionKind::NextSteps),
        "estimated time" | "timeline" => Some(SectionKind::Timeline),
        "notes" => Some(SectionKind::Notes),
        "files updated" | "files" => Some(SectionKind::Files),
        "focus area" | "context" => Some(SectionKind::LegacyFocus),
        "project" | "project focus" => Some(SectionKind::Project),
        _ => None,
    }
}

fn extract_bullet_path(line: &str) -> Option<String> {
    let trimmed = line.trim();
    let rest = trimmed
        .strip_prefix('*')
        .or_else(|| trimmed.strip_prefix('-'))
        .or_else(|| trimmed.strip_prefix('•'))
        .map(str::trim)?;

    if looks_like_file_path(rest) {
        Some(rest.to_string())
    } else {
        None
    }
}

pub(crate) fn looks_like_file_path(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return false;
    }

    if trimmed.contains(" / ") || trimmed.contains(" \\ ") {
        return false;
    }

    if FILE_EXTENSIONS
        .iter()
        .any(|ext| trimmed.to_ascii_lowercase().ends_with(ext))
    {
        return true;
    }

    if trimmed.contains('/') || trimmed.contains('\\') {
        let segments: Vec<&str> = trimmed
            .split(&['/', '\\'][..])
            .filter(|segment| !segment.is_empty())
            .collect();
        return segments.len() >= 2;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::fixtures::{
        COMPACT_DEPLOYMENT_HU, GRAF_ID_LABELED_TEXT, GRAF_ID_SAMPLE_JSON, NARRATIVE_WITH_FILES,
        UNSTRUCTURED_PROSE,
    };
    use crate::template::graf_id_handoff::parse_handoff_json;

    #[test]
    fn parses_graf_id_labeled_text_into_all_fields() {
        let context = parse_pretty_print(GRAF_ID_LABELED_TEXT);

        assert_eq!(
            context.current_status.as_deref(),
            Some("Deployment is nearly ready.")
        );
        assert_eq!(context.completed_work.len(), 3);
        assert_eq!(context.blockers.len(), 1);
        assert_eq!(context.next_steps.len(), 1);
        assert!(context.timeline.is_some());
        assert!(context.notes.is_some());
        assert_eq!(
            context.project_focus.as_deref(),
            Some("Mesencsi webshop")
        );
        assert!(context.raw_fallback.is_none());
    }

    #[test]
    fn unstructured_prose_becomes_raw_fallback() {
        let context = parse_pretty_print(UNSTRUCTURED_PROSE);

        assert_eq!(context.raw_fallback.as_deref(), Some(UNSTRUCTURED_PROSE));
        assert!(context.current_status.is_none());
    }

    #[test]
    fn parses_mixed_sections_and_files() {
        let context = parse_pretty_print(NARRATIVE_WITH_FILES);

        assert_eq!(
            context.project_focus.as_deref(),
            Some("Mobile app for home developmental routines.")
        );
        assert_eq!(context.files, vec!["app/(tabs)/index.tsx".to_string()]);
        assert!(context.raw_fallback.is_none());
    }

    #[test]
    fn parses_multiple_bullets_per_section() {
        let raw = "\
Current status:
Waiting on approval.

What changed:
- Item one.
- Item two.

Blockers:
* Blocker one.
* Blocker two.";

        let context = parse_pretty_print(raw);

        assert_eq!(context.completed_work.len(), 2);
        assert_eq!(context.blockers.len(), 2);
    }

    #[test]
    fn section_labels_are_case_insensitive() {
        let raw = "\
CURRENT STATUS:
Ready.

WHAT CHANGED:
- Done.";

        let context = parse_pretty_print(raw);

        assert_eq!(context.current_status.as_deref(), Some("Ready."));
        assert_eq!(context.completed_work, vec!["Done.".to_string()]);
    }

    #[test]
    fn compact_deployment_note_fills_structured_fields() {
        let context = parse_pretty_print(COMPACT_DEPLOYMENT_HU);

        assert!(context
            .current_status
            .as_deref()
            .unwrap()
            .contains("The webshop is ready for deployment"));
        assert!(!context.blockers.is_empty());
        assert!(context.timeline.is_some());
        assert!(!context.next_steps.is_empty());
    }

    #[test]
    fn round_trip_omits_empty_sections() {
        let handoff = parse_handoff_json(GRAF_ID_SAMPLE_JSON).expect("parse sample");
        let context = from_graf_id_handoff(&handoff);
        let text = pretty_print_to_context_text(&context);

        assert!(text.contains("Current status:"));
        assert!(text.contains("What changed:"));
        assert!(!text.contains("Files updated:"));
    }

    #[test]
    fn from_graf_id_handoff_matches_labeled_parse() {
        let handoff = parse_handoff_json(GRAF_ID_SAMPLE_JSON).expect("parse sample");
        let from_handoff = from_graf_id_handoff(&handoff);
        let from_text = parse_pretty_print(GRAF_ID_LABELED_TEXT);

        assert_eq!(
            from_handoff.current_status, from_text.current_status,
            "current_status"
        );
        assert_eq!(from_handoff.completed_work, from_text.completed_work);
        assert_eq!(from_handoff.project_focus, from_text.project_focus);
        assert_eq!(from_handoff.blockers, from_text.blockers);
        assert_eq!(from_handoff.next_steps, from_text.next_steps);
        assert_eq!(from_handoff.timeline, from_text.timeline);
    }
}
