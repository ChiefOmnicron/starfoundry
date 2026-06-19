use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{CharacterId, ContractId, CorporationId, LocationId};
use utoipa::ToSchema;

use crate::utils::from_datetime;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "collateral": 0,
        "contract_id": 231837574,
        "date_expired": "2026-06-15T22:12:21Z",
        "date_issued": "2026-05-18T22:12:21Z",
        "days_to_complete": 0,
        "end_location_id": 60003760,
        "issuer_corporation_id": 1467638572,
        "issuer_id": 240799055,
        "price": 12000000,
        "reward": 0,
        "start_location_id": 60003760,
        "title": "Some cool contract",
        "type": "item_exchange",
        "volume": 0.1
    })
)]
pub struct PublicContract {
    pub contract_id:            ContractId,
    /// Expiration date of the contract
    #[serde(deserialize_with = "from_datetime")]
    pub date_expired:           NaiveDateTime,
    /// Creation date of the contract
    #[serde(deserialize_with = "from_datetime")]
    pub date_issued:            NaiveDateTime,
    /// Character's corporation ID for the issuer
    pub issuer_corporation_id:  CorporationId,
    /// Character ID for the issuer
    pub issuer_id:              CharacterId,
    /// Type of the contract
    #[serde(rename = "type")]
    pub typ:                    ContractType,

    /// Title of the contract
    pub title:                  Option<String>,
    /// Volume of items in the contract
    pub volume:                 Option<f64>,
    /// true if the contract was issued on behalf of the issuer's corporation
    pub for_corporation:        Option<bool>,
    /// Number of days to perform the contract
    pub days_to_complete:       Option<i64>,

    /// Collateral price (for Couriers only)
    pub collateral:             Option<f64>,
    /// End location ID (for Couriers contract)
    pub end_location_id:        Option<LocationId>,
    /// Remuneration for contract (for Couriers only)
    pub reward:                 Option<f64>,
    /// Start location ID (for Couriers contract)
    pub start_location_id:      Option<LocationId>,

    /// Buyout price (for Auctions only)
    pub buyout:                 Option<f64>,
    // Price of contract (for ItemsExchange and Auctions)
    pub price:                  Option<f64>,
}

#[derive(
    Copy, Clone, Debug, Deserialize, Serialize, ToSchema,
    PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum ContractType {
    Unknown,
    ItemExchange,
    Auction,
    Courier,
    Loan,
}

impl From<ContractType> for String {
    fn from(value: ContractType) -> Self {
        match value {
            ContractType::Auction       => "auction",
            ContractType::Courier       => "courier",
            ContractType::ItemExchange  => "item_exchange",
            ContractType::Loan          => "loan",
            ContractType::Unknown       => "unknown"
        }
        .to_string()
    }
}
