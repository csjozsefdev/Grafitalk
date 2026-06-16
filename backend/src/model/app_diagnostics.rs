use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppDiagnostics {
    pub app_version: String,
    pub db_path: String,
    pub project_count: i64,
    pub latest_migration: Option<i32>,
    pub platform: String,
}
