use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode, decode, DecodingKey, Validation, TokenData};
use serde::{Deserialize, Serialize};
use starfoundry_libs_types::CharacterId;

use crate::auth::error::{AuthError, Result};

const ISS: &str = "StarFoundry";

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtToken {
    exp: i64,
    iss: String,

    sub: CharacterId,
}

impl JwtToken {
    pub fn new(
        character_id: CharacterId,
    ) -> Self {
        let exp = (
                Utc::now().naive_utc() + Duration::minutes(20)
            )
            .and_utc()
            .timestamp();

        Self {
            exp,

            sub: character_id,
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

        decode::<JwtToken>(
                &token,
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::default(),
            )
            .map_err(AuthError::JsonWebTokenDecode)
    }

    pub fn character_id(&self) -> CharacterId {
        self.sub
    }
}

pub struct RefreshToken(String);

impl RefreshToken {
    
}
