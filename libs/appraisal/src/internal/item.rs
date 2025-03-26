use serde::{Deserialize, Serialize};
use starfoundry_libs_types::TypeId;

#[derive(Debug, Deserialize, Serialize)]
#[deprecated(note = "replace with external implementation")]
pub struct InternalItem {
    pub quantity: i32,
    pub type_id: TypeId,
    pub sell: f64,
    pub total_sell: f64,
}
