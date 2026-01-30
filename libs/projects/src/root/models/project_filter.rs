use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::ProjectGroupUuid;

#[derive(Debug, Default, Deserialize, ToSchema, IntoParams)]
pub struct ProjectFilter {
    #[serde(default)]
    #[param(
        default = json!("<empty string>")
    )]
    pub name:   Option<String>,

    // workourd as arrays aren´t supported
    #[param(
        default = json!("CREATED,INITIALIZED,IN_PROGRESS,PAUSED,DONE")
    )]
    #[serde(default = "default_status")]
    pub status: Option<String>,

    #[serde(default)]
    pub project_group: Option<ProjectGroupUuid>,
}

fn default_status() -> Option<String> {
    Some("CREATED,INITIALIZED,IN_PROGRESS,PAUSED,DONE".into())
}
