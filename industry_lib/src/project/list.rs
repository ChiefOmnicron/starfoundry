use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use starfoundry_lib_eve_gateway::Item;
use starfoundry_lib_types::CharacterId;
use utoipa::{IntoParams, ToSchema};

use crate::{ProjectGroupUuid, ProjectJobUuid, ProjectUuid};
use crate::project::{ProjectJobStatus, ProjectStatus};
use crate::structure::Structure;
use crate::project_group::ProjectGroupMinimal;

#[derive(Debug, Default, Deserialize, Serialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ProjectFilter {
    #[serde(default)]
    #[param(
        example = json!("Project 1337"),
        required = false,
    )]
    pub name: Option<String>,

    #[param(
        default = json!("DRAFT,READY_TO_START,IN_PROGRESS,PAUSED,DONE"),
        required = false,
    )]
    #[serde(default = "default_status")]
    pub status: Option<String>,

    #[serde(default)]
    #[param(
        example = json!("019b5d76-0ebd-77f4-80b0-12daf86501b6"),
        required = false,
    )]
    pub project_group_id: Option<ProjectGroupUuid>,

    #[serde(default)]
    #[param(
        example = json!("Eistonen Kodan Sasen"),
        required = false,
    )]
    pub orderer: Option<String>,
}

fn default_status() -> Option<String> {
    Some("DRAFT,READY_TO_START,IN_PROGRESS,PAUSED,DONE".into())
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "b034c3a9-2f4d-487d-95bb-c66fc20148b3",
        "name": "My cool project",
        "status": "IN_PROGRESS",
        "orderer": "Me Myself and I",
        "sell_price": 1337
    })
)]
pub struct ProjectMinimal {
    pub id:            ProjectUuid,
    pub name:          String,
    pub status:        ProjectStatus,
    pub orderer:       String,
    pub project_group: ProjectGroupMinimal,

    pub sell_price:    Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectJob {
    pub id:         ProjectJobUuid,
    pub project_id: ProjectUuid,
    pub job_id:     Option<i32>,
    pub status:     ProjectJobStatus,

    pub runs:       i32,
    pub cost:       Option<f64>,

    pub item:       Item,
    pub structure:  Structure,
    pub started_by: Option<CharacterId>,

    pub end_date:   Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectJobGroup {
    pub header:  String,
    pub entries: Vec<ProjectJob>,
}

#[derive(Debug, Default, Deserialize, Serialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ProjectJobFilter {
    #[serde(default)]
    pub startable: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectJobAllGroup {
    pub header:     String,
    pub project_id: ProjectUuid,
    pub entries:    Vec<ProjectJob>,
}


#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectExcess {
    pub item:       Item,
    pub quantity:   i32,
    pub cost:       Option<f64>,
}
