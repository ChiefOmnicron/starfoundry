use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;
use utoipa::{IntoParams, ToSchema};

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::project_group::ProjectGroupError;
use crate::project_group::error::Result;
use crate::project_group::fetch::ProjectGroup;
use crate::{AppState, ProjectGroupUuid};

/// List Groups
///
/// - Alternative route: `/latest/project-groups`
/// - Alternative route: `/v1/project-groups`
///
/// ---
///
/// Lists all project groups the user has access to.
///
/// ## Security
/// - authenticated
/// - project_group:read
#[utoipa::path(
    get,
    path = "/",
    tag = "Project Groups",
    params(ProjectGroupFilter),
    responses(
        (
            body = Vec<ProjectGroup>,
            description = "List all groups that match the given filters",
            status = OK,
        ),
        (
            description = "There aren't any project groups created",
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
    State(state): State<AppState>,
    Query(filter): Query<ProjectGroupFilter>,
    identity: ExtractIdentity,
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
    filter: ProjectGroupFilter,
) -> Result<Vec<ProjectGroupMinimal>> {
    let owner_filter = match filter.owner {
        Some(true) => false,
        Some(false) | None => true,
    };
    let archived_filter = match filter.archived {
        Some(true) => true,
        Some(false) | None => false,
    };

    let entries = sqlx::query!(
        "
            SELECT
                pg.id,
                pg.name,
                pg.description,
                pg.owner = $1 AS is_owner,
                pg.archived,
                (
                    SELECT COUNT(*)
                    FROM project
                    WHERE project_group_id = pg.id
                ) AS projects
            FROM project_group pg
            JOIN project_group_member pgm ON pg.id = pgm.project_group_id
            -- fetch all projects where the user is a member
            WHERE pgm.character_id = $1 AND
                NOT (LOWER(name) LIKE '%' || LOWER($2) || '%') IS FALSE AND
                NOT (owner = $1 OR $3) IS FALSE AND
                archived = $4
            ORDER BY pg.name ASC
        ",
        *character_id,
        filter.name,
        owner_filter,
        archived_filter,
    )
    .fetch_all(pool)
    .await
    .map_err(ProjectGroupError::ListGroups)?;

    let mut project_groups = Vec::new();
    for entry in entries {
        let project_group = ProjectGroupMinimal {
            id:            entry.id.into(),
            name:          entry.name,
            project_count: entry
                .projects
                .unwrap_or(0),
            is_owner:      entry
                .is_owner
                .unwrap_or_default(),
            description:   entry.description,
            archived:      entry.archived,
        };
        project_groups.push(project_group);
    }

    Ok(project_groups)
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "b034c3a9-2f4d-487d-95bb-c66fc20148b3",
        "name": "My cool group",
        "project_count": 100,
        "is_owner": true,
        "description": "Bunch of cool projects",
        "members": [{
            "character_name": "SomeCharacterName",
            "character_id": 1337,

            "permission": [
                "READ",
                "WRITE_GROUP"
            ],
            "is_owner": false
        }]
    })
)]
pub struct ProjectGroupMinimal {
    pub id:            ProjectGroupUuid,
    pub name:          String,
    pub project_count: i64,
    pub is_owner:      bool,
    pub description:   Option<String>,
    pub archived:      bool,
}

#[derive(Debug, Default, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ProjectGroupFilter {
    #[serde(default)]
    #[param(
        example = json!("ProjectGroup1337"),
        required = false,
    )]
    pub name: Option<String>,

    #[serde(default)]
    pub owner: Option<bool>,

    #[serde(default)]
    pub archived: Option<bool>,
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

    use crate::project_group::list::{ProjectGroupFilter, ProjectGroupMinimal};
    use crate::project_group::project_group_test_routes;

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base")))]
    async fn direct_happy_path(pool: PgPool) {
        let result = super::list(
            &pool,
            CharacterId(1),
            ProjectGroupFilter {
                name:     None,
                owner:    None,
                archived: Some(false),
            },
        )
        .await;
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
            ProjectGroupFilter {
                name:     Some(String::from("Filter")),
                owner:    None,
                archived: Some(false),
            },
        )
        .await;
        assert!(result.is_ok());
        assert_eq!(
            result
                .unwrap()
                .len(),
            2
        );

        let result = super::list(
            &pool,
            CharacterId(1),
            ProjectGroupFilter {
                name:     Some(String::from("SomeGibberish")),
                owner:    None,
                archived: Some(false),
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

        let result = super::list(
            &pool,
            CharacterId(2),
            ProjectGroupFilter {
                name:     None,
                owner:    Some(true),
                archived: Some(false),
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
            ProjectGroupFilter {
                name:     None,
                owner:    Some(false),
                archived: Some(false),
            },
        )
        .await;
        assert!(result.is_ok());
        assert_eq!(
            result
                .unwrap()
                .len(),
            4
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

        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<ProjectGroupMinimal> = serde_json::from_slice(
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

        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<ProjectGroupMinimal> = serde_json::from_slice(
            &response
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes(),
        )
        .unwrap();
        assert_eq!(body.len(), 2);
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

        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let body: Vec<ProjectGroupMinimal> = serde_json::from_slice(
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

        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
