//! EVE-Gateway services.
//!
//! The service sis designed to abstract everything away from the EVE-API.
//!
//! Current supported features:
//! - Login with EVE SSO
//! - Wraps a small set of EVE-API calls (see the OpenAPI Documentation)
//! - Caching of routes

#![allow(clippy::redundant_field_names)]

pub mod api_docs;
pub mod auth;
pub mod config;
pub mod healthcheck;
pub mod metrics;
pub mod state;
pub mod utils;

pub mod asset;
pub mod character;
pub mod contract;
pub mod fitting;
pub mod industry;
pub mod item;
pub mod market;
pub mod search;
pub mod standing;
pub mod structure;
pub mod system;

pub mod eve;
pub mod proxy;

mod error;
pub use self::error::{EveGatewayError, Result};

mod client;
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub use self::client::{
    ApiClientExtended, EveGatewayApiClient, EveGatewayClient
};

pub const ENV_EVE_GATEWAY_API: &str = "STARFOUNDRY_EVE_GATEWAY_API_URL";
pub const ENV_EVE_GATEWAY_JWT_SIGN: &str = "STARFOUNDRY_EVE_GATEWAY_JWT_SIGN";

#[derive(Debug, Default, Deserialize, Serialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub struct TimeSpanQuery {
    pub start: NaiveDate,
    #[serde(default = "default_end")]
    pub end:   NaiveDate,
}

fn default_end() -> NaiveDate {
    Utc::now().date_naive()
}
