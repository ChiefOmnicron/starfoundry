use chrono::{Duration, Utc};
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{AllianceId, CharacterId, CorporationId};
use crate::{CharacterInfo, ENV_EVE_GATEWAY_API};

const JWT_EC_PRIVATE: &str = "-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQg/nCaLo8n7/yQw4IZ
X+021Zhco/ZVQK86VeSmNsGRFKyhRANCAASxb7I0zn2Ng7fvXP1JmLucUfrRTd8h
s2h5dOGHzHQZd2SRpHust9f3aDYFhOHJypF0NL/sJbYWQo6yqoscCVBb
-----END PRIVATE KEY-----";
pub const JWT_EC_PUBLIC: &str = "-----BEGIN PUBLIC KEY-----
MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEsW+yNM59jYO371z9SZi7nFH60U3f
IbNoeXThh8x0GXdkkaR7rLfX92g2BYThycqRdDS/7CW2FkKOsqqLHAlQWw==
-----END PUBLIC KEY-----";

/// The JWT-Tokens generated are only for testing purposes.
/// This is not secure in any way.
/// 
#[derive(Debug, Serialize, Deserialize)]
#[deprecated]
pub struct JwtTokenForTesting {
    iat: i64,
    exp: i64,
    iss: String,
    aud: Vec<String>,
    kid: String,

    sub: CharacterId,

    is_admin: bool,
    character_info: CharacterInfo,
}

impl JwtTokenForTesting {
    pub fn new(
        character_id: CharacterId,
    ) -> Self {
        let iat = Utc::now()
            .naive_utc()
            .and_utc()
            .timestamp();
        let exp = (
                Utc::now().naive_utc() + Duration::minutes(20)
            )
            .and_utc()
            .timestamp();

        Self {
            iat,
            exp,
            iss: "https://test.starfoundry.space".into(),
            kid: "starfoundry-eve-gateway".into(),
            aud: vec![
                "test.starfoundry.space".into()
            ],

            sub: character_id,

            is_admin: false,
            character_info: CharacterInfo {
                character_name: "Test character".into(),
                character_id: character_id,
                corporation_name: "Test corporation".into(),
                corporation_id: CorporationId(1),
                alliance_name: Some("Test alliance".into()),
                alliance_id: Some(AllianceId(1))
            },
        }
    }

    pub fn generate(
        self,
    ) -> String {
        let encoding_key = EncodingKey::from_ec_pem(JWT_EC_PRIVATE.as_bytes()).unwrap();

        let header = Header::new(jsonwebtoken::Algorithm::ES256);
        encode(
                &header,
                &self,
                &encoding_key,
            )
            .unwrap()
    }
}

pub fn decoding_key() -> DecodingKey {
    DecodingKey::from_ec_pem(JWT_EC_PUBLIC.as_bytes()).unwrap()
}

pub fn set_jwt_test_envs() {
    unsafe {
        std::env::set_var(ENV_EVE_GATEWAY_API, "https://test.starfoundry.space");
    }
}
