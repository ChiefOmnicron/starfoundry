use serde::de::DeserializeOwned;
use starfoundry_lib_gateway::ApiClient;
use starfoundry_lib_types::{CharacterId, StructureId, TypeId};

use crate::{AuthedCharacterInfo, CharacterInfo, EveGatewayApiClientAsset, EveGatewayApiClientEveAsset, EveGatewayApiClientFitting, EveGatewayApiClientIndustry, EveGatewayApiClientItem, EveGatewayApiClientSearch, EveGatewayApiClientStanding, EveGatewayApiClientSystem, ResolveStructureResponse, StructureRigBlueprintBonus, StructureRigResponse, StructureServiceResponse};
use crate::contract::EveGatewayApiClientContract;
use crate::error::Result;
use crate::market::EveGatewayApiClientMarket;

pub trait EveGatewayApiClient:
    ApiClient +
    ApiClientExtended +
    EveGatewayApiClientAsset +
    EveGatewayApiClientContract +
    EveGatewayApiClientEveAsset +
    EveGatewayApiClientFitting +
    EveGatewayApiClientMarket +
    EveGatewayApiClientIndustry +
    EveGatewayApiClientItem +
    EveGatewayApiClientSearch +
    EveGatewayApiClientStanding +
    EveGatewayApiClientSystem {

    #[allow(async_fn_in_trait)]
    async fn list_characters(
        &self,
    ) -> Result<Option<AuthedCharacterInfo>> {
        self
            .fetch("characters", &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_character(
        &self,
        character_id: CharacterId,
    ) -> Result<Option<CharacterInfo>> {
        self
            .fetch(
                &format!("characters/{}", *character_id),
                &()
            )
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
    async fn resolve_structure(
        &self,
        structure_id: StructureId,
    ) -> Result<Option<ResolveStructureResponse>> {
        self
            .fetch(
                &format!("structures/{}", *structure_id),
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_rig(
        &self,
        rig_type_id: TypeId,
    ) -> Result<Option<StructureRigResponse>> {
        self
            .fetch(&format!("structures/rigs/{}", *rig_type_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_service(
        &self,
        service_type_id: TypeId,
    ) -> Result<Option<StructureServiceResponse>> {
        self
            .fetch(&format!("structures/services/{}", *service_type_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_structure_rigs(
        &self,
        structure_type_id: TypeId,
    ) -> Result<Vec<StructureRigResponse>> {
        self
            .fetch(&format!("structures/{}/rigs", *structure_type_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_structure_services(
        &self,
        structure_type_id: TypeId,
    ) -> Result<StructureServiceResponse> {
        self
            .fetch(&format!("structures/{}/services", *structure_type_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_rig_blueprints(
        &self,
        rig_type_ids: Vec<TypeId>,
    ) -> Result<Vec<StructureRigBlueprintBonus>> {
        self
            .post("structures/rigs", rig_type_ids)
            .await
            .map_err(Into::into)
    }
}

pub trait ApiClientExtended: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn fetch_page<T>(
        &self,
        path: impl Into<String>,
    ) -> Result<Vec<T>>
    where
        T: DeserializeOwned + Send;
}
