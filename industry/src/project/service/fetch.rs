use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_industry::ProjectUuid;
use starfoundry_lib_industry::SolutionUuid;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;

use crate::project::error::ProjectError;
use crate::project::error::Result;
use crate::project::service::{ProjectExcess, ProjectJobFilter, ProjectStock, list_stock};
use crate::project::service::ProjectJobGroup;
use crate::project::service::ProjectProduct;
use crate::project::service::list_excess;
use crate::project::service::list_jobs;
use crate::project::service::list_products;
use crate::project::service::ProjectStatus;
use crate::project_group::service::ProjectGroup;

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
                project_id,
                eve_gateway_api_client
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
                project_id,
                eve_gateway_api_client
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

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "b034c3a9-2f4d-487d-95bb-c66fc20148b3",
        "name": "My cool project",
        "status": "IN_PROGRESS",
        "orderer": "Me Myself and I",
        "sell_price": 1337
    })
)]
pub struct Project {
    pub id:             ProjectUuid,
    pub name:           String,
    pub status:         ProjectStatus,
    pub orderer:        String,
    pub project_group:  ProjectGroup,
    pub products:       Vec<ProjectProduct>,
    pub stock:          Vec<ProjectStock>,
    pub excess:         Vec<ProjectExcess>,

    pub note:           Option<String>,
    pub sell_price:     Option<f64>,
    #[serde(skip)]
    pub solution_id:    Option<SolutionUuid>,
}

impl Project {
    pub async fn excess(
        &self,
        pool:                   &PgPool,
        eve_gateway_api_client: &impl EveGatewayApiClient,
    ) -> Result<Vec<ProjectExcess>> {
        list_excess(
                pool,
                self.id,
                eve_gateway_api_client,
            )
            .await
    }

    pub async fn products(
        &self,
        pool:                   &PgPool,
        eve_gateway_api_client: &impl EveGatewayApiClient,
    ) -> Result<Vec<ProjectProduct>> {
        list_products(
                pool,
                self.id,
                eve_gateway_api_client,
            )
            .await
    }

    pub async fn jobs(
        &self,
        pool:                   &PgPool,
        character_id:           CharacterId,
        eve_gateway_api_client: &impl EveGatewayApiClient,
    ) -> Result<Vec<ProjectJobGroup>> {
        list_jobs(
                pool,
                character_id,
                self.id,
                eve_gateway_api_client,
                ProjectJobFilter::default(),
            )
            .await
    }
}
