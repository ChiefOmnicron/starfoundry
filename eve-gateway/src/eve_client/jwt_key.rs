use serde::Deserialize;
use url::Url;

use crate::eve_client::error::{EveApiError, Result};
use crate::eve_client::EveApiClient;

impl EveApiClient {
    pub async fn jwt_keys(
        jwt_key_url: Url,
    ) -> Result<EveApiJwtKeys> {
        #[derive(Debug, Deserialize)]
        struct Response {
            keys: Vec<EveApiJwtKey>,
        }

        reqwest::get(jwt_key_url)
            .await
            .map_err(EveApiError::FetchEveJwtToken)?
            .json::<Response>()
            .await
            .map(|x| EveApiJwtKeys(x.keys))
            .map_err(|e| {
                EveApiError::FetchEveJwtToken(e)
            })
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct EveApiJwtKey {
    e:   Option<String>,
    n:   Option<String>,
    alg: String,
    kty: String,
}

pub struct EveApiJwtKeys(Vec<EveApiJwtKey>);

impl EveApiJwtKeys {
    pub fn rs256(
        &self
    ) -> Option<(String, String)> {
        self.0
            .iter()
            .find(|x| x.alg == "RS256" && x.kty == "RSA")
            .map(|x| {
                let x = x.clone();
                (x.n.unwrap_or_default(), x.e.unwrap_or_default())
            })
    }
}
