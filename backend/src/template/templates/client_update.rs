use crate::model::Project;

use super::common::{
    push_bullet_section, push_client_section, push_context_summary, push_heading,
    push_labeled_text, push_project_section,
};
use super::super::pretty_print::PrettyPrintContext;

pub fn render(project: &Project, context: &PrettyPrintContext) -> String {
    let mut sections = Vec::new();
    push_heading(&mut sections, "Client update");
    push_project_section(&mut sections, project);
    push_client_section(&mut sections, project);
    push_labeled_text(&mut sections, "Where things stand", context.current_status.as_deref());
    push_context_summary(&mut sections, context);
    push_bullet_section(&mut sections, "Progress", &context.completed_work);
    push_bullet_section(&mut sections, "Next milestone", &context.next_steps);
    push_labeled_text(&mut sections, "Expected timing", context.timeline.as_deref());
    sections.join("\n")
}
