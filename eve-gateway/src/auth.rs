mod access_token;
mod callback;
mod error;
//mod extractor;
//mod identity;
mod jwks;
mod jwt;
mod login;
//mod login_character;
//mod login_corporation;
//mod scopes;
//mod whoami;

pub use self::access_token::*;
pub use self::callback::*;
//pub use self::extractor::*;
//pub use self::identity::*;
pub use self::jwks::*;
pub use self::jwt::*;
pub use self::login::*;
//pub use self::login_character::*;
//pub use self::login_corporation::*;
//pub use self::scopes::*;
//pub use self::whoami::*;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    /*let login_alt = OpenApiRouter::new()
        .routes(routes!(login_character))
        .route_layer(middleware::from_fn(assert_login));

    let login_corporation = OpenApiRouter::new()
        .routes(routes!(login_corporation))
        .route_layer(middleware::from_fn(assert_login));*/

    OpenApiRouter::new()
        .routes(routes!(access_token))
        .routes(routes!(callback))
        .routes(routes!(login))
        //.merge(login_alt)
        //.merge(login_corporation)
}

pub fn well_known_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(jwks_json))
}
