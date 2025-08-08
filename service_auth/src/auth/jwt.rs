use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use starfoundry_libs_types::CharacterId;

use crate::auth::error::{AuthError, Result};

const ISS: &str = "StarFoundry";
const ACCESS_TOKEN_EXP: Duration = Duration::minutes(15);
const REFRESH_TOKEN_EXP: Duration = Duration::days(1);

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtToken {
    exp: i64,
    iat: i64,

    iss: String,
    sub: CharacterId,
}

impl JwtToken {
    pub fn new(
        character_id: CharacterId,
    ) -> Self {
        let exp = (
                Utc::now().naive_utc() + ACCESS_TOKEN_EXP
            )
            .and_utc()
            .timestamp();
        let iat = (Utc::now().naive_utc())
            .and_utc()
            .timestamp();

        Self {
            exp,
            iat,

            sub: character_id,
            // TODO: determine in config
            // TODO: also change to https://{domain}
            iss: ISS.into(),
        }
    }

    pub fn generate(
        self,
    ) -> Result<String> {
        // TODO: validate at application start
        let secret = std::env::var("SECRET_KEY")
            .map_err(|_| AuthError::MissingEnvSecretKey)?;

        encode(
                &Header::default(),
                &self,
                &EncodingKey::from_secret(secret.as_ref())
            )
            .map_err(AuthError::JsonWebTokenEncode)
    }

    pub fn verify(
        token: &str,
    ) -> Result<TokenData<JwtToken>, AuthError> {
        // TODO: validate additional claims
        // TODO: validate at application start
        let secret = std::env::var("SECRET_KEY")
            .map_err(|_| AuthError::MissingEnvSecretKey)?;

        let validation = Validation::new(Algorithm::HS256);
        decode::<JwtToken>(
                &token,
                &DecodingKey::from_secret(secret.as_ref()),
                &validation,
            )
            .map_err(AuthError::JsonWebTokenDecode)
    }

    pub fn character_id(&self) -> CharacterId {
        self.sub
    }
}
