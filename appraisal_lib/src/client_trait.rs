use crate::client::AppraisalClient;
use starfoundry_lib_gateway::ApiClient;
use crate::error::Result;
use crate::{Appraisal, CreateAppraisalRequest};

impl AppraisalApiClient for AppraisalClient {}

/// Trait that should be implemented on all clients
/// The default implementation will be sufficient in most cases, overwriting
/// them is only recommended for mocking tests
pub trait AppraisalApiClient: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn create(
        &self,
        info: CreateAppraisalRequest,
    ) -> Result<Option<Appraisal>> {
        self
            .post(
                "appraisals",
                info,
            )
            .await
            .map_err(Into::into)
    }
}
