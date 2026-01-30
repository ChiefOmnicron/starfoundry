use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::AppState;
use crate::project_group::error::Result;
use crate::project_group::ProjectGroupUuid;
use crate::project_group::service::delete;

/// Delete Group
/// 
/// - Alternative route: `/latest/project-groups/{ProjectGroupUuid}`
/// - Alternative route: `/v1/project-groups/{ProjectGroupUuid}`
/// 
/// ---
/// 
/// Deletes the group
/// 
/// ## Security
/// - authenticated
/// - project_group:write
/// 
#[utoipa::path(
    delete,
    path = "/{ProjectGroupUuid}",
    tag = "Project Groups",
    params(
        ProjectGroupUuid,
    ),
    responses(
        (
            description = "The group was successfully deleted",
            status = NO_CONTENT,
        ),
        NotFound,
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):             State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
) -> Result<impl IntoResponse> {
    delete(
        &state.pool,
        project_group_uuid,
    ).await?;

    Ok((
        StatusCode::NO_CONTENT,
        ()
    ))
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::header::HOST;
    use axum::http::StatusCode;
    use sqlx::PgPool;
    use starfoundry_lib_gateway::{HEADER_CHARACTER_ID, HEADER_CORPORATION_ID};

    use crate::project_group::project_group_test_routes;

    #[sqlx::test(
        fixtures("base", "delete"),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000010")
            .method("DELETE")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn dont_delete_if_a_project_is_connected(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .method("DELETE")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn unauthorized(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .method("DELETE")
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn forbidden(
        pool: PgPool,
    ) {
        
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .method("DELETE")
            // in the group, but only read permission
            .header(HEADER_CHARACTER_ID, 2)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .method("DELETE")
            // has the permission to write but isn't an owner
            .header(HEADER_CHARACTER_ID, 3)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn not_found(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000010")
            .method("DELETE")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
