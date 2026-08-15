#![allow(clippy::redundant_field_names)]

mod client_trait;
mod client;
mod error;
mod routes;

pub use self::client::*;
pub use self::client_trait::*;
pub use self::error::*;
pub use self::routes::*;

pub const ENV_MAPPING_API: &str = "STARFOUNDRY_MAPPING_API_URL";
