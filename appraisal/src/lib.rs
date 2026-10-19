mod api_docs;
mod client;
mod client_trait;
mod config;
mod error;
mod state;

pub const SERVICE_NAME: &str = "SF_APPRAISAL";

pub mod appraisal;
pub mod healthcheck;
pub mod metrics;

pub use crate::api_docs::ApiDoc;
pub use crate::client::*;
pub use crate::client_trait::*;
pub use crate::config::Config;
pub use crate::error::*;
pub use crate::metrics::{Metric, path_metrics};
pub use crate::state::AppState;
