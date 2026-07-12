use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{Error, TagUuid, Result};
use crate::tag::{TagAuto, TagType};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateTag {
    pub color:      String,
    pub content:    String,
    pub typ:        TagType,

    #[serde(default)]
    pub auto:       Vec<TagAuto>
}

impl CreateTag {
    pub fn validate(&self) -> Result<bool> {
        if self.typ == TagType::Auto && self.auto.is_empty() {
            return Err(Error::ValidationError("Field 'auto' must contain at least one value, when 'typ' is 'AUTO'".into()));
        }

        Ok(true)
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "fd324c9f-ecda-49c8-948e-18f4b4b23bff"
    })
)]
pub struct CreateTagResponse {
    pub id: TagUuid,
}

impl Default for CreateTagResponse {
    fn default() -> Self {
        Self {
            id: Uuid::default().into(),
        }
    }
}
