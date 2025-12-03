mod callback;
mod extractor;
mod history;
mod jwks;
mod jwt;
mod login;
mod token;

pub mod error;

pub use self::callback::*;
pub use self::extractor::*;
pub use self::jwks::*;
pub use self::jwt::*;
pub use self::login::*;
pub use self::token::*;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// returns all routes under the path `/auth`
pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(callback))
        .routes(routes!(login))
        .routes(routes!(token))
}

/// returns all routes under the path `/.well-known`
pub fn well_known_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(jwks_json))
}
