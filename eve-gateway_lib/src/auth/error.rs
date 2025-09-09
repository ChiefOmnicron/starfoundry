use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum AuthError {
    #[error("error while fetching JWT-Keys, '{0}'")]
    FetchJwtKey(reqwest::Error),
    #[error("invalid EC jwt key, '{0}'")]
    InvalidES256Key(jsonwebtoken::errors::Error),
    #[error("invalid access_token, '{0}'")]
    InvalidAccessToken(jsonwebtoken::errors::Error),
    #[error("no es256 key")]
    NoEs256Key,

    #[error("missing env, '{0}'")]
    MissingEnv(String),
}
