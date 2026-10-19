use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::project::error::{ProjectError, Result};
use crate::{AppState, ProjectGroupUuid, ProjectUuid, TagUuid};

/// Create Project
///
/// - Alternative route: `/latest/projects`
/// - Alternative route: `/v1/projects`
///
/// ---
///
/// Creates a new project
///
/// ## Security
/// - authenticated
/// - project_group:read
#[utoipa::path(
    post,
    path = "/",
    tag = "projects",
    request_body = CreateProject,
    responses(
        (
            body = CreateProjectResponse,
            description = "List all projects that match the given filters",
            status = OK,
        ),
        (
            description = "There aren't any projects matching the filter",
            status = NO_CONTENT,
        ),
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    identity: ExtractIdentity,
    State(state): State<AppState>,
    Json(project_info): Json<CreateProject>,
) -> Result<impl IntoResponse> {
    let id =
        create(&state.postgres, identity.character_id, project_info).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateProjectResponse {
            id,
        }),
    ))
}

pub async fn create(
    pool: &PgPool,
    character_id: CharacterId,
    project_info: CreateProject,
) -> Result<ProjectUuid> {
    project_info.validate()?;

    let project_id: ProjectUuid = sqlx::query!(
        r#"
            INSERT INTO project
            (
                owner,
                sell_price,
                project_group_id,
                orderer,
                name,
                note,
                pre_products,
                pre_additional
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
        "#,
        *character_id,
        project_info.sell_price,
        *project_info.project_group_id,
        project_info.orderer,
        project_info.name,
        project_info.notes,
        project_info.pre_products,
        project_info.pre_additional,
    )
    .fetch_one(pool)
    .await
    .map_err(ProjectError::Create)?
    .id
    .into();

    let tags = project_info
        .tags
        .map(|tags| {
            tags.into_iter()
                .map(|x| *x)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    sqlx::query!(
        "
            INSERT INTO project_tag
            (
                project_id,
                tag_id
            )
            SELECT $1, * FROM UNNEST(
                $2::UUID[]
            )
        ",
        *project_id,
        &tags,
    )
    .execute(pool)
    .await
    .map_err(ProjectError::Update)?;

    Ok(project_id)
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateProject {
    pub project_group_id: ProjectGroupUuid,
    pub orderer:          String,
    pub name:             String,

    pub sell_price: Option<f64>,
    pub notes:      Option<String>,

    pub pre_products:   Option<String>,
    pub pre_additional: Option<String>,

    pub tags: Option<Vec<TagUuid>>,
}

impl CreateProject {
    pub fn validate(&self) -> Result<bool> {
        if self.name.len() <= 100 {
            if self
                .name
                .trim()
                .is_empty()
            {
                return Err(ProjectError::ValidationError(
                    "Field 'name' must be set".into(),
                ));
            }
        } else {
            return Err(ProjectError::ValidationError(
                "Field 'name' is too long, max length: 100".into(),
            ));
        };

        Ok(true)
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "fd324c9f-ecda-49c8-948e-18f4b4b23bff"
    })
)]
pub struct CreateProjectResponse {
    pub id: ProjectUuid,
}

impl Default for CreateProjectResponse {
    fn default() -> Self {
        Self {
            id: Uuid::default().into(),
        }
    }
}
