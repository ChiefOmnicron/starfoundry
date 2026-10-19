pub mod api_docs;
pub mod config;
pub mod healthcheck;
pub mod metrics;
pub mod routes;
pub mod state;

mod client;
mod populate_cache;
mod populate_structures;

mod error;
pub use self::error::{MappingError, Result};

pub const SERVICE_NAME: &'static str = "starfoundry_mapping";
pub const ENV_MAPPING_API: &str = "STARFOUNDRY_MAPPING_API_URL";
