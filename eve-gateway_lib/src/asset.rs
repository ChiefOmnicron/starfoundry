mod blueprints;

pub use self::blueprints::*;

use axum::http::{HeaderMap, HeaderValue};
use reqwest::header::HOST;
use starfoundry_lib_gateway::{ApiClient, HEADER_CHARACTER_ID};
use starfoundry_lib_types::CharacterId;

use crate::Result;

pub trait EveGatewayApiClientAsset: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn fetch_blueprints(
        &self,
        source:       String,
        character_id: CharacterId,
    ) -> Result<Vec<Blueprint>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (*character_id).into());

        self
            .fetch_auth(
                "/assets/blueprints",
                &(),
                headers,
            )
            .await
            .map_err(Into::into)
    }
}
