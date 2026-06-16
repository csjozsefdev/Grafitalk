use super::pretty_print::PrettyPrintContext;

/// Parses compact single-line freelancer notes into structured fields (no AI).
pub fn try_parse_compact_note(raw: &str) -> Option<PrettyPrintContext> {
    let normalized = normalize_inline_note(raw);
    if normalized.is_empty() {
        return None;
    }

    try_parse_deployment_waiting(&normalized)
}

fn normalize_inline_note(raw: &str) -> String {
    raw.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn try_parse_deployment_waiting(text: &str) -> Option<PrettyPrintContext> {
    let lower = text.to_ascii_lowercase();

    let deploy_ready = lower.contains("deploy előtt")
        || lower.contains("deploy elott")
        || lower.contains("ready for deploy")
        || lower.contains("ready for deployment")
        || lower.contains("before deploy")
        || (lower.contains("deploy") && (lower.contains("előtt") || lower.contains("elott")));

    let waiting = lower.contains("vár")
        || lower.contains("var ")
        || lower.contains("waiting on")
        || lower.contains("waiting for");

    if !deploy_ready || !waiting {
        return None;
    }

    let topic = extract_topic_before_deploy(text)?;
    let dependency = extract_waiting_dependency(text)?;
    let timeline = extract_timeline_label(text);
    let topic_label = normalize_topic_for_client(&topic);
    let dependency_key = format!("{dependency} production key");
    let timeline_phrase = timeline.unwrap_or_else(|| "a short period".to_string());

    Some(PrettyPrintContext {
        project_focus: None,
        current_status: Some(format!(
            "{topic_label} is ready for deployment, but the final release is currently waiting on the {dependency_key}."
        )),
        completed_work: Vec::new(),
        blockers: vec![format!("{dependency} production approval / key access.")],
        next_steps: vec![format!(
            "Complete remaining deployment work once the {dependency} key is available."
        )],
        timeline: Some(format!(
            "Approximately {timeline_phrase} after credentials are available."
        )),
        notes: None,
        files: Vec::new(),
        raw_fallback: None,
    })
}

fn extract_topic_before_deploy(text: &str) -> Option<String> {
    let lower = text.to_ascii_lowercase();
    let deploy_idx = lower.find("deploy")?;
    let topic = text[..deploy_idx].trim();
    if topic.is_empty() {
        None
    } else {
        Some(topic.to_string())
    }
}

fn extract_waiting_dependency(text: &str) -> Option<String> {
    let lower = text.to_ascii_lowercase();

    if let Some(idx) = lower.find("barion") {
        return Some(capitalize_word(text[idx..].split_whitespace().next()?));
    }

    for token in text.split_whitespace() {
        let token_lower = token.to_ascii_lowercase();
        if token_lower.ends_with("kulcsra") || token_lower.ends_with("kulcs") {
            continue;
        }
        if token_lower == "vár" || token_lower == "var" {
            break;
        }
        if token.chars().any(|ch| ch.is_alphabetic()) && !is_status_word(&token_lower) {
            return Some(capitalize_word(token.trim_matches(|c: char| !c.is_alphanumeric())));
        }
    }

    None
}

fn is_status_word(word: &str) -> bool {
    matches!(
        word,
        "deploy"
            | "előtt"
            | "elott"
            | "áll"
            | "all"
            | "hátralévő"
            | "hatralevo"
            | "idő"
            | "ido"
            | "vár"
            | "var"
            | "fullstack"
            | "webshop"
            | "production"
            | "approval"
            | "key"
            | "access"
    )
}

fn extract_timeline_label(text: &str) -> Option<String> {
    let lower = text.to_ascii_lowercase();

    if lower.contains("~1nap") || lower.contains("1 nap") || lower.contains("1nap") {
        return Some("one day".to_string());
    }

    for token in lower.split_whitespace() {
        let cleaned = token.trim_start_matches('~');
        if let Some(num_part) = cleaned.strip_suffix("nap") {
            if let Ok(value) = num_part.parse::<u32>() {
                return Some(humanize_day_count(value));
            }
        }
        if let Some(num_part) = cleaned.strip_suffix("day") {
            if let Ok(value) = num_part.parse::<u32>() {
                return Some(humanize_day_count(value));
            }
        }
    }

    None
}

fn humanize_day_count(value: u32) -> String {
    match value {
        1 => "one day".to_string(),
        n => format!("{n} days"),
    }
}

fn normalize_topic_for_client(topic: &str) -> String {
    let lower = topic.to_ascii_lowercase();
    if lower.contains("webshop") {
        return "The webshop".to_string();
    }

    if lower.starts_with("the ") {
        return capitalize_first_letter(topic);
    }

    format!("The {}", topic.to_ascii_lowercase())
}

fn capitalize_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let rest: String = chars.collect();
            format!("{}{}", first.to_uppercase(), rest.to_ascii_lowercase())
        }
    }
}

fn capitalize_first_letter(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let rest: String = chars.collect();
            format!("{}{}", first.to_uppercase(), rest)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::fixtures::COMPACT_DEPLOYMENT_HU;

    #[test]
    fn parses_mesencsi_deployment_note_into_structured_fields() {
        let context = try_parse_compact_note(COMPACT_DEPLOYMENT_HU).expect("parse");

        assert!(context
            .current_status
            .as_deref()
            .unwrap()
            .contains("The webshop is ready for deployment"));
        assert!(context
            .current_status
            .as_deref()
            .unwrap()
            .contains("Barion production key"));
        assert_eq!(context.blockers.len(), 1);
        assert!(context.blockers[0].contains("Barion production approval"));
        assert!(context.timeline.is_some());
        assert!(!context.next_steps.is_empty());
    }

    #[test]
    fn does_not_parse_unrelated_note() {
        assert!(try_parse_compact_note("README aligned with deployment steps.").is_none());
    }
}
