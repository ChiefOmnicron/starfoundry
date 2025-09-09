use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::item::Item;
use starfoundry_lib_structures::StructureDatabase;
use crate::project_group::service::ProjectGroupMember;

#[derive(Debug, Serialize, ToSchema)]
#[cfg_attr(test, derive(serde::Deserialize))]
#[schema(
    example = json!({
        "id": "b034c3a9-2f4d-487d-95bb-c66fc20148b3",
        "name": "My cool group",
        "member_count": 5,
        "project_count": 100,
        "is_owner": true,
        "description": "Bunch of cool projects"
    })
)]
pub struct ProjectGroupFetch {
    pub id:                Uuid,
    pub name:              String,
    pub member_count:      i64,
    pub project_count:     i64,
    pub is_owner:          bool,

    pub description:       Option<String>,

    pub default_blacklist: Vec<Item>,
    pub default_market:    Vec<StructureDatabase>,
    pub members:           Vec<ProjectGroupMember>,
}
