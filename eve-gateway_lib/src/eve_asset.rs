mod asset;
mod blueprint;
mod location_flag;

pub use self::asset::*;
pub use self::blueprint::*;
pub use self::location_flag::*;

use starfoundry_lib_gateway::{ApiClient, HEADER_CHARACTER_ID};

use starfoundry_lib_types::{CharacterId, CorporationId};
use axum::http::{HeaderMap, HeaderValue};
use reqwest::header::HOST;

use crate::Result;

pub trait EveGatewayApiClientEveAsset: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn eve_fetch_character_assets(
        &self,
        source:       String,
        character_id: CharacterId,
    ) -> Result<Vec<Asset>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (*character_id).into());

        self
            .fetch_auth(
                &format!("eve/characters/{}/assets", character_id),
                &(),
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn eve_fetch_corporation_assets(
        &self,
        source:         String,
        character_id:   CharacterId,
        corporation_id: CorporationId,
    ) -> Result<Vec<Asset>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (*character_id).into());

        self
            .fetch_auth(
                &format!("eve/corporations/{}/assets", corporation_id),
                &(),
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn eve_fetch_character_blueprints(
        &self,
        source:       String,
        character_id: CharacterId,
    ) -> Result<Vec<Blueprint>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (*character_id).into());

        self
            .fetch_auth(
                &format!("eve/characters/{}/assets/blueprints", character_id),
                &(),
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn eve_fetch_corporation_blueprints(
        &self,
        source:         String,
        character_id:   CharacterId,
        corporation_id: CorporationId,
    ) -> Result<Vec<Blueprint>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (*character_id).into());

        self
            .fetch_auth(
                &format!("eve/corporations/{}/assets/blueprints", corporation_id),
                &(),
                headers,
            )
            .await
            .map_err(Into::into)
    }
}
