use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{CharacterId, ContractId, CorporationId, LocationId};
use utoipa::ToSchema;

use crate::utils::from_datetime;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
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

    /// Buyout price (for Auctions only)
    pub buyout:                 Option<f64>,
    /// Collateral price (for Couriers only)
    pub collateral:             Option<f64>,
    /// Number of days to perform the contract
    pub days_to_complete:       Option<i64>,
    /// End location ID (for Couriers contract)
    pub end_location_id:        Option<LocationId>,
    /// true if the contFLOWact was issued on behalf of the issuer's corporation
    pub for_corporation:        Option<bool>,
    // Price of contract (for ItemsExchange and Auctions)
    pub price:                  Option<f64>,
    /// Remuneration for contract (for Couriers only)
    pub reward:                 Option<f64>,
    /// Start location ID (for Couriers contract)
    pub start_location_id:      Option<LocationId>,
    /// Title of the contract
    pub title:                  Option<String>,
    /// Volume of items in the contract
    pub volume:                 Option<f64>,
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

impl Into<String> for ContractType {
    fn into(self) -> String {
        match self {
            ContractType::Auction       => "auction",
            ContractType::Courier       => "courier",
            ContractType::ItemExchange  => "item_exchange",
            ContractType::Loan          => "loan",
            ContractType::Unknown       => "unknown"
        }
        .to_string()
    }
}
