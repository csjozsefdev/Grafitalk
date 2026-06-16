use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportError {
    EmptyFile,
    EmptyContent,
    FileTooLarge { max_bytes: usize },
    InvalidHandoff(String),
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyFile => write!(f, "Import file is empty."),
            Self::EmptyContent => write!(f, "Import file contains no usable context."),
            Self::FileTooLarge { max_bytes } => write!(
                f,
                "Import file is too large (maximum {max_bytes} bytes)."
            ),
            Self::InvalidHandoff(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for ImportError {}
