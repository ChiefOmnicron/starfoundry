mod jump_plan;

use starfoundry_lib_gateway::ApiClient;

use crate::Result;

pub use self::jump_plan::*;

pub trait MappingApiClientRoute: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn create_jump_plan(
        &self,
        plan: CreateJumpPlan,
    ) -> Result<Vec<JumpPlanEntry>> {
        self
            .post("routes/jump-plans", plan)
            .await
            .map_err(Into::into)
    }
}
