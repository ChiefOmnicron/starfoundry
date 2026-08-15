use serde::{Deserialize, Serialize};
use starfoundry_lib_types::ContactId;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Standing {
    pub contact_id:     ContactId,
    pub contact_type:   String,
    pub standing:       f32,
}
