use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateKind {
    StatusUpdate,
    ClientUpdate,
    DebugReport,
    Handover,
    WeeklySummary,
}

impl TemplateKind {
    pub const DEFAULT: Self = Self::StatusUpdate;

    pub fn parse(value: &str) -> Result<Self, String> {
        match value.trim().to_ascii_lowercase().as_str() {
            "status_update" => Ok(Self::StatusUpdate),
            "client_update" => Ok(Self::ClientUpdate),
            "debug_report" => Ok(Self::DebugReport),
            "handover" => Ok(Self::Handover),
            "weekly_summary" => Ok(Self::WeeklySummary),
            other => Err(format!("Unknown template kind: {other}")),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::StatusUpdate => "status_update",
            Self::ClientUpdate => "client_update",
            Self::DebugReport => "debug_report",
            Self::Handover => "handover",
            Self::WeeklySummary => "weekly_summary",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::StatusUpdate => "Status update",
            Self::ClientUpdate => "Client update",
            Self::DebugReport => "Debug report",
            Self::Handover => "Handover",
            Self::WeeklySummary => "Weekly summary",
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::StatusUpdate,
            Self::ClientUpdate,
            Self::DebugReport,
            Self::Handover,
            Self::WeeklySummary,
        ]
    }
}

pub mod client_update;
pub mod common;
pub mod debug_report;
pub mod handover;
pub mod status_update;
pub mod weekly_summary;

use crate::model::Project;

use super::pretty_print::PrettyPrintContext;

pub fn render_template(
    kind: TemplateKind,
    project: &Project,
    context: &PrettyPrintContext,
) -> String {
    match kind {
        TemplateKind::StatusUpdate => status_update::render(project, context),
        TemplateKind::ClientUpdate => client_update::render(project, context),
        TemplateKind::DebugReport => debug_report::render(project, context),
        TemplateKind::Handover => handover::render(project, context),
        TemplateKind::WeeklySummary => weekly_summary::render(project, context),
    }
}

pub fn render_empty(kind: TemplateKind, project: &Project) -> String {
    common::render_empty_shell(kind.title(), project)
}
