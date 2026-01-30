use starfoundry_lib_gateway::ApiClient;

use crate::{PriceResponse, Result};

pub trait MarketApiClientPrice: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn all_prices(
        &self,
    ) -> Result<Vec<PriceResponse>> {
        self
            .fetch("prices", &())
            .await
            .map_err(Into::into)
    }
}
