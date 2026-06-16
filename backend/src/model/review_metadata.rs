use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectReviewMetadata {
    pub last_generated_at: Option<String>,
    pub last_template_kind: String,
    pub last_exported_at: Option<String>,
    pub last_export_format: Option<String>,
}
