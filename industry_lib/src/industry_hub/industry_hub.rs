use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{IndustryHubUuid, Structure};

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct IndustryHub {
    pub id:          IndustryHubUuid,
    pub name:        String,
    pub structures:  Vec<Structure>,
    pub shares:      Vec<IndustryHubShare>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    examples(
        json!({
            "name": "Rational Chaos Inc.",
            "share_id": 98024275,
            "share_type": "CORPORATION"
        })
    )
)]
pub struct IndustryHubShare {
    pub name:       String,
    /// either a character id, corporation id or alliance id
    pub share_id:   i32,
    pub share_type: IndustryHubShareType,
}

#[derive(
    Clone, Debug, Copy, Hash,
    PartialEq, Eq, PartialOrd, Ord,
    sqlx::Type, Deserialize, Serialize, ToSchema,
)]
#[sqlx(type_name = "SHARE_TYPE")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IndustryHubShareType {
    Character,
    Corporation,
    Alliance,
}
