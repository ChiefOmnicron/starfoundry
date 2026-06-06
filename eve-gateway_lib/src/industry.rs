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

    #[allow(async_fn_in_trait)]
    async fn fetch_corporation_jobs(
        &self,
    ) -> Result<Vec<IndustryJob>> {
        self
            .fetch_auth(
                &format!("eve/industry/jobs/corporation"),
                &(),
            )
            .await
            .map_err(Into::into)
    }

}
