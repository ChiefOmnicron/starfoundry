use serde::{Deserialize, Serialize};
use starfoundry_lib_types::ItemId;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ResolvedItem {
    pub item_id:    ItemId,
    pub name:       String,
}
