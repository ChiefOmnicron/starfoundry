use starfoundry_lib_types::{StructureId, TypeId};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams)]
pub struct MarketVirtualRequest {
    pub type_id:    TypeId,
    pub market:     StructureId,
    pub quantity:   i32,
}
