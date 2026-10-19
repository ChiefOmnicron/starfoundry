mod fetch;
mod fetch_bulk;
mod fetch_distance;
mod list;
mod list_distances;
mod list_in_range;
mod models;

pub mod error;
use starfoundry_gateway::ApiClient;
use starfoundry_lib_types::SystemId;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub(crate) use self::fetch::fetch_system;
pub use self::models::*;
use crate::Result;
use crate::state::AppState;

/// Exposes all routes that are under `/systems`
pub fn routes() -> OpenApiRouter<AppState> {
    let fetch_system = OpenApiRouter::new().routes(routes!(fetch::api));
    let fetch_system_bulk =
        OpenApiRouter::new().routes(routes!(fetch_bulk::api));
    let list = OpenApiRouter::new().routes(routes!(list::api));

    let fetch_distance =
        OpenApiRouter::new().routes(routes!(fetch_distance::api));
    let list_distances =
        OpenApiRouter::new().routes(routes!(list_distances::api));
    let list_in_range =
        OpenApiRouter::new().routes(routes!(list_in_range::api));

    OpenApiRouter::new()
        .merge(fetch_system)
        .merge(fetch_system_bulk)
        .merge(list)
        .merge(fetch_distance)
        .merge(list_distances)
        .merge(list_in_range)
}

pub trait EveGatewayApiClientSystem: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn fetch_system(
        &self,
        system_id: SystemId,
    ) -> Result<Option<System>> {
        self.fetch(&format!("systems/{}", *system_id), &())
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

        self.post("systems", system_ids)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_systems(
        &self,
        filter: SystemSearchQuery,
    ) -> Result<Vec<System>> {
        self.fetch("systems", &filter)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_distance(
        &self,
        start_system_id: SystemId,
        end_system_id: SystemId,
    ) -> Result<Option<SystemDistance>> {
        self.fetch(
            &format!(
                "systems/{}/distances/{}",
                *start_system_id, *end_system_id
            ),
            &(),
        )
        .await
        .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_distances(&self) -> Result<Vec<SystemDistanceMinimal>> {
        self.fetch("systems/distances", &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_in_range(
        &self,
        system_id: SystemId,
    ) -> Result<Vec<SystemDistance>> {
        self.fetch(&format!("systems/{}/distances", system_id), &())
            .await
            .map_err(Into::into)
    }
}
