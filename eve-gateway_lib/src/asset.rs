mod blueprints;

pub use self::blueprints::*;

use starfoundry_lib_gateway::ApiClient;

use crate::Result;

pub trait EveGatewayApiClientAsset: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn fetch_blueprints(
        &self
    ) -> Result<Vec<Blueprint>> {
        self
            .fetch_auth(
                "/assets/blueprints",
                &(),
            )
            .await
            .map_err(Into::into)
    }
}
