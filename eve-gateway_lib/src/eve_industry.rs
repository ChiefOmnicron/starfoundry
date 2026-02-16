use starfoundry_lib_gateway::ApiClient;

use crate::{IndustrySystem, Result};

pub trait EveGatewayApiClientEveIndustry: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn eve_fetch_system_index(
        &self,
    ) -> Result<Vec<IndustrySystem>> {
        self
            .fetch(&format!("eve/industry/system-index"), &())
            .await
            .map_err(Into::into)
    }
}
