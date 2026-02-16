use starfoundry_lib_gateway::ApiClient;
use starfoundry_lib_types::{CharacterId, StructureId, SystemId, TypeId};

use crate::{CharacterInfo, EveGatewayApiClientEveAsset, EveGatewayApiClientIndustry, EveGatewayApiClientItem, ResolveStructureResponse, StructureRigBlueprintBonus, StructureRigResponse, StructureServiceResponse, System};
use crate::contract::EveGatewayApiClientContract;
use crate::error::Result;
use crate::eve_industry::EveGatewayApiClientEveIndustry;
use crate::eve_market::EveGatewayApiClientEveMarket;

pub trait EveGatewayApiClient:
    ApiClient +
    EveGatewayApiClientContract +
    EveGatewayApiClientEveAsset +
    EveGatewayApiClientEveIndustry +
    EveGatewayApiClientEveMarket +
    EveGatewayApiClientIndustry +
    EveGatewayApiClientItem {

    #[allow(async_fn_in_trait)]
    async fn fetch_character(
        &self,
        character_id: CharacterId,
    ) -> Result<CharacterInfo> {
        self
            .fetch(&format!("characters/{}", *character_id), &())
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
    ) -> Result<ResolveStructureResponse> {
        self
            .fetch(&format!("structures/{}", *structure_id), &())
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

    #[allow(async_fn_in_trait)]
    async fn fetch_system(
        &self,
        system_id: SystemId,
    ) -> Result<Option<System>> {
        self
            .fetch(&format!("universe/systems/{}", *system_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_system_bulk(
        &self,
        system_ids: Vec<SystemId>,
    ) -> Result<Vec<System>> {
        self
            .post("universe/systems", system_ids)
            .await
            .map_err(Into::into)
    }
}
