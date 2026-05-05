use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_industry::project::{Project, ProjectStatus};
use starfoundry_lib_industry::ProjectUuid;
use starfoundry_lib_types::CharacterId;

use crate::project::error::ProjectError;
use crate::project::error::Result;
use crate::project::service::{list_excess, list_products, list_stock};

pub async fn fetch(
    pool:                   &PgPool,
    character_id:           CharacterId,
    project_id:             ProjectUuid,
    eve_gateway_api_client: &impl EveGatewayApiClient,
) -> Result<Option<Project>> {
    let entry = sqlx::query!(r#"
            SELECT
                id,
                name,
                status AS "status: ProjectStatus",
                orderer,
                sell_price,
                project_group_id,
                solution_id,
                note
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
        .map_err(|e| ProjectError::Fetch(e, project_id))?;

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

        let products = list_products(
                pool,
                eve_gateway_api_client,
                project_id,
            )
            .await?;

        let stock = list_stock(
                pool,
                project_id,
                eve_gateway_api_client
            )
            .await?;

        let excess = list_excess(
                pool,
                eve_gateway_api_client,
                project_id,
            )
            .await?;

        let project = Project {
            id:             x.id.into(),
            name:           x.name,
            status:         x.status,
            orderer:        x.orderer,
            sell_price:     x.sell_price,
            products:       products,
            stock:          stock,
            excess:         excess,

            note:           x.note,
            project_group:  project_group,
            solution_id:    x.solution_id.map(Into::into),
        };
        Ok(Some(project))
    } else {
        Ok(None)
    }
}
