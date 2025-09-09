use serde::Deserialize;
use url::Url;
use jsonwebtoken::DecodingKey;
use crate::Result;
use crate::auth::error::AuthError;

#[derive(Clone, Debug, Deserialize)]
pub struct JwtKey {
    x:   Option<String>,
    y:   Option<String>,
    alg: String,
    kty: String,
}

pub struct EveApiJwtKeys(Vec<JwtKey>);

impl EveApiJwtKeys {
    pub fn es256(
        &self
    ) -> Option<(String, String)> {
        self.0
            .iter()
            .find(|x| x.alg == "ES256" && x.kty == "EC")
            .map(|x| {
                let x = x.clone();
                (x.x.unwrap_or_default(), x.y.unwrap_or_default())
            })
    }
}

pub async fn load_signature(
    jwt_key_url: Url,
) -> Result<DecodingKey> {
    #[derive(Debug, Deserialize)]
    struct Response {
        keys: Vec<JwtKey>,
    }

    let jwt_keys = reqwest::get(jwt_key_url)
        .await
        .map_err(AuthError::FetchJwtKey)?
        .json::<Response>()
        .await
        .map(|x| EveApiJwtKeys(x.keys))
        .map_err(AuthError::FetchJwtKey)?;

    if let Some((x, y)) = jwt_keys.es256() {
        DecodingKey::from_ec_components(
            &x,
            &y,
        )
        .map_err(|e| AuthError::InvalidES256Key(e).into())
    } else {
        tracing::error!("no es256 key");
        return Err(AuthError::NoEs256Key.into());
    }
}
