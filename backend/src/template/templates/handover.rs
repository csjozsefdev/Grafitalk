use crate::model::Project;

use super::common::{
    push_bullet_section, push_context_summary, push_files, push_heading, push_labeled_text,
    push_project_focus, push_project_section,
};
use super::super::pretty_print::PrettyPrintContext;

pub fn render(project: &Project, context: &PrettyPrintContext) -> String {
    let mut sections = Vec::new();
    push_heading(&mut sections, "Handover");
    push_project_section(&mut sections, project);
    push_project_focus(&mut sections, context);
    push_labeled_text(&mut sections, "State at handover", context.current_status.as_deref());
    push_context_summary(&mut sections, context);
    push_bullet_section(&mut sections, "Work completed", &context.completed_work);
    push_files(&mut sections, context);
    push_bullet_section(&mut sections, "Open items for next owner", &context.next_steps);
    push_bullet_section(&mut sections, "Outstanding blockers", &context.blockers);
    push_labeled_text(&mut sections, "Handover notes", context.notes.as_deref());
    sections.join("\n")
}
