use std::collections::HashMap;

use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_eve_gateway::asset::Item;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;
use utoipa::{IntoParams, ToSchema};

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::project::error::{ProjectError, Result};
use crate::project::{ProjectJobStatus, ProjectStatus};
use crate::project_group::list::{ProjectGroupFilter, ProjectGroupMinimal};
use crate::structure::Structure;
use crate::{AppState, ProjectGroupUuid, ProjectJobUuid, ProjectUuid, TagUuid};

/// List Projects
///
/// - Alternative route: `/latest/projects`
/// - Alternative route: `/v1/projects`
///
/// ---
///
/// Lists all projects the user has access to.
///
/// ## Security
/// - authenticated
/// - project_group:read
#[utoipa::path(
    get,
    path = "/",
    tag = "projects",
    params(ProjectFilter),
    responses(
        (
            body = Vec<ProjectMinimal>,
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
    Query(filter): Query<ProjectFilter>,
) -> Result<impl IntoResponse> {
    let data = list(&state.postgres, identity.character_id, filter).await?;

    if data.is_empty() {
        Ok((StatusCode::NO_CONTENT, Json(data)).into_response())
    } else {
        Ok((StatusCode::OK, Json(data)).into_response())
    }
}

pub async fn list(
    pool: &PgPool,
    character_id: CharacterId,
    filter: ProjectFilter,
) -> Result<Vec<ProjectMinimal>> {
    let user_project_groups = crate::project_group::list::list(
        pool,
        character_id,
        Default::default(),
    )
    .await?
    .into_iter()
    .map(|x| *x.id)
    .collect::<Vec<_>>();

    let filter_status: Vec<String> = if let Some(x) = filter
        .status
        .clone()
    {
        x.split(",")
            .map(|x| x.into())
            .collect::<Vec<_>>()
    } else {
        vec![
            "DRAFT".into(),
            "READY_TO_START".into(),
            "IN_PROGRESS".into(),
            "PAUSED".into(),
            "DONE".into(),
        ]
    };

    let tags = if let Some(x) = filter.tags {
        x
    } else {
        Vec::new()
    };

    let limit = if filter_status.is_empty()
        || (filter_status.len() == 1 && filter_status.contains(&"DONE".into()))
    {
        Some(25i64)
    } else {
        filter.limit
    };

    let entries = sqlx::query!(r#"
            SELECT
                id,
                name,
                status AS "status: ProjectStatus",
                orderer,
                sell_price,
                project_group_id
            FROM project p
            WHERE
                (
                    NOT (LOWER(name) LIKE '%' || LOWER($2) || '%') IS FALSE AND
                    NOT (status = ANY($3::PROJECT_STATUS[])) IS FALSE AND
                    NOT (LOWER(orderer) LIKE '%' || LOWER($4) || '%') IS FALSE AND
                    NOT (project_group_id = $5::UUID) IS FALSE AND
                    NOT (
                        ARRAY_LENGTH($7::UUID[], 1) = 0 OR
                        EXISTS(
                            SELECT 1
                            FROM project_tag
                            WHERE project_id = p.id
                            AND tag_id = ANY($7::UUID[])
                        )
                    ) IS FALSE
                )
                AND
                (
                    -- check if the character is in the group
                    (
                        NOT (project_group_id = ANY($6::UUID[])) IS FALSE
                    )
                    OR
                    -- as a fallback check if the character is the owner
                    (
                        owner = $1
                    )
                )
            ORDER BY name
            LIMIT $8
        "#,
            *character_id,
            filter.name,
            &filter_status as _,
            filter.orderer,
            filter.project_group_id.map(|x| *x),
            &user_project_groups,
            &tags.into_iter().map(|x| *x).collect::<Vec<_>>(),
            limit,
        )
        .fetch_all(pool)
        .await
        .map_err(ProjectError::List)?;

    let project_groups = crate::project_group::list::list(
        pool,
        character_id,
        ProjectGroupFilter::default(),
    )
    .await?
    .into_iter()
    .map(|x| (x.id, x))
    .collect::<HashMap<_, _>>();

    let mut projects = Vec::new();
    for entry in entries {
        let project_group = if let Some(x) = project_groups.get(
            &entry
                .project_group_id
                .into(),
        ) {
            x.clone()
        } else {
            continue;
        };

        let project_group = ProjectMinimal {
            id:            entry.id.into(),
            name:          entry.name,
            status:        entry.status,
            orderer:       entry.orderer,
            sell_price:    entry.sell_price,
            project_group: project_group,
        };
        projects.push(project_group);
    }

    Ok(projects)
}

#[derive(Debug, Default, Deserialize, Serialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ProjectFilter {
    #[serde(default)]
    #[param(
        example = json!("Project 1337"),
        required = false,
    )]
    pub name: Option<String>,

    #[param(
        default = json!("DRAFT,READY_TO_START,IN_PROGRESS,PAUSED,DONE"),
        required = false,
    )]
    #[serde(default = "default_status")]
    pub status: Option<String>,

    #[serde(default)]
    #[param(
        example = json!("019b5d76-0ebd-77f4-80b0-12daf86501b6"),
        required = false,
    )]
    pub project_group_id: Option<ProjectGroupUuid>,

    #[serde(default)]
    #[param(
        example = json!("Eistonen Kodan Sasen"),
        required = false,
    )]
    pub orderer: Option<String>,

    #[serde(default)]
    #[param(
        example = json!(10),
        required = false,
    )]
    pub limit: Option<i64>,

    #[serde(default)]
    #[param(
        example = json!(["01a0e02c-a5cf-78ca-8b8e-dcd27b5f3ced"]),
        required = false,
    )]
    pub tags: Option<Vec<TagUuid>>,
}

