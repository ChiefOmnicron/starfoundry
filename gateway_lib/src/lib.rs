#![allow(clippy::redundant_field_names)]

mod auth;
mod api_client;
mod identity;

pub use self::auth::*;
pub use self::error::*;
pub use self::api_client::*;
pub use self::identity::*;

pub mod error;

pub const HEADER_CHARACTER_ID: &str   = "X-SF-CharacterId";
pub const HEADER_CORPORATION_ID: &str = "X-SF-CorporationId";
pub const HEADER_ALLIANCE_ID: &str    = "X-SF-AllianceId";
pub const HEADER_IS_ADMIN: &str       = "X-SF-IsAdmin";
pub const HEADER_SOURCE: &str         = "X-SF-Source";
pub const HEADER_SERVICE: &str        = "X-SF-Service";


// TODO: Remove once this_error implements it
// https://github.com/dtolnay/thiserror/issues/424
// https://github.com/dtolnay/thiserror/pull/431
// boxed_from!(Error::GatewayClientError, starfoundry_lib_gateway::error::Error);
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

