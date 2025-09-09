use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};
use serde::Deserialize;
use std::sync::Arc;

use crate::{CharacterInfo, Result, ENV_EVE_GATEWAY_API_URL};
use crate::auth::error::AuthError;

pub const ENV_EVE_GATEWAY_JWK_URL: &str = "STARFOUNDRY_EVE_GATEWAY_JWK_URL";

/// Validates the token and returns the claims from the token if it is valid
/// 
pub fn verify(
    access_token: &str,
    audience:     &str,
    decoding_key: Arc<DecodingKey>,
) -> Result<TokenData<Claims>> {
    let mut validation = Validation::new(Algorithm::ES256);
    validation.set_required_spec_claims(&["exp", "iss", "aud", "kid"]);
    validation.set_issuer(&[
        gateway_url()?,
    ]);
    validation.set_audience(&[
        audience,
    ]);
    validation.validate_exp = true;
    validation.validate_aud = true;

    jsonwebtoken::decode::<Claims>(
        &access_token,
        &decoding_key,
        &validation
    )
    .map_err(AuthError::InvalidAccessToken)
    .map_err(Into::into)
}

fn gateway_url() -> Result<String> {
    if cfg!(test) {
        return Ok("https://test.starfoundry.space".into());
    }

    std::env::var(ENV_EVE_GATEWAY_API_URL)
        .map_err(|_| AuthError::MissingEnv(ENV_EVE_GATEWAY_API_URL.into()).into())
}

/// Decoded access token
#[derive(Debug, Deserialize)]
pub struct Claims {
    /// determines if the character is an admin for the application
    pub is_admin:       bool,
    /// General character information
    pub character_info: CharacterInfo,
}