fn default_status() -> Option<String> {
    Some("DRAFT,READY_TO_START,IN_PROGRESS,PAUSED,DONE".into())
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
pub struct ProjectMinimal {
    pub id:            ProjectUuid,
    pub name:          String,
    pub status:        ProjectStatus,
    pub orderer:       String,
    pub project_group: ProjectGroupMinimal,

    pub sell_price: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectJob {
    pub id:         ProjectJobUuid,
    pub project_id: ProjectUuid,
    pub job_id:     Option<i32>,
    pub status:     ProjectJobStatus,

    pub runs: i32,
    pub cost: Option<f64>,

    pub item:       Item,
    pub structure:  Structure,
    pub started_by: Option<CharacterId>,

    pub end_date: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectJobGroup {
    pub header:  String,
    pub entries: Vec<ProjectJob>,
}

#[derive(Debug, Default, Deserialize, Serialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ProjectJobFilter {
    #[serde(default)]
    pub startable: Option<bool>,
}

#[derive(Debug, Default, Deserialize, Serialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ProjectAllJobFilter {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub tags: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectJobAllGroup {
    pub header:     String,
    pub project_id: ProjectUuid,
    pub entries:    Vec<ProjectJob>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectExcess {
    pub item:     Item,
    pub quantity: i32,
    pub cost:     Option<f64>,
}

#[cfg(test)]
mod list_project_group_test {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::StatusCode;
    use axum::http::header::HOST;
    use http_body_util::BodyExt;
    use sqlx::PgPool;
    use starfoundry_gateway::{
        HEADER_CHARACTER_ID, HEADER_CORPORATION_ID, HEADER_SERVICE
    };
    use starfoundry_lib_types::CharacterId;

    use crate::project::list::{ProjectFilter, ProjectMinimal};
    use crate::project::project_test_routes;

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base")))]
    async fn directly_happy_path(pool: PgPool) {
        let result =
            super::list(&pool, CharacterId(1), ProjectFilter::default()).await;
        assert!(result.is_ok());
        assert_eq!(
            result
                .unwrap()
                .len(),
            4
        );

        let result = super::list(
            &pool,
            CharacterId(1),
            ProjectFilter {
                name: Some(String::from("Filter")),
                ..Default::default()
            },
        )
        .await;
        assert!(result.is_ok());
        assert_eq!(
            result
                .unwrap()
                .len(),
            1
        );

        let result = super::list(
            &pool,
            CharacterId(1),
            ProjectFilter {
                name: Some(String::from("SomeGibberish")),
                ..Default::default()
            },
        )
        .await;
        assert!(result.is_ok());
        assert_eq!(
            result
                .unwrap()
                .len(),
            0
        );
    }

    #[sqlx::test(fixtures("base"))]
    async fn happy_path_all(pool: PgPool) {
        let request = Request::builder()
            .uri("/")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();

        let response = project_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<ProjectMinimal> = serde_json::from_slice(
            &response
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes(),
        )
        .unwrap();
        assert_eq!(body.len(), 4);
    }

    #[sqlx::test(fixtures("base"))]
    async fn happy_path_filter(pool: PgPool) {
        // Filter
        let request = Request::builder()
            .uri("/?name=Filter")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();

        let response = project_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<ProjectMinimal> = serde_json::from_slice(
            &response
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes(),
        )
        .unwrap();
        assert_eq!(body.len(), 1);
    }

    #[sqlx::test(fixtures("base"))]
    async fn happy_path_empty(pool: PgPool) {
        // Empty
        let request = Request::builder()
            .uri("/?name=SomeGibberish")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();

        let response = project_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let body: Vec<ProjectMinimal> = serde_json::from_slice(
            &response
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes(),
        )
        .unwrap();
        assert_eq!(body.len(), 0);
    }

    #[sqlx::test(fixtures("base"))]
    async fn unauthorized(pool: PgPool) {
        let request = Request::builder()
            .uri("/")
            .method("GET")
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();

        let response = project_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
