use serde::{Deserialize, Serialize};
use starfoundry_lib_market::{MarketBulkResponse, MarketItem};
use starfoundry_lib_types::StructureId;
use utoipa::ToSchema;

use crate::appraisal::{AppraisalCode, AppraisalError, Result};

#[derive(Debug, Deserialize, ToSchema)]
#[schema(
    examples(
        json!({
            "item_str": "Tritanium\t100\nPyerite\t100",
            "market": 60003760,
            "modifier": 100
        }),
        json!({
            "item_str": "Tritanium\t100\nPyerite\t100",
            "market": 60003760,
            "modifier": 110,
            "mode": "MULTIBUY",
            "persist": "NO_PERSIST"
        })
    )
)]
pub struct CreateAppraisalRequest {
    /// Market that should be used to create the appraisal
    /// Uses Jita per default
    #[serde(default = "default_market")]
    pub market_id:  StructureId,

    /// List of items. TypeId and Quantity
    /// Either `item_list` or `item_str` must be set
    #[serde(default)]
    pub item_list:  Option<Vec<MarketItem>>,
    /// List of items in a string. One item per line
    /// Either `item_list` or `item_str` must be set
    #[serde(default)]
    pub item_str:   Option<String>,

    /// Appraisal mode
    /// 
    /// `APPRAISAL` -> Creates an appraisal based on the most expensive buy, or the cheapest sell
    /// `MULTIBUY` -> Behaves like the in-game Multibuy window
    /// 
    /// Default: `APPRAISAL`
    /// 
    #[serde(default)]
    pub mode:       AppraisalMode,

    /// Persist
    /// 
    /// `PERSIST`       -> Persist the appraisal for future calls
    /// `NO_PERSIST`    -> Does not persist the appraisal, use this if you create a lot
    /// 
    /// Default: `PERSIST`
    /// 
    #[serde(default)]
    pub persist:    AppraisalPersistence,

    /// Optional comment
    #[serde(default)]
    pub comment:    Option<String>,

    /// Optional price modifier
    #[serde(default = "default_price_modifier")]
    pub modifier:   u32,
}

impl CreateAppraisalRequest {
    pub fn validate(&self) -> Result<bool> {
        if self.item_list.is_some() || self.item_str.is_some() {
            Ok(true)
        } else {
            Err(AppraisalError::InvalidAppraisal("either `item_list` or `item_str` must be set".into()))
        }
    }
}

fn default_market() -> StructureId {
    StructureId(60003760)
}

fn default_price_modifier() -> u32 {
    100u32
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
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

#[derive(Debug, Serialize, ToSchema)]
pub struct Appraisal {
    pub code:           AppraisalCode,
    pub created_at_ts:  i64,

    pub invalid:        Vec<String>,
    pub items:          Vec<MarketBulkResponse>,
    
    pub market_id:      StructureId,
    pub total:          AppraisalTotal,

    pub modifier:       u32,
    pub comment:        Option<String>,
    pub raw:            Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AppraisalTotal {
    pub buy:        f64,
    pub sell:       f64,
}
