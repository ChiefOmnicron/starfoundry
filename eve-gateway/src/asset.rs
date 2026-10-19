mod models;
pub use self::models::*;

pub mod eve_resolve_character_asset;
pub mod eve_resolve_corporation_asset;

pub mod list_blueprints;

mod error;

use starfoundry_gateway::ApiClient;
use starfoundry_lib_types::{CharacterId, CorporationId, ItemId};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::Result;
use crate::client::ApiClientExtended;
use crate::state::AppState;

/// returns all routes that are under the path `/assets`
pub fn routes() -> OpenApiRouter<AppState> {
    let list_blueprints =
        OpenApiRouter::new().routes(routes!(self::list_blueprints::api));

    OpenApiRouter::new().merge(list_blueprints)
}

pub trait EveGatewayApiClientAsset: ApiClient + ApiClientExtended {
    #[allow(async_fn_in_trait)]
    async fn list_character_assets(&self) -> Result<Vec<Asset>> {
        self.fetch_page("proxy/auth/characters/assets")
            .await
    }

    #[allow(async_fn_in_trait)]
    async fn list_character_blueprints(
        &self
    ) -> Result<Vec<EveBlueprintResponse>> {
        self.fetch_page("proxy/auth/characters/blueprints")
            .await
    }

    #[allow(async_fn_in_trait)]
    async fn list_corporation_assets(&self) -> Result<Vec<Asset>> {
        self.fetch_page("proxy/auth/corporations/assets")
            .await
    }

    #[allow(async_fn_in_trait)]
    async fn list_corporation_blueprints(
        &self
    ) -> Result<Vec<EveBlueprintResponse>> {
        self.fetch_page("proxy/auth/corporations/blueprints")
            .await
    }

    #[allow(async_fn_in_trait)]
    async fn eve_resolve_character_asset(
        &self,
        character_id: CharacterId,
        assets: Vec<ItemId>,
    ) -> Result<Vec<ResolvedItem>> {
        self.post(&format!("eve/characters/{}/assets", character_id,), assets)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn eve_resolve_corporation_asset(
        &self,
        corporation_id: CorporationId,
        assets: Vec<ItemId>,
    ) -> Result<Vec<ResolvedItem>> {
        self.post(
            &format!("eve/corporations/{}/assets", corporation_id,),
            assets,
        )
        .await
        .map_err(Into::into)
    }
}
