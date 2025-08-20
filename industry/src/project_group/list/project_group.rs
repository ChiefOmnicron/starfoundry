use serde::Serialize;
use utoipa::ToSchema;

use crate::project_group::list_members::ProjectGroupMember;
use crate::project_group::ProjectGroupUuid;
use crate::structure::Structure;
use crate::item::Item;

#[derive(Debug, Serialize, ToSchema)]
#[cfg_attr(test, derive(serde::Deserialize))]
#[schema(
    example = json!({
        "id": "b034c3a9-2f4d-487d-95bb-c66fc20148b3",
        "name": "My cool group",
        "project_count": 100,
        "is_owner": true,
        "description": "Bunch of cool projects",
        "default_blacklist": [{
            "base_price": null,
            "category_id": 6,
            "group_id": 30,
            "meta_group_id": null,
            "name": "Ragnarok",
            "repackaged": 10000000,
            "type_id": 23773,
            "volume": 100000000
        }],
        "default_market": [{
            "id": "15bd47e3-6b38-4cc1-887b-94924fff30a1",
            "name": "1DQ1-A - RIP",
            "structure_id": 1337,
            "system": {
                "constellation_id": 20000696,
                "constellation_name": "O-EIMK",
                "region_id": 10000060,
                "region_name": "Delve",
                "system_id": 30004759,
                "system_name": "1DQ1-A",
                "security": -0.38578233,
                "security_group": "NULLSEC",
            },
            "structure_type": {
                "base_price": null,
                "category_id": 65,
                "group_id": 1657,
                "meta_group_id": 1,
                "name": "Keepstar",
                "repackaged": null,
                "type_id": 35834,
                "volume": 800000
            },
            "rigs": [],
            "service": [{
                "base_price": null,
                "category_id": 66,
                "group_id": 1321,
                "meta_group_id": 54,
                "name": "Standup Market Hub I",
                "repackaged": null,
                "type_id": 35892,
                "volume": 32000
            }]
        }],
        "members": [{
            "character_name": "SomeCharacterName",
            "character_id": 1337,

            "accepted": true,
            "permission": [
                "READ",
                "WRITE_GROUP"
            ],
            "is_owner": false
        }]
    })
)]
pub struct ProjectGroup {
    pub id:                ProjectGroupUuid,
    pub name:              String,
    pub project_count:     i64,
    pub is_owner:          bool,
    pub description:       Option<String>,

    pub default_blacklist: Vec<Item>,
    pub default_market:    Vec<Structure>,
    pub members:           Vec<ProjectGroupMember>,
}
