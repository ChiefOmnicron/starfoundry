mod system;
mod system_distance;

use starfoundry_lib_gateway::ApiClient;
use starfoundry_lib_types::SystemId;

use crate::Result;

pub use self::system::*;
pub use self::system_distance::*;

pub trait EveGatewayApiClientSystem: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn fetch_system(
        &self,
        system_id: SystemId,
    ) -> Result<Option<System>> {
        self
            .fetch(&format!("systems/{}", *system_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_system_bulk(
        &self,
        system_ids: Vec<SystemId>,
    ) -> Result<Vec<System>> {
        let mut system_ids = system_ids;
        system_ids.sort();
        system_ids.dedup();

        self
            .post("systems", system_ids)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_systems(
        &self,
    ) -> Result<Vec<System>> {
        self
            .fetch("systems", &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_distance(
        &self,
        start_system_id:    SystemId,
        end_system_id:      SystemId,
    ) -> Result<Option<SystemDistance>> {
        self
            .fetch(&format!("systems/{}/distances/{}", *start_system_id, *end_system_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_distances(
        &self,
    ) -> Result<Vec<SystemDistanceMinimal>> {
        self
            .fetch("systems/distances", &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_in_range(
        &self,
        system_id: SystemId,
    ) -> Result<Vec<SystemDistance>> {
        self
            .fetch(&format!("systems/{}/distances", system_id), &())
            .await
            .map_err(Into::into)
    }
}
