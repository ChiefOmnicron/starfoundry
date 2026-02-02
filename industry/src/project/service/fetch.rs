use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_types::CharacterId;

use crate::project::error::ProjectError;
use crate::project::error::Result;
use crate::project::ProjectUuid;
use crate::project::service::{ProjectList, ProjectStatus};

pub async fn fetch(
    pool:                   &PgPool,
    character_id:           CharacterId,
    project_id:             ProjectUuid,
    eve_gateway_api_client: &impl EveGatewayApiClient,
) -> Result<Option<ProjectList>> {
    let entry = sqlx::query!(r#"
            SELECT
                id,
                name,
                status AS "status: ProjectStatus",
                orderer,
                sell_price,
                project_group_id
            FROM project
            WHERE
                (owner = $1 OR owner = 0) AND
                id = $2
        "#,
            *character_id,
            *project_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| ProjectError::FetchProject(e, project_id))?;

    if let Some(x) = entry {
        let project_group = if let Ok(Some(x)) = crate::project_group::service::fetch(
            pool,
            eve_gateway_api_client,
            character_id,
            x.project_group_id.into(),
        ).await {
            x
        } else {
            return Ok(None);
        };

        let project = ProjectList {
            id:            x.id.into(),
            name:          x.name,
            status:        x.status,
            orderer:       x.orderer,
            sell_price:    x.sell_price,
            project_group: project_group,
        };
        Ok(Some(project))
    } else {
        Ok(None)
    }
}
