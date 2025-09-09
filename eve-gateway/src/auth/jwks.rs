use axum::Json;
use axum::response::IntoResponse;
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};
use jwk_kit::generator::ecdsa::extract_es256_coordinates;
use jwk_kit::jwk::{create_jwks, JwkBuilder};
use reqwest::StatusCode;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::api_docs::InternalServerError;
use crate::auth::error::{AuthError, Result};
use crate::auth::{issuer, public_ecdsa_key};
use crate::character::CharacterInfo;

/// JWKS
/// 
/// ---
/// 
/// Returns the public keys for validating the JWT signature.
/// 
#[utoipa::path(
    get,
    path = "/jwks",
    tag = "Auth",
    responses(
        (
            status = OK,
            description = "Current keys used",
            body = Jwks,
            content_type = "application/json",
        ),
        InternalServerError,
    ),
)]
pub async fn jwks_json() -> Result<impl IntoResponse> {
    let public_pem = public_ecdsa_key()?;
    let (x, y) = extract_es256_coordinates(&public_pem)
        .map_err(AuthError::EcPublicKeyExtractXY)?;

        let ec_jwk = JwkBuilder::new("EC")
        .set_key_use("sig")
        .set_algorithm("ES256")
        .set_key_id("starfoundry-eve-gateway")
        .set_curve_type("P-256")
        .set_x_coordinate(&x)
        .set_y_coordinate(&y)
        .build()
        .map_err(AuthError::EcPublicKeyBuildResponse)?;
    let jwks = create_jwks(vec![ec_jwk]);

    Ok((
        StatusCode::OK,
        Json(jwks),
    ).into_response())
}

pub fn verify(
    access_token: &str,
    audience:     Vec<String>,
) -> Result<TokenData<Claims>> {
    let public_pem = public_ecdsa_key()?;
    let (x, y) = extract_es256_coordinates(&public_pem)
        .map_err(AuthError::EcPublicKeyExtractXY)?;

    let decoding_key = DecodingKey::from_ec_components(
            &x,
            &y,
        )
        .map_err(|e| AuthError::InvalidES256Key(e))?;

    let mut validation = Validation::new(Algorithm::ES256);
    validation.set_required_spec_claims(&["exp", "iss", "aud", "kid"]);
    validation.set_issuer(&[
        issuer()?,
    ]);
    validation.set_audience(&audience);
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

/// Decoded access token
#[derive(Debug, Deserialize)]
pub struct Claims {
    /// General character information
    pub character_info: CharacterInfo,
}

// only required as a type hint for utoipa
#[allow(dead_code)]
#[derive(ToSchema)]
#[schema(example = json!({
    "keys": [{
        "kty": "EC",
        "use": "sig",
        "alg": "ES256",
        "kid": "starfoundry-eve-gateway",
        "crv": "P-256",
        "x": "E_F29AWYozaY4fzVjaeSfgASXmHJqKwjmT-4foXMPHc",
        "y": "pmIsw6PA0hFO1JIzPjNqkXN6dDEIk8LYLdtH9Vq1Qj4"
    }]
}))]
struct Jwks {
    keys: Vec<Jwk>
}

// only required as a type hint for utoipa
#[allow(dead_code)]
#[derive(ToSchema)]
struct Jwk {
    kty:   String,
    r#use: String,
    alg:   String,
    kid:   String,
    crv:   String,
    x:     String,
    y:     String,
}
