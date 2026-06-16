#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportFormat {
    Json,
    PlainText,
    Markdown,
}

impl ImportFormat {
    pub fn from_hint(hint: &str) -> Option<Self> {
        match hint.trim().to_ascii_lowercase().as_str() {
            "json" | "application/json" => Some(Self::Json),
            "txt" | "text" | "plain" | "text/plain" => Some(Self::PlainText),
            "md" | "markdown" | "text/markdown" => Some(Self::Markdown),
            _ => None,
        }
    }
}

pub fn detect_format(content: &str, path_hint: Option<&str>) -> ImportFormat {
    if let Some(path) = path_hint {
        let lower = path.to_ascii_lowercase();
        if lower.ends_with(".json") {
            return ImportFormat::Json;
        }
        if lower.ends_with(".md") || lower.ends_with(".markdown") {
            return ImportFormat::Markdown;
        }
        if lower.ends_with(".txt") {
            return ImportFormat::PlainText;
        }
    }

    let trimmed = content.trim();
    if trimmed.starts_with('{')
        && trimmed.ends_with('}')
        && serde_json::from_str::<serde_json::Value>(trimmed).is_ok()
    {
        return ImportFormat::Json;
    }

    if looks_like_markdown(trimmed) {
        return ImportFormat::Markdown;
    }

    ImportFormat::PlainText
}

fn looks_like_markdown(content: &str) -> bool {
    content.lines().any(|line| {
        let trimmed = line.trim();
        trimmed.starts_with("# ")
            || trimmed.starts_with("## ")
            || trimmed.starts_with("### ")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_json_from_extension() {
        assert_eq!(
            detect_format("{}", Some("handoff.json")),
            ImportFormat::Json
        );
    }

    #[test]
    fn detects_markdown_from_headers() {
        let content = "## Current status\nReady.";
        assert_eq!(detect_format(content, None), ImportFormat::Markdown);
    }

    #[test]
    fn detects_plain_text_by_default() {
        assert_eq!(
            detect_format("Current status:\nReady.", None),
            ImportFormat::PlainText
        );
    }
}
