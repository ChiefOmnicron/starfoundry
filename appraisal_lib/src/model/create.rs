use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use starfoundry_lib_types::StructureId;
use starfoundry_lib_market::MarketItem;
use crate::{AppraisalMode, AppraisalPersistence};
use crate::error::{Error, Result};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
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
            Err(Error::InvalidAppraisal("either `item_list` or `item_str` must be set".into()))
        }
    }
}

fn default_market() -> StructureId {
    StructureId(60003760)
}

fn default_price_modifier() -> u32 {
    100u32
}
