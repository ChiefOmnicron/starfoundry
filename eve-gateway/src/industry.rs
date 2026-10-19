mod error;
mod fetch_blueprint_dependencies_bulk;
mod fetch_blueprint_json;
mod fetch_system_index;
mod list_system_index;
mod models;

use starfoundry_gateway::ApiClient;
use starfoundry_lib_types::{SystemId, TypeId};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub use self::models::{IndustryActivity, IndustryJob};
use crate::client::ApiClientExtended;
use crate::industry::fetch_blueprint_dependencies_bulk::BlueprintDependency;
use crate::industry::fetch_blueprint_json::BlueprintJson;
use crate::industry::models::{
    IndustrySystem, SystemIndex, SystemIndexHistory
};
use crate::state::AppState;
use crate::{Result, TimeSpanQuery};

/// Exposes all routes that are under `/industry`
pub fn routes() -> OpenApiRouter<AppState> {
    let fetch_blueprint_dependencies = OpenApiRouter::new()
        .routes(routes!(self::fetch_blueprint_dependencies_bulk::api));

    let fetch_blueprint_json =
        OpenApiRouter::new().routes(routes!(self::fetch_blueprint_json::api));

    let fetch_system_index =
        OpenApiRouter::new().routes(routes!(self::fetch_system_index::api));
    let list_system_index =
        OpenApiRouter::new().routes(routes!(self::list_system_index::api));

    OpenApiRouter::new()
        .merge(fetch_blueprint_dependencies)
        .merge(fetch_blueprint_json)
        .merge(fetch_system_index)
        .merge(list_system_index)
}

pub trait EveGatewayApiClientIndustry: ApiClient + ApiClientExtended {
    #[allow(async_fn_in_trait)]
    async fn fetch_blueprint_dependencies_bulk(
        &self,
        type_ids: Vec<TypeId>,
    ) -> Result<Vec<BlueprintDependency>> {
        self.post("industry/blueprints/dependencies/bulk", type_ids)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_blueprint_json(
        &self,
        type_id: TypeId,
    ) -> Result<Option<BlueprintJson>> {
        self.fetch(&format!("industry/blueprints/{}/json", type_id), &())
            .await
            .map_err(Into::into)
    }

    /// Lists all active industry jobs from the character from the EVE-API
    #[allow(async_fn_in_trait)]
    async fn list_character_jobs(&self) -> Result<Vec<IndustryJob>> {
        self.fetch_page("proxy/auth/characters/industry/jobs")
            .await
    }

    /// Lists all active industry jobs from the character from the EVE-API
    #[allow(async_fn_in_trait)]
    async fn list_corporation_jobs(&self) -> Result<Vec<IndustryJob>> {
        self.fetch_page("proxy/auth/corporations/industry/jobs")
            .await
    }

    /// Fetches a specific index for a system.
    /// The values are cached and might not be the latest ones.
    #[allow(async_fn_in_trait)]
    async fn fetch_system_index(
        &self,
        system_id: SystemId,
    ) -> Result<Option<SystemIndex>> {
        self.fetch(&format!("industry/system-index/{}", system_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_system_index_history(
        &self,
        system_id: SystemId,
        time_span: TimeSpanQuery,
    ) -> Result<Vec<SystemIndexHistory>> {
        self.fetch(
            &format!("industry/system-index/{}/history", system_id),
            &time_span,
        )
        .await
        .map_err(Into::into)
    }

    /// Lists all industry systems from the EVE-API
    #[allow(async_fn_in_trait)]
    async fn list_system_index(&self) -> Result<Vec<IndustrySystem>> {
        self.fetch("proxy/industry/systems", &())
            .await
            .map_err(Into::into)
    }
}
