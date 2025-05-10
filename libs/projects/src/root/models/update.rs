use serde::Deserialize;
use utoipa::ToSchema;

use crate::{ProjectGroupUuid, ProjectStatus};

#[derive(Debug, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "name": "My cool project name",
        "status": "DONE",
        "orderer": "its for somebody I used to know",
        "project_group_id": "60f5931b-bea5-45a1-a2ea-520d2535b138",
        "note": null,
        "sell_price": 1337
    })
)]
pub struct UpdateProject {
    pub name:             String,
    pub status:           ProjectStatus,

    pub orderer:          Option<String>,
    pub sell_price:       Option<f64>,

    pub note:             Option<String>,
    pub project_group_id: Option<ProjectGroupUuid>,
}
