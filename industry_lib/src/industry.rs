mod calculation;
mod stock;

use crate::Result;

pub use self::calculation::*;
pub use self::stock::*;

use starfoundry_lib_gateway::ApiClient;

pub trait IndustryApiClientIndustry: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn calculation(
        &self,
        request: &BuildEngine,
    ) -> Result<Vec<BuildEngineResponse>> {
        self
            .post(
                "industry/calculation",
                request,
            )
            .await
            .map_err(Into::into)
    }
}
