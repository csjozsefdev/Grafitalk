use crate::model::Project;
use crate::template::pretty_print::PrettyPrintContext;

pub fn push_heading(sections: &mut Vec<String>, title: &str) {
    sections.push(title.to_string());
    sections.push(String::new());
}

pub fn push_project_section(sections: &mut Vec<String>, project: &Project) {
    sections.push("Project:".to_string());
    sections.push(project.name.clone());
    sections.push(String::new());
}

pub fn push_client_section(sections: &mut Vec<String>, project: &Project) {
    if let Some(label) = project
        .client_label
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        sections.push("Client:".to_string());
        sections.push(label.to_string());
        sections.push(String::new());
    }
}

pub fn push_project_focus(sections: &mut Vec<String>, context: &PrettyPrintContext) {
    if let Some(focus) = context
        .project_focus
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        sections.push("Project focus:".to_string());
        sections.push(focus.to_string());
        sections.push(String::new());
    }
}

pub fn push_labeled_text(sections: &mut Vec<String>, label: &str, value: Option<&str>) {
    if let Some(text) = value.filter(|value| !value.is_empty()) {
        sections.push(format!("{label}:"));
        sections.push(text.to_string());
        sections.push(String::new());
    }
}

pub fn push_current_status(sections: &mut Vec<String>, context: &PrettyPrintContext) {
    push_labeled_text(sections, "Current status", context.current_status.as_deref());
}

pub fn push_context_summary(sections: &mut Vec<String>, context: &PrettyPrintContext) {
    if context.current_status.is_some() || context.has_structured_content() {
        return;
    }

    if let Some(fallback) = context
        .raw_fallback
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        sections.push("Summary:".to_string());
        sections.push(fallback.to_string());
        sections.push(String::new());
    }
}

pub fn push_bullet_section(sections: &mut Vec<String>, title: &str, items: &[String]) {
    if items.is_empty() {
        return;
    }

    sections.push(format!("{title}:"));
    sections.push(String::new());
    for item in items {
        sections.push(format!("* {item}"));
    }
    sections.push(String::new());
}

pub fn push_timeline(sections: &mut Vec<String>, context: &PrettyPrintContext) {
    push_labeled_text(sections, "Timeline", context.timeline.as_deref());
}

pub fn push_notes(sections: &mut Vec<String>, context: &PrettyPrintContext) {
    push_labeled_text(sections, "Notes", context.notes.as_deref());
}

pub fn push_files(sections: &mut Vec<String>, context: &PrettyPrintContext) {
    if context.files.is_empty() {
        return;
    }

    sections.push("Files Updated:".to_string());
    sections.push(String::new());
    for path in &context.files {
        sections.push(format!("* {path}"));
    }
    sections.push(String::new());
}

pub fn render_empty_shell(title: &str, project: &Project) -> String {
    let mut sections = vec![title.to_string(), String::new()];
    push_project_section(&mut sections, project);
    sections.push("Add project context before generating a draft.".to_string());
    sections.join("\n")
}
