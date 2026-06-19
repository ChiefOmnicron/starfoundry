use std::ops::Deref;

use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use starfoundry_lib_eve_gateway::CharacterInfo;
use starfoundry_lib_types::CharacterId;

use crate::auth::error::{AuthError, Result};

pub const ENV_JWT_ECDSA_PRIVATE: &str  = "STARFOUNDRY_EVE_GATEWAY_JWT_ECDSA_PRIVATE";
pub const ENV_JWT_ECDSA_PUBLIC: &str   = "STARFOUNDRY_EVE_GATEWAY_JWT_ECDSA_PUBLIC";
pub const ENV_JWT_ISSUER_DOMAIN: &str  = "STARFOUNDRY_EVE_GATEWAY_JWT_ISSUER_DOMAIN";

const ACCESS_TOKEN_EXP: Duration  = Duration::minutes(15);
const REFRESH_TOKEN_EXP: Duration = Duration::days(1);
const JWT_KID: &str               = "starfoundry-eve-gateway";

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    exp:            i64,
    iat:            i64,

    aud:            Vec<String>,
    iss:            String,
    kid:            String,

    is_admin:       bool,
    character_info: CharacterInfo,
    key_type:       KeyType,

    pub sub:        CharacterId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    exp:     i64,
    iat:     i64,

    iss:     String,

    pub sub: CharacterId,
}

impl RefreshTokenClaims {
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

pub fn issuer() -> Result<String> {
    std::env::var(ENV_JWT_ISSUER_DOMAIN)
        .map_err(|_| AuthError::EnvNotSet(ENV_JWT_ISSUER_DOMAIN.into()))
}

fn private_ecdsa_key() -> Result<String> {
    std::env::var(ENV_JWT_ECDSA_PRIVATE)
        .map_err(|_| AuthError::EnvNotSet(ENV_JWT_ECDSA_PRIVATE.into()))
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KeyType {
    /// The JWT token is from a character
    Character,
    /// The JWT token is from a service
    Service,
}

#[derive(Debug, Serialize)]
pub struct JwtToken(String);

impl JwtToken {
    pub fn new_access_token(
        character_id:   CharacterId,
        character_info: CharacterInfo,
        is_admin:       bool,
        host:           String,
    ) -> Result<Self> {
        let exp = (
                Utc::now().naive_utc() + ACCESS_TOKEN_EXP
            )
            .and_utc()
            .timestamp();
        let iat = (Utc::now().naive_utc())
            .and_utc()
            .timestamp();

        let claims = AccessTokenClaims {
            exp,
            iat,

            aud: vec![
                host,
            ],
            sub: character_id,
            iss: issuer()?,
            kid: JWT_KID.into(),

            key_type: KeyType::Character,

            character_info,
            is_admin,
        };
        let claims = serde_json::to_value(claims).unwrap_or_default();

        Self::generate_token(claims)
    }

    pub fn new_refresh_token(
        character_id: CharacterId,
    ) -> Result<JwtToken> {
        let exp = (
                Utc::now().naive_utc() + REFRESH_TOKEN_EXP
            )
            .and_utc()
            .timestamp();
        let iat = (Utc::now().naive_utc())
            .and_utc()
            .timestamp();

        let claims = RefreshTokenClaims {
            exp,
            iat,

            sub: character_id,
            iss: issuer()?,
        };
        let claims = serde_json::to_value(claims).unwrap_or_default();

        Self::generate_token(claims)
    }

    fn generate_token(
        claims: serde_json::Value,
    ) -> Result<JwtToken> {
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
            .map(JwtToken)
            .map_err(AuthError::JsonWebTokenEncode)
    }
}

impl Deref for JwtToken {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
