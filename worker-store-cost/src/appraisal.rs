use serde::Deserialize;
use starfoundry_lib_items::Item;
use starfoundry_lib_types::TypeId;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Appraisal {
    #[serde(skip)]
    pub id:             Uuid,
    /// code that was used to create the appraisal, will be null if it was for compression or reprocessing
    /// will be 10 characters long, [a-zA-z0-9]
    pub code:           Option<String>,
    /// timestamp when it was created in milliseconds
    pub created_at:     i64,
    /// all parsed items with their pricing information
    pub items:          Vec<AppraisalItem>,
    /// includes all fields that could not be parsed
    pub invalid:        Vec<String>,
    /// market that was used to create the appraisal
    pub market_id:      i64,

    /// initial appraisal input
    pub raw:            Option<String>,

    /// comment for the appraisal
    pub comment:        Option<String>,
    /// the price modifier is applied on the return data
    pub price_modifier: i16,

    pub total:          AppraisalTotal,
}

#[derive(Debug, Deserialize)]
pub struct AppraisalItem {
    pub quantity: i64,
    pub type_id:  TypeId,
    /// if there are not enough items on the market to cover the requested amount
    pub low_data: bool,

    pub volume: f32,

    /// general item information
    pub meta: Item,

    /// buy information
    pub buy:  MarketEntry,
    /// sell information
    pub sell: MarketEntry,
}

#[derive(Debug, Deserialize)]
pub struct AppraisalTotal {
    pub buy:    f64,
    pub sell:   f64,
    pub volume: f32
}

#[derive(Debug, Deserialize)]
pub struct MarketEntry {
    pub max:          f64,
    pub min:          f64,

    pub total_orders: i64,

    pub per_item:     MarketEntyPerItem,
}

#[derive(Debug, Deserialize)]
pub struct MarketEntyPerItem {
    pub avg:    f64,
    pub max:    f64,
    pub min:    f64,
}
