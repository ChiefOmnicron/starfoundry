use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::FilterUuid;
use crate::filter::FilterType;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateFilter {
    pub name:       String,
    pub filters:    serde_json::Value,
    pub typ:        FilterType,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateFilterResponse {
    pub id: FilterUuid,
}
