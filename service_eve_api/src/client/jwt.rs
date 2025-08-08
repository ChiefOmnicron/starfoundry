use base64::{Engine as _, engine::general_purpose};
use serde::Deserialize;
use starfoundry_libs_types::CharacterId;
use crate::client::{EveApiClient, EveApiError, EveClientId, Result};
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};
use url::Url;

/// Parsed version of the response from EVE after a successful login.
///
#[derive(Debug, Deserialize)]
pub struct EveJwtToken {
    /// Access token required for every request
    pub access_token: String,
    /// Token to get a new access token
    pub refresh_token: String,
}

impl EveJwtToken {
    /// Validates the token and returns the claims from the token if it is valid
    /// 
    pub async fn validate(
        &self,
        jwt_key_url: Url,
        client_id:   EveClientId,
    ) -> Result<TokenData<Claims>> {
        let jwt_keys = EveApiClient::jwt_keys(jwt_key_url).await?;

        let rs256_key = if let Some((modulus, exponent)) = jwt_keys.rs256() {
            DecodingKey::from_rsa_components(
                &modulus,
                &exponent,
            )
            .map_err(|_| EveApiError::InvalidRS256Key)?
        } else {
            tracing::error!("no rs256 key");
            return Err(EveApiError::NoRs256Key);
        };

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_required_spec_claims(&["exp", "iss", "aud"]);
        validation.set_issuer(&[
            "https://login.eveonline.com",
            "login.eveonline.com",
        ]);
        validation.set_audience(&[
            "EVE Online",
            &client_id,
        ]);

        jsonwebtoken::decode::<Claims>(
            &self.access_token,
            &rs256_key,
            &validation
        )
        .map_err(EveApiError::ParseEveJwtAccessToken)
    }

    /// Extracts the payload from the access token
    ///
    /// # Errors
    ///
    /// Fails when the payload could not be decoded or parsed
    ///
    /// # Returns
    ///
    /// Payload of the access token
    ///
    #[deprecated]
    pub fn payload(&self) -> Result<Claims, EveApiError> {
        let payload = self.access_token.to_string();
        let payload = payload.split('.').collect::<Vec<_>>();
        let payload = payload.get(1).copied().unwrap_or_default();
        let decoded = general_purpose::STANDARD_NO_PAD
            .decode(payload)
            .map_err(EveApiError::OAuthPayloadDecode)?;
        serde_json::from_slice(&decoded).map_err(EveApiError::OAuthParseError)
    }

    /// Gets the character id from the token
    ///
    /// # Errors
    ///
    /// Fails when either getting the payload fails or the user identification
    /// could not be parsed
    ///
    /// # Returns
    ///
    /// The character id
    ///
    pub fn character_id(&self) -> Result<CharacterId, EveApiError> {
        let character_id = self
            .payload()?
            .sub
            .replace("CHARACTER:EVE:", "")
            .parse::<i32>()
            .map_err(EveApiError::OAuthParseCharacterId)?;
        Ok(character_id.into())
    }
}

/// Decoded access token
#[derive(Debug, Deserialize)]
pub struct Claims {
    /// List of all permissions that where granted
    pub scp: ClaimScp,
    /// User identification
    pub sub: String,
    /// Audience
    pub aud: Vec<String>,
    /// URI of the SSO
    pub iss: String,
}

impl Claims {
    // TODO: add rsa validation
    /// Validates that the given payload is valid and comes from eve
    #[deprecated]
    pub fn is_valid(
        &self,
    ) -> bool {
        let mut result = true;

        if
            !self.iss.contains("https://login.eveonline.com") ||
            !self.iss.contains("login.eveonline.com")
        {
            tracing::error!("Invalid ISS");
            result = false
        }

        if !self.aud.contains(&"EVE Online".into()) {
            tracing::error!("Invalid Audience, does not include 'Eve Online'");
            result = false
        }

        /*if !self.aud.contains(&EveApiClient::client_id()) {
            tracing::error!("Invalid Audience, does not include our client_id");
            result = false
        }*/

        result
    }
}

/// Parses the scp field in the payload
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ClaimScp {
    /// The permission is a single String
    Str(String),
    /// The permission is a list of strings
    StrArray(Vec<String>),
}

impl ClaimScp {
    /// Gets the inner string or vec of string and returns it as vec
    pub fn into_vec(self) -> Vec<String> {
        match self {
            Self::Str(x) => vec![x],
            Self::StrArray(x) => x,
        }
    }
}
