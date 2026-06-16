use std::path::PathBuf;
use std::sync::Mutex;

use backend::service::ProjectService;

pub struct AppState {
    pub project_service: Mutex<ProjectService>,
    pub db_path: PathBuf,
    pub app_version: String,
}