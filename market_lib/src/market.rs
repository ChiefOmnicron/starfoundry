mod bulk;
mod gas_utils;
mod virtual_market;
use starfoundry_lib_gateway::ApiClient;

pub use self::bulk::*;
pub use self::gas_utils::*;
pub use self::virtual_market::*;

use crate::Result;

pub trait MarketApiClientOrder: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn bulk_latest_orders(
        &self,
        request: MarketBulkRequest,
    ) -> Result<Vec<MarketBulkResponse>> {
        self
            .post(
                "markets/bulk",
                request,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn update_virtual_market(
        &self,
        request: Vec<MarketVirtualRequest>,
    ) -> Result<()> {
        self
            .post(
                "markets/virtual",
                request,
            )
            .await
            .map_err(Into::into)
    }
}
