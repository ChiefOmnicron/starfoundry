use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use starfoundry_lib_types::CharacterId;

use crate::auth::error::{AuthError, Result};
use crate::character::CharacterInfo;

const ACCESS_TOKEN_EXP: Duration  = Duration::minutes(15);
const REFRESH_TOKEN_EXP: Duration = Duration::days(1);
const JWT_KID: &str               = "starfoundry-eve-gateway";

pub const ENV_JWT_ECDSA_PRIVATE: &str  = "STARFOUNDRY_EVE_GATEWAY_JWT_ECDSA_PRIVATE";
pub const ENV_JWT_ECDSA_PUBLIC: &str   = "STARFOUNDRY_EVE_GATEWAY_JWT_ECDSA_PUBLIC";
pub const ENV_JWT_ISSUER_DOMAIN: &str  = "STARFOUNDRY_EVE_GATEWAY_JWT_ISSUER_DOMAIN";

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    exp:            i64,
    iat:            i64,

    aud:            Vec<String>,
    iss:            String,
    kid:            String,

    is_admin:       bool,
    character_info: CharacterInfo,

    pub sub:        CharacterId,
}

impl AccessTokenClaims {
    pub fn new(
        character_id:   CharacterId,
        character_info: CharacterInfo,
        is_admin:       bool,
        host:           String,
    ) -> Result<String> {
        let exp = (
                Utc::now().naive_utc() + ACCESS_TOKEN_EXP
            )
            .and_utc()
            .timestamp();
        let iat = (Utc::now().naive_utc())
            .and_utc()
            .timestamp();

        let claims = Self {
            exp,
            iat,

            aud: vec![
                host,
            ],
            sub: character_id,
            iss: issuer()?,
            kid: JWT_KID.into(),

            character_info,
            is_admin,
        };
        let claims = serde_json::to_value(claims).unwrap_or_default();

        token(claims)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    exp:     i64,
    iat:     i64,

    iss:     String,

    pub sub: CharacterId,
}

impl RefreshTokenClaims {
    pub fn new(
        character_id: CharacterId,
    ) -> Result<String> {
        let exp = (
                Utc::now().naive_utc() + REFRESH_TOKEN_EXP
            )
            .and_utc()
            .timestamp();
        let iat = (Utc::now().naive_utc())
            .and_utc()
            .timestamp();

        let claims = Self {
            exp,
            iat,

            sub: character_id,
            iss: issuer()?,
        };
        let claims = serde_json::to_value(claims).unwrap_or_default();

        token(claims)
    }

    pub fn verify(
        token:  &str,
    ) -> Result<TokenData<Self>> {
        let ec_pem = DecodingKey::from_ec_pem(
                public_ecdsa_key()?.as_bytes()
            )
            .map_err(AuthError::LoadEcPem)?;

        let validation = Validation::new(Algorithm::ES256);
        decode::<Self>(
                &token,
                &ec_pem,
                &validation,
            )
            .map_err(AuthError::JsonWebTokenDecode)
    }
}

pub fn public_ecdsa_key() -> Result<String> {
    std::env::var(ENV_JWT_ECDSA_PUBLIC)
        .map_err(|_| AuthError::EnvNotSet(ENV_JWT_ECDSA_PUBLIC.into()))
}

fn private_ecdsa_key() -> Result<String> {
    std::env::var(ENV_JWT_ECDSA_PRIVATE)
        .map_err(|_| AuthError::EnvNotSet(ENV_JWT_ECDSA_PRIVATE.into()))
}

fn issuer() -> Result<String> {
    std::env::var(ENV_JWT_ISSUER_DOMAIN)
        .map_err(|_| AuthError::EnvNotSet(ENV_JWT_ISSUER_DOMAIN.into()))
}

fn token(
    claims: serde_json::Value,
) -> Result<String> {
    let ec_pem = EncodingKey::from_ec_pem(
            private_ecdsa_key()?.as_bytes()
        )
        .map_err(AuthError::LoadEcPem)?;
    let header = Header::new(Algorithm::ES256);

    encode(
            &header,
            &claims,
            &ec_pem
        )
        .map_err(AuthError::JsonWebTokenEncode)
}
