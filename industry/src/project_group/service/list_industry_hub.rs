use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_types::CharacterId;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;
use crate::industry_hub::service::{IndustryHub, fetch};

pub async fn list_industry_hubs(
    pool:                   &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    character_id:           CharacterId,
    project_group_uuid:     ProjectGroupUuid,
) -> Result<Vec<IndustryHub>> {
    let entries = sqlx::query!(
        "
            SELECT industry_hub_id
            FROM project_group_industry_hub pgm
            WHERE project_group_id = $1
        ",
            *project_group_uuid,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ProjectGroupError::ListIndustryHubs(e, project_group_uuid))?;

    let mut hubs = Vec::new();
    for entry in entries {
        let hub = fetch(
            pool,
            eve_gateway_api_client,
            character_id,
            entry.industry_hub_id.into(),
        ).await?;

        if let Some(x) = hub {
            hubs.push(x);
        }
    }

    Ok(hubs)
}
