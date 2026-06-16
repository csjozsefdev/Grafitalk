use serde::{Deserialize, Serialize};

pub const SOURCE_GRAFITALK: &str = "grafitalk";
pub const SCHEMA_VERSION: &str = "0.1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DraftExportDocument {
    pub source: String,
    pub schema_version: String,
    pub project_name: String,
    pub template_kind: String,
    pub subject: String,
    pub body: String,
    pub exported_at: String,
}
