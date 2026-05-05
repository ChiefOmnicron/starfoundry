use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::ProjectGroupUuid;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "b034c3a9-2f4d-487d-95bb-c66fc20148b3",
        "name": "My cool group",
        "project_count": 100,
        "is_owner": true,
        "description": "Bunch of cool projects",
        "members": [{
            "character_name": "SomeCharacterName",
            "character_id": 1337,

            "permission": [
                "READ",
                "WRITE_GROUP"
            ],
            "is_owner": false
        }]
    })
)]
pub struct ProjectGroupMinimal {
    pub id:            ProjectGroupUuid,
    pub name:          String,
    pub project_count: i64,
    pub is_owner:      bool,
    pub description:   Option<String>,
    pub archived:      bool,
}
