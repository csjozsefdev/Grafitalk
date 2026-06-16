use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub client_label: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub last_used_at: Option<String>,
    pub archived_at: Option<String>,
}
