mod create;
mod delete;
mod error;
mod list;

use serde::{Deserialize, Serialize};
use starfoundry_gateway::ApiClient;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::filter::create::{CreateFilter, CreateFilterResponse};
use crate::{AppState, FilterUuid, Result};

pub fn routes() -> OpenApiRouter<AppState> {
    let create = OpenApiRouter::new().routes(routes!(create::api));

    let delete = OpenApiRouter::new().routes(routes!(delete::api));

    let list = OpenApiRouter::new().routes(routes!(list::api));

    OpenApiRouter::new()
        .merge(create)
        .merge(delete)
        .merge(list)
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Filter {
    pub id:      FilterUuid,
    pub name:    String,
    pub filters: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FilterType {
    ProjectJob,
}

impl From<FilterType> for String {
    fn from(value: FilterType) -> Self {
        match value {
            FilterType::ProjectJob => "PROJECT_JOB",
        }
        .into()
    }
}

pub trait IndustryApiClientFilter: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn create(
        &self,
        filter: CreateFilter,
    ) -> Result<Vec<CreateFilterResponse>> {
        self.post("filters", filter)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list(&self) -> Result<Vec<Filter>> {
        self.fetch("filters", &())
            .await
            .map_err(Into::into)
    }
}
