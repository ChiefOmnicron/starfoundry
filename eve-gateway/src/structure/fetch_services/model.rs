use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::item::Item;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct StructureServiceResponse {
    pub services: Vec<Item>,
    pub slots:    i32,
}
