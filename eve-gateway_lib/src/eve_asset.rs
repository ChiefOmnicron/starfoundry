mod asset;
mod blueprint;
mod location_flag;
mod resolved_item;

pub use self::asset::*;
pub use self::blueprint::*;
pub use self::location_flag::*;
pub use self::resolved_item::*;

use starfoundry_lib_gateway::ApiClient;
use starfoundry_lib_types::CharacterId;
use starfoundry_lib_types::CorporationId;
use starfoundry_lib_types::ItemId;

use crate::Result;

pub trait EveGatewayApiClientEveAsset: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn eve_resolve_character_asset(
        &self,
        character_id:   CharacterId,
        assets:         Vec<ItemId>,
    ) -> Result<Vec<ResolvedItem>> {
        self
            .post_auth(
                &format!(
                    "eve/characters/{}/assets",
                    character_id,
                ),
                assets,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn eve_resolve_corporation_asset(
        &self,
        corporation_id: CorporationId,
        assets:         Vec<ItemId>,
    ) -> Result<Vec<ResolvedItem>> {
        self
            .post_auth(
                &format!(
                    "eve/corporations/{}/assets",
                    corporation_id,
                ),
                assets,
            )
            .await
            .map_err(Into::into)
    }
}
