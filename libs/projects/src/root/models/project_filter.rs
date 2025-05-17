use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::ProjectGroupUuid;

#[derive(Debug, Default, Deserialize, ToSchema, IntoParams)]
#[into_params(
    parameter_in = Query
)]
pub struct ProjectFilter {
    #[serde(default)]
    #[param(
        default = json!("<empty string>")
    )]
    pub name:   Option<String>,

    // workaround as arrays aren´t supported
    /// Possible values:
    /// - DONE
    /// - IN_PROGRESS
    /// - PREPARING
    /// - PAUSED
    #[param(
        default = json!("IN_PROGRESS")
    )]
    #[serde(default = "default_status")]
    pub status: Option<String>,

    #[serde(default)]
    pub project_group: Option<ProjectGroupUuid>,
}

fn default_status() -> Option<String> {
    Some("IN_PROGRESS".into())
}
