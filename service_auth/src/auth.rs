mod callback;
mod error;
mod extractor;
mod identity;
mod jwt;
mod login_character;
mod login_corporation;
mod login;
mod refresh_token;
mod scopes;
mod whoami;

pub use self::callback::*;
pub use self::extractor::*;
pub use self::identity::*;
pub use self::jwt::*;
pub use self::login_character::*;
pub use self::login_corporation::*;
pub use self::login::*;
pub use self::refresh_token::*;
pub use self::scopes::*;
pub use self::whoami::*;

use axum::middleware;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    let login_alt = OpenApiRouter::new()
        .routes(routes!(login_character))
        .route_layer(middleware::from_fn(assert_login));

    let login_corporation = OpenApiRouter::new()
        .routes(routes!(login_corporation))
        .route_layer(middleware::from_fn(assert_login));

    let whoami = OpenApiRouter::new()
        .routes(routes!(whoami))
        .route_layer(middleware::from_fn(assert_login));

    OpenApiRouter::new()
        .routes(routes!(callback))
        .routes(routes!(login))
        .routes(routes!(refresh_token))
        .merge(login_alt)
        .merge(login_corporation)
        .merge(whoami)
}
