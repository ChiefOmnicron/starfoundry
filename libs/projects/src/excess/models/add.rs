use serde::Deserialize;
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct AddExcess {
    pub type_id:  TypeId,
    pub quantity: i32,
}
