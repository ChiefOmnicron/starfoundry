use serde::{Deserialize, Serialize};
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct BlueprintJson {
    pub blueprint_type_id: TypeId,
    pub product_type_id:   TypeId,
    pub data:              serde_json::Value,
}
