mod api_client;
mod auth;
mod config;
mod error;
mod identity;
mod state;

pub mod catch_all;
pub mod healthcheck;
pub mod metrics;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub use crate::api_client::*;
pub use crate::auth::*;
pub use crate::config::Config;
pub use crate::error::*;
pub use crate::identity::*;
pub use crate::state::AppState;

pub(crate) const SERVICE_NAME: &str = "SF_GATEWAY";

pub const HEADER_CHARACTER_ID: &str = "X-SF-CharacterId";
pub const HEADER_CORPORATION_ID: &str = "X-SF-CorporationId";
pub const HEADER_ALLIANCE_ID: &str = "X-SF-AllianceId";
pub const HEADER_IS_ADMIN: &str = "X-SF-IsAdmin";
pub const HEADER_SOURCE: &str = "X-SF-Source";
pub const HEADER_SERVICE: &str = "X-SF-Service";

/// Contains the redirection URL including scopes
/// for the request to redirect to
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginRedirect {
    pub url: String,
}

// TODO: Remove once this_error implements it
// https://github.com/dtolnay/thiserror/issues/424
// https://github.com/dtolnay/thiserror/pull/431
// boxed_from!(Error::GatewayClientError, starfoundry_gateway::Error);
#[macro_export]
macro_rules! boxed_from {
    ($dst_ty:ident :: $variant:ident, $src_ty:ty) => {
        impl From<$src_ty> for $dst_ty {
            fn from(value: $src_ty) -> Self {
                Self::$variant(Box::new(value))
            }
        }
    };
}
