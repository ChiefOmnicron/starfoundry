mod all;
mod error;

use starfoundry_gateway::ApiClient;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;
use crate::error::Result;
use crate::price::all::PriceResponse;

pub fn routes() -> OpenApiRouter<AppState> {
    let all = OpenApiRouter::new().routes(routes!(all::api));

    OpenApiRouter::new().merge(all)
}

pub trait MarketApiClientPrice: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn all_prices(&self) -> Result<Vec<PriceResponse>> {
        self.fetch("prices", &())
            .await
            .map_err(Into::into)
    }
}
