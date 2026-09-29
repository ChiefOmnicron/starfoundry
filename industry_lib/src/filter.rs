use crate::Result;

use serde::{Deserialize, Serialize};
use starfoundry_lib_gateway::ApiClient;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Filter {
    pub filter_name:    String,
    pub entries:        Vec<serde_json::Value>,
}

pub trait IndustryApiClientFilter: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn list(
        &self,
    ) -> Result<Vec<Filter>> {
        self
            .fetch(
                "filters",
                &(),
            )
            .await
            .map_err(Into::into)
    }
}
