use crate::model::Project;

use super::common::{
    push_bullet_section, push_context_summary, push_files, push_heading, push_labeled_text,
    push_project_focus, push_project_section,
};
use super::super::pretty_print::PrettyPrintContext;

pub fn render(project: &Project, context: &PrettyPrintContext) -> String {
    let mut sections = Vec::new();
    push_heading(&mut sections, "Debug report");
    push_project_section(&mut sections, project);
    push_project_focus(&mut sections, context);
    push_bullet_section(&mut sections, "Issue / blocker", &context.blockers);
    push_labeled_text(&mut sections, "Observed state", context.current_status.as_deref());
    push_context_summary(&mut sections, context);
    push_bullet_section(&mut sections, "Changes attempted", &context.completed_work);
    push_files(&mut sections, context);
    push_labeled_text(&mut sections, "Investigation notes", context.notes.as_deref());
    push_bullet_section(&mut sections, "Suggested fix", &context.next_steps);
    sections.join("\n")
}
