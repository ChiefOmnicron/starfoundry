use starfoundry_lib_eve_api::{CorporationInfo, EveApiClient};
use starfoundry_lib_types::CorporationId;
use tracing::instrument;

use crate::corporation::CorporationError;

#[instrument(err, level = "error")]
pub async fn info(
    client:         EveApiClient,
    corporation_id: CorporationId,
) -> Result<CorporationInfo, CorporationError> {
    client
        .corporation_info(corporation_id)
        .await
        .map_err(CorporationError::FetchInfo)
}
