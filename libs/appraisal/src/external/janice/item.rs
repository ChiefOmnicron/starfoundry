use serde::Deserialize;

use crate::external::janice::value::JaniceValueItem;

#[derive(Debug, Deserialize)]
pub struct JaniceItem {
    /// Effective price
    #[serde(rename = "effectivePrices")]
    pub effective_price: JaniceValueItem,
    /// Price if sold immediate
    #[serde(rename = "immediatePrices")]
    pub immediate_price: JaniceValueItem,
    /// Average price of the top 5
    #[serde(rename = "top5AveragePrices")]
    pub average_price:   JaniceValueItem,

    /// Given amount of the item
    pub amount:          u64,

    /// Information about the item
    #[serde(rename = "itemType")]
    pub item_type:       JaniceItemType,
}

/// Represents an janice Appraisal Value
/// 
#[derive(Debug, Deserialize)]
pub struct JaniceItemType {
    /// TypeId of the item
    pub eid:  u32,
    /// Name of the item
    pub name: String,
}
