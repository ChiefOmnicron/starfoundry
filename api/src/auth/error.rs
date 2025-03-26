#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    InvalidIdentity,

    CreateAuthClient(starfoundry_libs_eve_api::Error),
    ConnectError(starfoundry_libs_eve_api::Error),
    SqlError(sqlx::Error),

    // Callback errors
    InvalidCode,
    InvalidState,
    InvalidIntention,
    InvalidCharacter,
    CannotGetIntentionToken(sqlx::Error),
    UnknownCharacter(sqlx::Error),
    CannotUpdateLogin(sqlx::Error),
    CannotUpdateEsiTokens(sqlx::Error),

    // Secure token errors
    MissingEnvSecretKey,
    HmacInitError,
    InvalidBase64,

    JsonWebTokenEncode(jsonwebtoken::errors::Error,),
    JsonWebTokenDecode(jsonwebtoken::errors::Error,),

    // FIXME: proper error
    FixMe
}

impl warp::reject::Reject for AuthError {}

impl From<sqlx::Error> for AuthError {
    fn from(x: sqlx::Error) -> Self {
        Self::SqlError(x)
    }
}

impl From<starfoundry_libs_eve_api::Error> for AuthError {
    fn from(x: starfoundry_libs_eve_api::Error) -> Self {
        Self::ConnectError(x)
    }
}
