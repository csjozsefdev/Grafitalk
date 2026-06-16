use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportError {
    EmptyDraft,
    InvalidFormat(String),
    InvalidTemplateKind(String),
    PdfGeneration(String),
    Serialization(String),
}

impl fmt::Display for ExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDraft => write!(f, "Nothing to export yet."),
            Self::InvalidFormat(value) => write!(f, "Unsupported export format: {value}"),
            Self::InvalidTemplateKind(value) => write!(f, "{value}"),
            Self::PdfGeneration(message) => write!(f, "Failed to generate PDF: {message}"),
            Self::Serialization(message) => write!(f, "Failed to serialize export: {message}"),
        }
    }
}

impl std::error::Error for ExportError {}
