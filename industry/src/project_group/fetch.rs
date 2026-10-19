use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_eve_gateway::EveGatewayApiClient;
use starfoundry_eve_gateway::character::CharacterInfo;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::project_group::ProjectGroupError;
use crate::project_group::error::Result;
use crate::project_group::list_members::list_members;
use crate::project_group::permission::ProjectGroupPermission;
use crate::{AppState, ProjectGroupUuid, eve_gateway_api_client};

/// Fetch Group
///
/// - Alternative route: `/latest/project-groups/{ProjectGroupUuid}`
/// - Alternative route: `/v1/project-groups/{ProjectGroupUuid}`
///
/// ---
///
/// Fetches information about a project group
///
/// ## Security
/// - authenticated
/// - project_group:read
#[utoipa::path(
    get,
    path = "/{ProjectGroupUuid}",
    tag = "Project Groups",
    params(
        ProjectGroupUuid,
    ),
    responses(
        (
            body = ProjectGroup,
            description = "Information about the group",
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
    State(state): State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
    identity: ExtractIdentity,
) -> Result<impl IntoResponse> {
    let entry = fetch(
        &state.postgres,
        &eve_gateway_api_client()?,
        identity.character_id,
        project_group_uuid,
    )
    .await?;

    if let Some(x) = entry {
        Ok((StatusCode::OK, Json(x)).into_response())
    } else {
        Ok((StatusCode::NO_CONTENT, ()).into_response())
    }
}

pub async fn fetch(
    pool: &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    character_id: CharacterId,
    project_group_uuid: ProjectGroupUuid,
) -> Result<Option<ProjectGroup>> {
    let entry = sqlx::query!(
        "
            SELECT
                id,
                name,
                description,
                archived,
                owner = $1 AS is_owner,
                (
                    SELECT COUNT(*)
                    FROM project
                    WHERE project_group_id = $2
                ) AS projects
            FROM project_group pg
            WHERE pg.id = $2
        ",
        *character_id,
        *project_group_uuid,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| ProjectGroupError::FetchGroup(e, project_group_uuid))?;

    if let Some(x) = entry {
        let project_group = ProjectGroup {
            id:            x.id.into(),
            name:          x.name,
            project_count: x
                .projects
                .unwrap_or(0),
            is_owner:      x
                .is_owner
                .unwrap_or_default(),
            description:   x.description,
            archived:      x.archived,
            members:       list_members(
                pool,
                eve_gateway_api_client,
                project_group_uuid,
            )
            .await?,
        };
        Ok(Some(project_group))
    } else {
        Ok(None)
    }
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
pub struct ProjectGroup {
    pub id:            ProjectGroupUuid,
    pub name:          String,
    pub project_count: i64,
    pub is_owner:      bool,
    pub description:   Option<String>,
    pub members:       Vec<ProjectGroupMember>,
    pub archived:      bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "character_name": "SomeCharacterName",
        "character": {
            "character_name": "SomeCharacter",
            "character_id": 1337,
            "corporation_name": "SomeCorporation",
            "corporation_id": 420,
            "alliance_name": "SomeAlliance",
            "alliance_id": 69,
        },
        "permissions": [
            "READ",
            "WRITE_GROUP"
        ],
        "is_owner": false
    })
)]
pub struct ProjectGroupMember {
    pub character:   CharacterInfo,
    pub permissions: ProjectGroupPermission,
    pub is_owner:    bool,
}

#[cfg(test)]
mod fetch_project_group_test {
    use std::str::FromStr;

    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::StatusCode;
    use axum::http::header::HOST;
    use sqlx::PgPool;
    use starfoundry_gateway::{
        HEADER_CHARACTER_ID, HEADER_CORPORATION_ID, HEADER_SERVICE
    };
    use starfoundry_lib_types::CharacterId;
    use uuid::Uuid;

    use crate::project_group::project_group_test_routes;
    use crate::test_util::EveGatewayTestApiClient;

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base"),))]
    async fn direct_happy_path(pool: PgPool) {
        let gateway_client = EveGatewayTestApiClient::new();

        let response = super::fetch(
            &pool,
            &gateway_client,
            CharacterId(1),
            Uuid::from_str("00000000-0000-0000-0000-000000000001")
                .unwrap()
                .into(),
        )
        .await
        .unwrap();

        let response = response.unwrap();
        assert_eq!(response.is_owner, true);
        assert_eq!(response.name, "First".to_string());
        assert_eq!(response.description, Some("Description".into()));
    }

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base"),))]
    async fn direct_not_found(pool: PgPool) {
        let gateway_client = EveGatewayTestApiClient::new();

        let response = super::fetch(
            &pool,
            &gateway_client,
            CharacterId(1),
            Uuid::from_str("00000000-0000-0000-0000-000000000000")
                .unwrap()
                .into(),
        )
        .await;

        assert!(
            response
                .unwrap()
                .is_none()
        );
    }

    #[sqlx::test(fixtures("base"))]
    async fn happy_path(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[sqlx::test(fixtures("base"))]
    async fn unauthorized(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .method("GET")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(fixtures("base"))]
    async fn forbidden(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000005")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[sqlx::test(fixtures("base"))]
    async fn not_found(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000000")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
