use std::sync::Arc;

use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};
use serde::Deserialize;

use crate::auth::CharacterInfo;
use crate::error::{GatewayError, Result};

pub const ENV_EVE_GATEWAY_JWT_SIGN: &str = "STARFOUNDRY_EVE_GATEWAY_JWT_SIGN";
pub const ENV_EVE_GATEWAY_JWK_URL: &str = "STARFOUNDRY_EVE_GATEWAY_JWK_URL";

/// Validates the token and returns the claims from the token if it is valid
pub fn verify(
    access_token: &str,
    audience: &str,
    decoding_key: Arc<DecodingKey>,
) -> Result<TokenData<Claims>> {
    let mut validation = Validation::new(Algorithm::ES256);
    validation.set_required_spec_claims(&["exp", "iss", "aud", "kid"]);
    validation.set_issuer(&[gateway_url()?]);
    validation.set_audience(&[audience]);
    validation.validate_exp = true;
    validation.validate_aud = true;

    jsonwebtoken::decode::<Claims>(&access_token, &decoding_key, &validation)
        .map_err(GatewayError::InvalidAccessToken)
}

fn gateway_url() -> Result<String> {
    std::env::var(ENV_EVE_GATEWAY_JWT_SIGN).map_err(|_| {
        GatewayError::EnvNotSet(ENV_EVE_GATEWAY_JWT_SIGN.into()).into()
    })
}

/// Decoded access token
#[derive(Debug, Deserialize)]
pub struct Claims {
    /// determines if the character is an admin for the application
    pub is_admin:       bool,
    /// General character information
    pub character_info: CharacterInfo,
}
