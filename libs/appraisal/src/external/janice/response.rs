use serde::Deserialize;

use crate::external::janice::item::JaniceItem;
use crate::external::janice::value::JaniceValue;

/// Represents the response from janice
///
/// Not all fields are represented
/// 
/// # Example
/// 
/// ``` json
/// {
///  "id": 0,
///  "created": "2023-05-03T06:37:09.9909689Z",
///  "expires": "2023-06-02T06:37:09.9909689Z",
///  "datasetTime": "2023-05-03T06:26:53.7654668Z",
///  "code": null,
///  "designation": "appraisal",
///  "pricing": "split",
///  "pricingVariant": "immediate",
///  "pricePercentage": 1,
///  "comment": null,
///  "isCompactized": true,
///  "input": "Neurolink Protection Cell 10",
///  "failures": "",
///  "market": {
///    "id": 2,
///    "name": "Jita 4-4"
///  },
///  "totalVolume": 55000,
///  "totalPackagedVolume": 55000,
///  "effectivePrices": {
///    "totalBuyPrice": 8703000000,
///    "totalSplitPrice": 9186000000,
///    "totalSellPrice": 9669000000
///  },
///  "immediatePrices": {
///    "totalBuyPrice": 8703000000,
///    "totalSplitPrice": 9186000000,
///    "totalSellPrice": 9669000000
///  },
///  "top5AveragePrices": {
///    "totalBuyPrice": 8703000000,
///    "totalSplitPrice": 9187500000,
///    "totalSellPrice": 9672000000
///  },
///  "items": [
///    {
///      "id": 0,
///      "amount": 10,
///      "buyOrderCount": 15,
///      "buyVolume": 17,
///      "sellOrderCount": 49,
///      "sellVolume": 101,
///      "effectivePrices": {
///        "buyPrice": 870300000,
///        "splitPrice": 918600000,
///        "sellPrice": 966900000,
///        "buyPriceTotal": 8703000000,
///        "splitPriceTotal": 9186000000,
///        "sellPriceTotal": 9669000000,
///        "buyPrice5DayMedian": 869760000,
///        "splitPrice5DayMedian": 918200000,
///        "sellPrice5DayMedian": 869760000,
///        "buyPrice30DayMedian": 902663333.3333334,
///        "splitPrice30DayMedian": 943500000,
///        "sellPrice30DayMedian": 999700000
///      },
///      "immediatePrices": {
///        "buyPrice": 870300000,
///        "splitPrice": 918600000,
///        "sellPrice": 966900000,
///        "buyPriceTotal": 8703000000,
///        "splitPriceTotal": 9186000000,
///        "sellPriceTotal": 9669000000,
///        "buyPrice5DayMedian": 869760000,
///        "splitPrice5DayMedian": 918200000,
///        "sellPrice5DayMedian": 869760000,
///        "buyPrice30DayMedian": 902663333.3333334,
///        "splitPrice30DayMedian": 943500000,
///        "sellPrice30DayMedian": 999700000
///      },
///      "top5AveragePrices": {
///        "buyPrice": 870300000,
///        "splitPrice": 918750000,
///        "sellPrice": 967200000,
///        "buyPriceTotal": 8703000000,
///        "splitPriceTotal": 9187500000,
///        "sellPriceTotal": 9672000000,
///        "buyPrice5DayMedian": 869900000,
///        "splitPrice5DayMedian": 918512500,
///        "sellPrice5DayMedian": 869900000,
///        "buyPrice30DayMedian": 900000000,
///        "splitPrice30DayMedian": 943780000,
///        "sellPrice30DayMedian": 999750000
///      },
///      "totalVolume": 55000,
///      "totalPackagedVolume": 55000,
///      "itemType": {
///        "eid": 57488,
///        "name": "Neurolink Protection Cell",
///        "volume": 5500,
///        "packagedVolume": 5500
///      }
///    }
///  ]
///}
/// ```
/// 
#[derive(Debug, Deserialize)]
pub struct JaniceResponse {
    /// Effective price
    #[serde(rename = "effectivePrices")]
    pub effective_price: JaniceValue,
    /// Price if sold immediate
    #[serde(rename = "immediatePrices")]
    pub immediate_price: JaniceValue,
    /// Average price of the top 5
    #[serde(rename = "top5AveragePrices")]
    pub average_price:   JaniceValue,

    /// Breakdown of all items
    pub items:           Vec<JaniceItem>,

    /// Optional code to share the appraisal, will be set if `persist` is true
    pub code:            Option<String>,
}
