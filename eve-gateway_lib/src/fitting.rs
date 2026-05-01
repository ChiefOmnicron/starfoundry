mod fit;

pub use self::fit::*;

use starfoundry_lib_gateway::{ApiClient, HEADER_CHARACTER_ID};
use starfoundry_lib_types::{CharacterId, FittingId};
use axum::http::{HeaderMap, HeaderValue};
use reqwest::header::HOST;

use crate::Result;

pub trait EveGatewayApiClientEveFitting: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn eve_create_fit(
        &self,
        source:         String,
        character_id:   CharacterId,
        data:           EveFit,
    ) -> Result<EveFitResponse> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (*character_id).into());

        self
            .post_auth(
                &format!("eve/characters/{}/fittings", character_id),
                data,
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn eve_delete_fit(
        &self,
        source:         String,
        character_id:   CharacterId,
        fitting_id:     FittingId,
    ) -> Result<EveFitResponse> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (*character_id).into());

        self
            .delete_auth(
                &format!(
                    "eve/characters/{}/fittings/{}",
                    character_id,
                    fitting_id,
                ),
                headers,
            )
            .await
            .map_err(Into::into)
    }
}
