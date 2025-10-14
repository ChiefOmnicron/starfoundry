mod auth_callback;
mod auth_login_callback;
mod auth_login;
mod auth_token;
mod generic_delete;
mod generic_get;
mod generic_post;
mod generic_put;
mod store_general_info;
mod well_known_jwks;

pub use self::auth_callback::*;
pub use self::auth_login::*;
pub use self::auth_login_callback::*;
pub use self::auth_token::*;
pub use self::generic_delete::*;
pub use self::generic_get::*;
pub use self::generic_post::*;
pub use self::generic_put::*;
pub use self::store_general_info::*;
pub use self::well_known_jwks::*;

use axum::http::HeaderMap;

// TODO: library for use in other services
pub const HEADER_CHARACTER_ID: &str   = "X-SF-CharacterId";
pub const HEADER_CORPORATION_ID: &str = "X-SF-CorporationId";
pub const HEADER_ALLIANCE_ID: &str    = "X-SF-AllianceId";
pub const HEADER_IS_ADMIN: &str       = "X-SF-IsAdmin";

pub fn add_headers(
    headers:        &mut HeaderMap,
    character_id:   i32,
    corporation_id: i32,
    alliance_id:    Option<i32>,
    is_admin:       bool,
) {
    headers.insert(HEADER_CHARACTER_ID, character_id.into());
    headers.insert(HEADER_CORPORATION_ID, corporation_id.into());

    if let Some(x) = alliance_id {
        headers.insert(HEADER_ALLIANCE_ID, x.into());
    }

    if is_admin {
        headers.insert(HEADER_IS_ADMIN, (true as i32).into());
    }
}
