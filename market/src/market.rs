mod bulk;
mod error;
mod last_fetch;
mod virtual_market;

use starfoundry_gateway::ApiClient;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub(crate) use self::bulk::MarketEntry;
pub use self::bulk::{
    MarketBulkRequest, MarketBulkResponse, MarketItem, MarketStrategy, SmartBuyConfig
};
pub use self::virtual_market::MarketVirtualRequest;
use crate::AppState;
use crate::error::Result;

pub fn routes() -> OpenApiRouter<AppState> {
    let bulk = OpenApiRouter::new().routes(routes!(bulk::api));

    let last_fetch = OpenApiRouter::new().routes(routes!(last_fetch::api));

    let virtual_market =
        OpenApiRouter::new().routes(routes!(virtual_market::api));

    OpenApiRouter::new()
        .merge(bulk)
        .merge(last_fetch)
        .merge(virtual_market)
}

pub trait MarketApiClientOrder: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn bulk_latest_orders(
        &self,
        request: MarketBulkRequest,
    ) -> Result<Vec<MarketBulkResponse>> {
        self.post("markets/bulk", request)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn update_virtual_market(
        &self,
        request: Vec<MarketVirtualRequest>,
    ) -> Result<()> {
        self.post("markets/virtual", request)
            .await
            .map_err(Into::into)
    }
}
