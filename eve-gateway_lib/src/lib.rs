mod auth;
mod character;
mod api_client;
mod item;
mod universe;

pub use self::auth::*;
pub use self::character::*;
pub use self::error::*;
pub use self::api_client::*;
pub use self::item::*;
pub use self::universe::*;

pub mod error;
pub mod test;

pub const HEADER_CHARACTER_ID: &str   = "X-SF-CharacterId";
pub const HEADER_CORPORATION_ID: &str = "X-SF-CorporationId";
pub const HEADER_ALLIANCE_ID: &str    = "X-SF-AllianceId";
pub const HEADER_IS_ADMIN: &str       = "X-SF-IsAdmin";
