mod callback;
mod error;
mod extractor;
mod identity;
mod jwt;
mod login_alt;
mod login_corporation;
mod login;
mod refresh_token;
mod scopes;
mod whoami;

pub use self::callback::*;
pub use self::extractor::*;
pub use self::identity::*;
pub use self::jwt::*;
pub use self::login_alt::*;
pub use self::login_corporation::*;
pub use self::login::*;
pub use self::refresh_token::*;
pub use self::scopes::*;
pub use self::whoami::*;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(callback))
        .routes(routes!(login_alt))
        .routes(routes!(login_corporation))
        .routes(routes!(login))
        .routes(routes!(refresh_token))
        .routes(routes!(whoami))
}
