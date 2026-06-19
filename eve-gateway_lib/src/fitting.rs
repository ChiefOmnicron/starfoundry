mod fit;

pub use self::fit::*;

use starfoundry_lib_gateway::{ApiClient, Identity};
use starfoundry_lib_types::FittingId;

use crate::Result;

pub trait EveGatewayApiClientFitting: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn create_fit(
        &self,
        identity:   Identity,
        data:       EveFit,
    ) -> Result<EveFitResponse> {
        self
            .post_auth(
                &format!("characters/{}/fittings", identity.character_id()),
                data,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn delete_fit(
        &self,
        identity:   Identity,
        fitting_id: FittingId,
    ) -> Result<EveFitResponse> {
        self
            .delete_auth(
                &format!(
                    "characters/{}/fittings/{}",
                    identity.character_id(),
                    fitting_id,
                ),
            )
            .await
            .map_err(Into::into)
    }
}
