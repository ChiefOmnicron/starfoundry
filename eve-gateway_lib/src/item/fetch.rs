use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{EveGatewayClient, Result};
use starfoundry_lib_types::{CategoryId, GroupId, TypeId};

pub async fn fetch_item(
    gateway_client: &impl EveGatewayClient,
    type_id:        TypeId,
) -> Result<Item> {
    gateway_client
        .fetch(&format!("items/{}", *type_id))
        .await
        .map_err(Into::into)
}


#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "base_price": null,
        "category_id": 6,
        "group_id": 30,
        "meta_group_id": null,
        "name": "Ragnarok",
        "repackaged": 10000000,
        "type_id": 23773,
        "volume": 100000000
    })
)]
pub struct Item {
    pub type_id:        TypeId,
    pub category_id:    CategoryId,
    pub group_id:       GroupId,
    pub volume:         f32,
    pub name:           String,

    pub meta_group_id:  Option<GroupId>,
    pub repackaged:     Option<i32>,
}
