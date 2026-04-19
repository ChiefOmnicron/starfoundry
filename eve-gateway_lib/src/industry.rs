mod blueprint_dependency;
mod blueprint_json;
mod industry_job;
mod system_index;

pub use self::blueprint_dependency::*;
pub use self::blueprint_json::*;
pub use self::industry_job::*;
pub use self::system_index::*;

use starfoundry_lib_gateway::{ApiClient, HEADER_CHARACTER_ID, HEADER_CORPORATION_ID};
use starfoundry_lib_types::{CharacterId, CorporationId, SystemId, TypeId};

use crate::Result;
use axum::http::{HeaderMap, HeaderValue};
use reqwest::header::HOST;

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
        source:         String,
        character_id:   CharacterId,
        corporation_id: CorporationId,
    ) -> Result<Vec<IndustryJob>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (*character_id).into());
        headers.insert(HEADER_CORPORATION_ID, (*corporation_id).into());

        self
            .fetch_auth(
                &format!("eve/industry/jobs/corporation"),
                &(),
                headers,
            )
            .await
            .map_err(Into::into)
    }

}
