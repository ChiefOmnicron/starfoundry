use starfoundry_lib_gateway::ApiClient;
use starfoundry_lib_types::{CharacterId, StructureId, TypeId};

use crate::error::Result;
use crate::{CharacterInfo, Item, ResolveStructureResponse};

pub trait EveGatewayApiClient: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn fetch_character(
        &self,
        character_id: CharacterId,
    ) -> Result<CharacterInfo> {
        self
            .fetch(&format!("characters/{}", *character_id))
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_character_bulk(
        &self,
        character_ids: Vec<CharacterId>,
    ) -> Result<Vec<CharacterInfo>> {
        self
            .post("characters/bulk", character_ids)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_item(
        &self,
        type_id: TypeId,
    ) -> Result<Option<Item>> {
        self
            .fetch(&format!("items/{}", *type_id))
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_item_bulk(
        &self,
        type_ids: Vec<TypeId>,
    ) -> Result<Vec<Item>> {
        self
            .post("items/bulk", type_ids)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn resolve_structure(
        &self,
        structure_id: StructureId,
    ) -> Result<Vec<ResolveStructureResponse>> {
        self
            .fetch(&format!("universe/structures/{}", *structure_id))
            .await
            .map_err(Into::into)
    }
}
