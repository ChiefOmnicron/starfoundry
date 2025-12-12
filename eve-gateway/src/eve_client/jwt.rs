use serde::Deserialize;
use starfoundry_lib_types::CharacterId;
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};
use url::Url;

use crate::eve_client::error::{EveApiError, Result};
use crate::eve_client::utils::single_or_vec;
use crate::eve_client::{EveApiClient, EveClientId};

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
        validation.set_required_spec_claims(&["exp", "iss", "aud", "kid"]);
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
    pub fn extract_character_id(
        claim: &Claims,
    ) -> Result<CharacterId, EveApiError> {
        let character_id = claim
            .sub
            .replace("CHARACTER:EVE:", "")
            .parse::<i32>()
            .map_err(EveApiError::OAuthParseCharacterId)?;
        Ok(character_id.into())
    }

    /// Gets all granted scopes
    ///
    /// # Returns
    ///
    /// The character id
    ///
    pub fn extract_scopes(
        claim: &Claims,
    ) -> Vec<String> {
        claim.scp.clone()
    }
}

/// Decoded access token
#[derive(Debug, Deserialize)]
pub struct Claims {
    /// User identification
    pub sub: String,
    /// User scope
    #[serde(deserialize_with = "single_or_vec")]
    pub scp: Vec<String>,
}
