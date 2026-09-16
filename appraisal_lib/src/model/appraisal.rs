use serde::{Deserialize, Serialize};
use starfoundry_lib_market::MarketBulkResponse;
use starfoundry_lib_types::StructureId;
use utoipa::ToSchema;

use crate::AppraisalCode;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum AppraisalMode {
    Appraisal,
    Multibuy,
}

impl Default for AppraisalMode {
    fn default() -> Self {
        Self::Appraisal
    }
}

impl From<AppraisalMode> for String {
    fn from(value: AppraisalMode) -> Self {
        match value {
            AppraisalMode::Appraisal    => "APPRAISAL",
            AppraisalMode::Multibuy     => "MULTIBUY"
        }.into()
    }
}

impl TryFrom<String> for AppraisalMode {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let converted = match value.as_ref() {
            "APPRAISAL" => AppraisalMode::Appraisal,
            "MULTIBUY"  => AppraisalMode::Multibuy,
            _           => AppraisalMode::Appraisal
        };
        Ok(converted)
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum AppraisalPersistence {
    Persist,
    NoPersist,
}

impl Default for AppraisalPersistence {
    fn default() -> Self {
        Self::Persist
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Appraisal {
    pub code:           AppraisalCode,
    pub created_at_ts:  i64,

    pub invalid:        Vec<String>,
    pub items:          Vec<MarketBulkResponse>,

    pub market_id:      StructureId,
    pub total:          AppraisalTotal,

    pub mode:           AppraisalMode,

    pub modifier:       u32,
    pub comment:        Option<String>,
    pub raw:            Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AppraisalTotal {
    pub buy:        f64,
    pub sell:       f64,
}
