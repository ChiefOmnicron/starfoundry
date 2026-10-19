mod code;
mod compression;
mod create;
mod error;
mod fetch;
mod reprocessing;
mod settings;

use serde::{Deserialize, Serialize};
use starfoundry_lib_types::StructureId;
use starfoundry_market::market::MarketBulkResponse;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub use self::code::AppraisalCode;
pub use self::create::CreateAppraisalRequest;
pub use self::error::*;
use crate::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    let create = OpenApiRouter::new().routes(routes!(create::api));
    let fetch = OpenApiRouter::new().routes(routes!(fetch::api));

    // let compression = OpenApiRouter::new()
    //    .routes(routes!(compression::api));
    let reprocessing = OpenApiRouter::new().routes(routes!(reprocessing::api));

    // let settings = OpenApiRouter::new()
    //    .routes(routes!(settings::api));

    OpenApiRouter::new()
        .merge(create)
        .merge(fetch)
        //.merge(compression)
        .merge(reprocessing)

    //.merge(settings)
}

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
            AppraisalMode::Appraisal => "APPRAISAL",
            AppraisalMode::Multibuy => "MULTIBUY",
        }
        .into()
    }
}

impl TryFrom<String> for AppraisalMode {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let converted = match value.as_ref() {
            "APPRAISAL" => AppraisalMode::Appraisal,
            "MULTIBUY" => AppraisalMode::Multibuy,
            _ => AppraisalMode::Appraisal,
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
    pub code:          AppraisalCode,
    pub created_at_ts: i64,

    pub invalid: Vec<String>,
    pub items:   Vec<MarketBulkResponse>,

    pub market_id: StructureId,
    pub total:     AppraisalTotal,

    pub mode: AppraisalMode,

    pub modifier: u32,
    pub comment:  Option<String>,
    pub raw:      Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AppraisalTotal {
    pub buy:  f64,
    pub sell: f64,
}
