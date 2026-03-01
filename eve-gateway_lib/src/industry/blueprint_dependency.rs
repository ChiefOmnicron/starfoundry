use serde::{Deserialize, Serialize};
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct BlueprintDependency {
    pub blueprint_type_id: TypeId,
    pub product_type_id:   TypeId,
    pub time:              i32,
    pub depends_on:        Vec<TypeId>,
}
