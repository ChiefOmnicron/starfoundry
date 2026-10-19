use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_eve_gateway::EveGatewayApiClient;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::project_group::ProjectGroupError;
use crate::project_group::error::Result;
use crate::project_group::fetch::ProjectGroupMember;
use crate::project_group::permission::ProjectGroupPermission;
use crate::{AppState, ProjectGroupUuid, eve_gateway_api_client};

/// Fetch Members Self
///
/// - Alternative route:
///   `/latest/project-groups/{ProjectGroupUuid}/members/self`
/// - Alternative route: `/v1/project-groups/{ProjectGroupUuid}/members/self`
///
/// Fetches yourself
///
/// ## Security
/// - authenticated
/// - project_group:read
#[utoipa::path(
    get,
    path = "/{ProjectGroupUuid}/members/self",
    tag = "Project Groups",
    params(
        ProjectGroupUuid,
    ),
    responses(
        (
            body = ProjectGroupMember,
            description = "Your identity within the project",
            status = OK,
        ),
        Unauthorized,
        Forbidden,
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
    let data = fetch_members_self(
        &state.postgres,
        identity.character_id,
        &eve_gateway_api_client()?,
        project_group_uuid,
    )
    .await?;

    Ok((StatusCode::OK, Json(data)).into_response())
}

pub async fn fetch_members_self(
    pool: &PgPool,
    character_id: CharacterId,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    project_group_uuid: ProjectGroupUuid,
) -> Result<Option<ProjectGroupMember>> {
    let entry = sqlx::query!(
        "
            SELECT
                permission,
                (pg.owner = $2) AS is_owner
            FROM project_group_member pgm
            JOIN project_group pg ON pg.id = pgm.project_group_id
            WHERE project_group_id = $1
            AND character_id = $2
        ",
        *project_group_uuid,
        *character_id,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| ProjectGroupError::FetchMembersSelf(e, project_group_uuid))?;

    match eve_gateway_api_client
        .fetch_character(character_id)
        .await
    {
        Ok(Some(x)) => {
            Ok(Some(ProjectGroupMember {
                character:   x,
                permissions: ProjectGroupPermission::new(entry.permission),
                is_owner:    entry
                    .is_owner
                    .unwrap_or(false),
            }))
        },
        Ok(None) => Ok(None),
        Err(_) => Ok(None),
    }
}

#[cfg(test)]
mod fetch_members_self_test {
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

        let response = super::fetch_members_self(
            &pool,
            CharacterId(1),
            &gateway_client,
            Uuid::from_str("00000000-0000-0000-0000-000000000001")
                .unwrap()
                .into(),
        )
        .await;

        assert!(response.is_ok());
    }

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base"),))]
    async fn default_if_entry_does_not_exist(pool: PgPool) {
        let gateway_client = EveGatewayTestApiClient::new();

        let response = super::fetch_members_self(
            &pool,
            CharacterId(1),
            &gateway_client,
            Uuid::from_str("00000000-0000-0000-0000-000000000000")
                .unwrap()
                .into(),
        )
        .await;

        assert!(response.is_err());
    }

    #[sqlx::test(fixtures("base"))]
    async fn happy_path(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001/members/self")
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
            .uri("/00000000-0000-0000-0000-000000000001/members/self")
            .method("GET")
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(fixtures("base"))]
    async fn forbidden(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000005/members/self")
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
            .uri("/00000000-0000-0000-0000-000000000000/members/self")
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
