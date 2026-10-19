mod error;
mod jump_plan;

use starfoundry_gateway::ApiClient;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::error::Result;
use crate::routes::jump_plan::{CreateJumpPlan, JumpPlanEntry};
use crate::state::AppState;

/// Exposes all routes that are under `/routes`
pub fn routes() -> OpenApiRouter<AppState> {
    let jump_plan = OpenApiRouter::new().routes(routes!(jump_plan::api));

    OpenApiRouter::new().merge(jump_plan)
}

pub trait MappingApiClientRoute: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn create_jump_plan(
        &self,
        plan: CreateJumpPlan,
    ) -> Result<Vec<JumpPlanEntry>> {
        self.post("routes/jump-plans", plan)
            .await
            .map_err(Into::into)
    }
}
