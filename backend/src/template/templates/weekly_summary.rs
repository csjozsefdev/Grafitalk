use crate::model::Project;

use super::common::{
    push_bullet_section, push_context_summary, push_heading, push_labeled_text,
    push_project_section,
};
use super::super::pretty_print::PrettyPrintContext;

pub fn render(project: &Project, context: &PrettyPrintContext) -> String {
    let mut sections = Vec::new();
    push_heading(&mut sections, "Weekly summary");
    push_project_section(&mut sections, project);
    push_labeled_text(&mut sections, "Week overview", context.current_status.as_deref());
    push_context_summary(&mut sections, context);
    push_bullet_section(&mut sections, "Highlights", &context.completed_work);
    push_labeled_text(&mut sections, "Timeline / estimate", context.timeline.as_deref());
    push_bullet_section(&mut sections, "Blockers this week", &context.blockers);
    push_bullet_section(&mut sections, "Priorities for next week", &context.next_steps);
    sections.join("\n")
}
