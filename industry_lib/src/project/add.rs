use serde::{Deserialize, Serialize};
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

use crate::StructureUuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AddExcessEntryRequest {
    pub type_id:    TypeId,
    pub quantity:   i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AddMarketEntryRequest {
    pub type_id:    TypeId,
    pub quantity:   i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AddJobEntryRequest {
    pub type_id:        TypeId,
    pub runs:           i32,
    pub structure_id:   StructureUuid,
}
