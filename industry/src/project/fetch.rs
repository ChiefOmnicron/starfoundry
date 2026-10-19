use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_eve_gateway::EveGatewayApiClient;
use starfoundry_eve_gateway::asset::Item;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;
use starfoundry_market::market::MarketBulkResponse;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::project::ProjectStatus;
use crate::project::error::{ProjectError, Result};
use crate::project::list::{ProjectExcess, ProjectMinimal};
use crate::project::list_excess::list_excess;
use crate::project::list_parent::list_parent_projects;
use crate::project::list_product::list_products;
use crate::project::list_stock::list_stock;
use crate::project::list_sub::list_sub_projects;
use crate::project::list_tags::list_tags;
use crate::project_group::fetch::ProjectGroup;
use crate::tag::fetch::Tag;
use crate::{AppState, ProjectUuid, SolutionUuid, eve_gateway_api_client};

/// Fetch Tag
///
/// - Alternative route: `/latest/projects/{ProjectUuid}`
/// - Alternative route: `/v1/projects/{ProjectUuid}`
///
/// ---
///
/// Fetches information about a project
///
/// ## Security
/// - authenticated
/// - structure:read
#[utoipa::path(
    get,
    path = "/{ProjectUuid}",
    tag = "Projects",
    params(
        ProjectUuid,
    ),
    responses(
        (
            body = Project,
            description = "Information about the structure",
            status = OK,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    identity: ExtractIdentity,
    State(state): State<AppState>,
    Path(project_id): Path<ProjectUuid>,
) -> Result<impl IntoResponse> {
    let entry = fetch(
        &state.postgres,
        identity.character_id,
        project_id,
        &eve_gateway_api_client()?,
    )
    .await?;

    if let Some(x) = entry {
        Ok((StatusCode::OK, Json(x)).into_response())
    } else {
        Ok((StatusCode::NO_CONTENT, Json(())).into_response())
    }
}

pub async fn fetch(
    pool: &PgPool,
    character_id: CharacterId,
    project_id: ProjectUuid,
    eve_gateway_api_client: &impl EveGatewayApiClient,
) -> Result<Option<Project>> {
    let entry = sqlx::query!(
        r#"
            SELECT
                id,
                name,
                status AS "status: ProjectStatus",
                orderer,
                sell_price,
                project_group_id,
                solution_id,
                note,
                pre_products,
                pre_additional
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
        let project_group = if let Ok(Some(x)) =
            crate::project_group::fetch::fetch(
                pool,
                eve_gateway_api_client,
                character_id,
                x.project_group_id
                    .into(),
            )
            .await
        {
            x
        } else {
            return Ok(None);
        };

        let products =
            list_products(pool, eve_gateway_api_client, project_id).await?;

        let stock =
            list_stock(pool, project_id, eve_gateway_api_client).await?;

        let excess =
            list_excess(pool, eve_gateway_api_client, project_id).await?;

        let tags = list_tags(pool, project_id).await?;

        let sub_projects =
            list_sub_projects(pool, character_id, project_id).await?;

        let parent_projects =
            list_parent_projects(pool, character_id, project_id).await?;

        let project = Project {
            id:         x.id.into(),
            name:       x.name,
            status:     x.status,
            orderer:    x.orderer,
            sell_price: x.sell_price,
            products:   products,
            stock:      stock,
            excess:     excess,
            tags:       tags,

            note:            x.note,
            project_group:   project_group,
            sub_projects:    sub_projects,
            parent_projects: parent_projects,

            solution_id: x
                .solution_id
                .map(Into::into),

            pre_products:   x.pre_products,
            pre_additional: x.pre_additional,
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
    pub id:            ProjectUuid,
    pub name:          String,
    pub status:        ProjectStatus,
    pub orderer:       String,
    pub project_group: ProjectGroup,
    pub products:      Vec<ProjectProduct>,
    pub stock:         Vec<ProjectStock>,
    pub excess:        Vec<ProjectExcess>,
    pub tags:          Vec<Tag>,

    pub note:            Option<String>,
    pub sell_price:      Option<f64>,
    pub sub_projects:    Option<Vec<ProjectMinimal>>,
    pub parent_projects: Option<Vec<ProjectMinimal>>,

    #[serde(skip)]
    pub solution_id: Option<SolutionUuid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_products:   Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_additional: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectProduct {
    pub item:                Item,
    pub quantity:            i32,
    pub material_efficiency: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectCost {
    pub sell_price: f64,

    pub job_cost:    f64,
    pub market_cost: f64,
    pub misc_cost:   f64,
    pub excess_cost: f64,
    pub stock_cost:  f64,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectTimeLeft {
    /// # Possible values:
    ///
    /// - CREATED
    /// - RAW_MATERIALS
    /// - INTERMEDIATE_REACTIONS
    /// - COMPOSITE_REACTIONS
    /// - BIOCHEM_REACTIONS
    /// - HYBRID_REACTIONS
    /// - CONSTRUCTION_COMPONENTS
    /// - ADVANCED_CAPITAL_CONSTRUCTION_COMPONENTS
    /// - CAPITAL_CONSTRUCTION_COMPONENTS
    /// - TOOLS
    /// - T1_STUFF
    /// - T2_STUFF
    /// - CHARGES
    /// - SHIPS
    /// - UNKNOWN
    /// - DONE
    pub state:   String,
    pub date_ms: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectMarket {
    pub id:       Uuid,
    pub item:     Item,
    pub quantity: i32,

    pub cost:   Option<f64>,
    pub source: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectMarketBuy {
    pub id:       Uuid,
    pub item:     Item,
    pub quantity: i32,

    pub cost:   Option<f64>,
    pub source: Option<String>,

    pub entries: Vec<MarketBulkResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectMisc {
    pub id:   Uuid,
    pub item: String,
    pub cost: f64,

    pub description: Option<String>,
    pub quantity:    Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectStock {
    pub item:     Item,
    pub quantity: i32,
    pub cost:     Option<f64>,
}
