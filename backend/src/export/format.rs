#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Txt,
    Md,
    Json,
    Pdf,
}

impl ExportFormat {
    pub fn parse(value: &str) -> Result<Self, super::error::ExportError> {
        match value.trim().to_ascii_lowercase().as_str() {
            "txt" | "text" | "plain" => Ok(Self::Txt),
            "md" | "markdown" => Ok(Self::Md),
            "json" => Ok(Self::Json),
            "pdf" => Ok(Self::Pdf),
            other => Err(super::error::ExportError::InvalidFormat(other.to_string())),
        }
    }

    pub fn from_path_hint(path: &str) -> Result<Self, super::error::ExportError> {
        let lower = path.to_ascii_lowercase();
        if lower.ends_with(".txt") {
            return Ok(Self::Txt);
        }
        if lower.ends_with(".md") || lower.ends_with(".markdown") {
            return Ok(Self::Md);
        }
        if lower.ends_with(".json") {
            return Ok(Self::Json);
        }
        if lower.ends_with(".pdf") {
            return Ok(Self::Pdf);
        }

        Err(super::error::ExportError::InvalidFormat(
            "Choose a .txt, .md, .json, or .pdf file.".to_string(),
        ))
    }

    pub fn extension(self) -> &'static str {
        match self {
            Self::Txt => "txt",
            Self::Md => "md",
            Self::Json => "json",
            Self::Pdf => "pdf",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_format_aliases() {
        assert_eq!(ExportFormat::parse("txt").expect("txt"), ExportFormat::Txt);
        assert_eq!(ExportFormat::parse("markdown").expect("md"), ExportFormat::Md);
    }

    #[test]
    fn detects_format_from_path() {
        assert_eq!(
            ExportFormat::from_path_hint("draft.pdf").expect("pdf"),
            ExportFormat::Pdf
        );
    }
}
