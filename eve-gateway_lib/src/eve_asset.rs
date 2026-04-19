mod asset;
mod blueprint;
mod location_flag;
mod resolved_item;

pub use self::asset::*;
pub use self::blueprint::*;
pub use self::location_flag::*;
pub use self::resolved_item::*;

use starfoundry_lib_gateway::{ApiClient, HEADER_CHARACTER_ID};

use starfoundry_lib_types::{CharacterId, CorporationId, ItemId};
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
    ) -> Result<Vec<EveBlueprintResponse>> {
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
    ) -> Result<Vec<EveBlueprintResponse>> {
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

    #[allow(async_fn_in_trait)]
    async fn eve_resolve_character_asset(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        assets:         Vec<ItemId>,
    ) -> Result<Vec<ResolvedItem>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .post_auth(
                &format!("eve/characters/{}/assets", character_id),
                assets,
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn eve_resolve_corporation_asset(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        corporation_id: &CorporationId,
        assets:         Vec<ItemId>,
    ) -> Result<Vec<ResolvedItem>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .post_auth(
                &format!("eve/corporations/{}/assets", corporation_id),
                assets,
                headers,
            )
            .await
            .map_err(Into::into)
    }
}
