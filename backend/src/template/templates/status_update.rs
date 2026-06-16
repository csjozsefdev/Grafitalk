use crate::model::Project;

use super::common::{
    push_bullet_section, push_client_section, push_context_summary, push_files, push_heading,
    push_labeled_text, push_project_focus, push_project_section,
};
use super::super::pretty_print::PrettyPrintContext;

pub fn render(project: &Project, context: &PrettyPrintContext) -> String {
    let mut sections = Vec::new();
    push_heading(&mut sections, "Status update");
    push_project_section(&mut sections, project);
    push_client_section(&mut sections, project);
    push_project_focus(&mut sections, context);
    push_labeled_text(&mut sections, "Current status", context.current_status.as_deref());
    push_context_summary(&mut sections, context);
    push_bullet_section(&mut sections, "Completed work", &context.completed_work);
    push_bullet_section(&mut sections, "Current blocker", &context.blockers);
    push_bullet_section(&mut sections, "Next step", &context.next_steps);
    push_labeled_text(&mut sections, "Timeline", context.timeline.as_deref());
    push_labeled_text(&mut sections, "Notes", context.notes.as_deref());
    push_files(&mut sections, context);
    sections.join("\n")
}
