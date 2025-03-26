use serde::Deserialize;

/// Represents an janice Appraisal Value
/// 
#[derive(Debug, Deserialize)]
pub struct JaniceValue {
    /// Buy price for all items
    #[serde(rename = "totalBuyPrice")]
    pub buy_price:   f32,
    /// Split price for all items
    #[serde(rename = "totalSplitPrice")]
    pub split_price: f32,
    /// Sell price for all items
    #[serde(rename = "totalSellPrice")]
    pub sell_price:  f32,
}

/// Represents an janice Appraisal Item Value
/// 
#[derive(Debug, Deserialize)]
pub struct JaniceValueItem {
    /// Buy price for one items
    #[serde(rename = "buyPrice")]
    pub buy_price:         f32,
    /// Split price for one items
    #[serde(rename = "splitPrice")]
    pub split_price:       f32,
    /// Sell price for one items
    #[serde(rename = "sellPrice")]
    pub sell_price:        f32,

    /// Buy price for all items
    #[serde(rename = "buyPriceTotal")]
    pub buy_price_total:   f32,
    /// Split price for all items
    #[serde(rename = "splitPriceTotal")]
    pub split_price_total: f32,
    /// Sell price for all items
    #[serde(rename = "sellPriceTotal")]
    pub sell_price_total:  f32,
}
