mod blueprints;

pub use self::blueprints::*;

use starfoundry_lib_gateway::ApiClient;

use crate::{Asset, EveBlueprintResponse, Result};

pub trait EveGatewayApiClientAsset: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn list_character_assets(
        &self,
    ) -> Result<Vec<Asset>> {
        self
            .fetch_auth(
                "proxy/list/auth/characters/assets",
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_character_blueprints(
        &self,
    ) -> Result<Vec<EveBlueprintResponse>> {
        self
            .fetch_auth(
                "proxy/list/auth/characters/blueprints",
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_corporation_assets(
        &self,
    ) -> Result<Vec<Asset>> {
        self
            .fetch_auth(
                "proxy/list/auth/corporations/assets",
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_corporation_blueprints(
        &self,
    ) -> Result<Vec<EveBlueprintResponse>> {
        self
            .fetch_auth(
                "proxy/list/auth/corporations/blueprints",
                &(),
            )
            .await
            .map_err(Into::into)
    }
}
