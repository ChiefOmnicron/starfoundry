mod blueprints;

pub use self::blueprints::*;

use starfoundry_lib_gateway::ApiClient;

use crate::{ApiClientExtended, Asset, EveBlueprintResponse, Result};

pub trait EveGatewayApiClientAsset: ApiClient + ApiClientExtended {
    #[allow(async_fn_in_trait)]
    async fn list_character_assets(
        &self,
    ) -> Result<Vec<Asset>> {
        self
            .fetch_page(
                "proxy/auth/characters/assets",
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_character_blueprints(
        &self,
    ) -> Result<Vec<EveBlueprintResponse>> {
        self
            .fetch_page(
                "proxy/auth/characters/blueprints",
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_corporation_assets(
        &self,
    ) -> Result<Vec<Asset>> {
        self
            .fetch_page(
                "proxy/auth/corporations/assets",
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_corporation_blueprints(
        &self,
    ) -> Result<Vec<EveBlueprintResponse>> {
        self
            .fetch_page(
                "proxy/auth/corporations/blueprints",
            )
            .await
            .map_err(Into::into)
    }
}
