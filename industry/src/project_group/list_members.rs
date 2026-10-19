use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_eve_gateway::EveGatewayApiClient;

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::project_group::ProjectGroupError;
use crate::project_group::error::Result;
use crate::project_group::fetch::ProjectGroupMember;
use crate::project_group::permission::ProjectGroupPermission;
use crate::{AppState, ProjectGroupUuid, eve_gateway_api_client};

/// List Group Members
///
/// - Alternative route: `/latest/project-groups/{ProjectGroupUuid}/members`
/// - Alternative route: `/v1/project-groups/{ProjectGroupUuid}/members`
///
/// Lists all members of a group
///
/// ## Security
/// - authenticated
/// - project_group:read
#[utoipa::path(
    get,
    path = "/{ProjectGroupUuid}/members",
    tag = "Project Groups",
    params(
        ProjectGroupUuid,
    ),
    responses(
        (
            body = Vec<ProjectGroupMember>,
            description = "Members of the group",
            status = OK,
        ),
        (
            description = "There aren't any members",
            status = NO_CONTENT,
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
) -> Result<impl IntoResponse> {
    let data = list_members(
        &state.postgres,
        &eve_gateway_api_client()?,
        project_group_uuid,
    )
    .await?;

    if data.is_empty() {
        Ok((StatusCode::NO_CONTENT, Json(data)).into_response())
    } else {
        Ok((StatusCode::OK, Json(data)).into_response())
    }
}

pub async fn list_members(
    pool: &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    project_group_uuid: ProjectGroupUuid,
) -> Result<Vec<ProjectGroupMember>> {
    let entries = sqlx::query!(
        "
            SELECT
                pgm.character_id,
                permission,
                (pg.owner = pgm.character_id) AS is_owner
            FROM project_group_member pgm
            JOIN project_group pg ON pg.id = pgm.project_group_id
            WHERE project_group_id = $1
        ",
        *project_group_uuid,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ProjectGroupError::ListMembers(e, project_group_uuid))?;

    let mut members = Vec::new();
    for entry in entries {
        // TODO: add bulk fetch
        let character = match eve_gateway_api_client
            .fetch_character(
                entry
                    .character_id
                    .into(),
            )
            .await
        {
            Ok(Some(x)) => x,
            Ok(None) => continue,
            Err(_) => continue,
        };

        let member = ProjectGroupMember {
            character:   character,
            permissions: ProjectGroupPermission::new(entry.permission),
            is_owner:    entry
                .is_owner
                .unwrap_or(false),
        };
        members.push(member);
    }
    members.sort_by_key(|x| {
        x.character
            .character_name
            .clone()
    });

    Ok(members)
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::StatusCode;
    use axum::http::header::HOST;
    use http_body_util::BodyExt;
    use sqlx::PgPool;
    use starfoundry_gateway::{
        HEADER_CHARACTER_ID, HEADER_CORPORATION_ID, HEADER_SERVICE
    };

    use crate::project_group::list_members::ProjectGroupMember;
    use crate::project_group::project_group_test_routes;

    #[sqlx::test(fixtures("base"))]
    async fn happy_path(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001/members")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<ProjectGroupMember> = serde_json::from_slice(
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
    async fn unauthorized(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001/members")
            .method("GET")
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(fixtures("base"))]
    async fn forbidden(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000005/members")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[sqlx::test(fixtures("base"))]
    async fn not_found(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000000/members")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
