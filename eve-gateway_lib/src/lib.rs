#![allow(clippy::redundant_field_names)]

mod asset;
mod character;
mod client_trait;
mod client;
mod error;
mod eve_asset;
mod fitting;
mod industry;
mod item;
mod market;
mod search;
mod standing;
mod structure_type;
mod structure;
mod system;
mod utils;

pub mod contract;

pub use self::asset::*;
pub use self::character::*;
pub use self::client::*;
pub use self::client_trait::*;
pub use self::error::*;
pub use self::eve_asset::*;
pub use self::fitting::*;
pub use self::industry::*;
pub use self::item::*;
pub use self::market::*;
pub use self::search::*;
pub use self::standing::*;
pub use self::structure::*;
pub use self::structure_type::*;
pub use self::system::*;

use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use chrono::{NaiveDate, Utc};

pub const ENV_EVE_GATEWAY_API: &str      = "STARFOUNDRY_EVE_GATEWAY_API_URL";
pub const ENV_EVE_GATEWAY_JWT_SIGN: &str = "STARFOUNDRY_EVE_GATEWAY_JWT_SIGN";

#[derive(Debug, Default, Deserialize, Serialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub struct TimeSpanQuery {
    pub start:      NaiveDate,
    #[serde(default = "default_end")]
    pub end:        NaiveDate,
}

fn default_end() -> NaiveDate {
    Utc::now().date_naive()
}
