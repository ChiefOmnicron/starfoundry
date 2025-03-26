use base64::{Engine as _, engine::general_purpose};
use serde::Deserialize;
use starfoundry_libs_types::CharacterId;

use crate::Error;

/// Decoded access token
#[derive(Debug, Deserialize)]
pub struct EveOAuthPayload {
    /// List of all permissions that where granted
    pub scp: Scp,
    /// User identification
    pub sub: String,
    /// Audience
    pub aud: Vec<String>,
    /// URI of the SSO
    pub iss: String,
}

impl EveOAuthPayload {
    /// Validates that the given payload is valid and comes from eve
    pub fn is_valid(&self) -> bool {
        let mut result = true;

        if !self.aud.contains(&"EVE Online".into()) {
            tracing::error!("Invalid Audience");
            result = false
        }
        // TODO: update to https://login.eveonline.com
        if !self.iss.contains("login.eveonline.com") {
            tracing::error!("Invalid ISS");
            result = false
        }

        result
    }
}

/// Parses the scp field in the payload
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Scp {
    /// The permission is a single String
    Str(String),
    /// The permission is a list of strings
    StrArray(Vec<String>),
}

impl Scp {
    /// Gets the inner string or vec of string and returns it as vec
    pub fn into_vec(self) -> Vec<String> {
        match self {
            Self::Str(x) => vec![x],
            Self::StrArray(x) => x,
        }
    }
}

/// Parsed version of the response from EVE after a successfull login.
///
#[derive(Debug, Deserialize)]
pub struct EveOAuthToken {
    /// Access token required for every request
    pub access_token: String,
    /// Type of the token
    pub token_type: String,
    /// Lifetime of the token, typically 1199 seconds
    pub expires_in: i32,
    /// Token to get a new access token
    pub refresh_token: String,
}

impl EveOAuthToken {
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
    pub fn payload(&self) -> Result<EveOAuthPayload, Error> {
        let payload = self.access_token.to_string();
        let payload = payload.split('.').collect::<Vec<_>>();
        let payload = payload.get(1).copied().unwrap_or_default();
        let decoded = general_purpose::STANDARD_NO_PAD
            .decode(payload)
            .map_err(Error::OAuthPayloadDecode)?;
        serde_json::from_slice(&decoded).map_err(Error::OAuthParseError)
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
    pub fn character_id(&self) -> Result<CharacterId, Error> {
        let character_id = self
            .payload()?
            .sub
            .replace("CHARACTER:EVE:", "")
            .parse::<i32>()
            .map_err(Error::OAuthParseCharacterId)?;
        Ok(character_id.into())
    }
}

