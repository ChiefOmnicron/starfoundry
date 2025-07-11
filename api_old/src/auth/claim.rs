use jsonwebtoken::{EncodingKey, Header, encode, decode, DecodingKey, Validation, TokenData};
use serde::{Deserialize, Serialize};
use starfoundry_libs_types::CharacterId;

use super::AuthError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    pub exp:          i64,

    pub character_id: CharacterId,
}

pub fn generate_jwt(
    claim: Claim,
) -> Result<String, AuthError> {
    let secret = std::env::var("SECRET_KEY")
        .map_err(|_| AuthError::MissingEnvSecretKey)?;

    encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(secret.as_ref())
        )
        .map_err(AuthError::JsonWebTokenEncode)
}

pub fn validate_jwt(
    token: String,
) -> Result<TokenData<Claim>, AuthError> {
    let secret = std::env::var("SECRET_KEY")
        .map_err(|_| AuthError::MissingEnvSecretKey)?;

    decode::<Claim>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .map_err(AuthError::JsonWebTokenDecode)
}
