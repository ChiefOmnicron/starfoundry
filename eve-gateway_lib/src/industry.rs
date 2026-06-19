mod blueprint_dependency;
mod blueprint_json;
mod industry_job;
mod system_index;

pub use self::blueprint_dependency::*;
pub use self::blueprint_json::*;
pub use self::industry_job::*;
pub use self::system_index::*;

use starfoundry_lib_gateway::ApiClient;
use starfoundry_lib_types::{SystemId, TypeId};

use crate::Result;

pub trait EveGatewayApiClientIndustry: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn fetch_blueprint_dependencies_bulk(
        &self,
        type_ids: Vec<TypeId>,
    ) -> Result<Vec<BlueprintDependency>> {
        self
            .post(
                "industry/blueprints/dependencies/bulk",
                type_ids,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_blueprint_json(
        &self,
        type_id: TypeId,
    ) -> Result<Option<BlueprintJson>> {
        self
            .fetch(&format!("industry/blueprints/{}/json", type_id), &())
            .await
            .map_err(Into::into)
    }

    /// Lists all active industry jobs from the character from the EVE-API
    /// 
    #[allow(async_fn_in_trait)]
    async fn list_character_jobs(
        &self,
    ) -> Result<Vec<IndustryJob>> {
        self
            .fetch_auth(
                "proxy/list/auth/characters/industry/jobs",
                &(),
            )
            .await
            .map_err(Into::into)
    }

    /// Lists all active industry jobs from the character from the EVE-API
    /// 
    #[allow(async_fn_in_trait)]
    async fn list_corporation_jobs(
        &self,
    ) -> Result<Vec<IndustryJob>> {
        self
            .fetch_auth(
                "proxy/list/auth/corporations/industry/jobs",
                &(),
            )
            .await
            .map_err(Into::into)
    }

    /// Fetches a specific index for a system.
    /// The values are cached and might not be the latest ones.
    /// 
    #[allow(async_fn_in_trait)]
    async fn fetch_system_index(
        &self,
        system_id: SystemId,
    ) -> Result<Option<SystemIndex>> {
        self
            .fetch(&format!("industry/system-index/{}", system_id), &())
            .await
            .map_err(Into::into)
    }

    /// Lists all industry systems from the EVE-API
    /// 
    #[allow(async_fn_in_trait)]
    async fn list_system_index(
        &self,
    ) -> Result<Vec<IndustrySystem>> {
        self
            .fetch(
                "proxy/list/industry/systems",
                &(),
            )
            .await
            .map_err(Into::into)
    }
}
