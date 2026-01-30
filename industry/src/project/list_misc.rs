use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;

use crate::api_docs::{BadRequest, InternalServerError, NotFound, Unauthorized};
use crate::AppState;
use crate::project::error::Result;
use crate::project::ProjectUuid;
use crate::project::service::{ProjectMisc, list_misc};

/// List Groups
/// 
/// - Alternative route: `/latest/projects/{ProjectUuid}/misc`
/// - Alternative route: `/v1/projects/{ProjectUuid}/misc`
/// 
/// ---
/// 
/// Lists all misc entries that belong to the project
/// 
/// ## Security
/// - authenticated
/// - project:read
/// 
#[utoipa::path(
    get,
    path = "/{ProjectUuid}/misc",
    tag = "projects",
    params(
        ProjectUuid,
    ),
    responses(
        (
            body = Vec<ProjectMisc>,
            description = "List all misc entries for a project",
            status = OK,
        ),
        (
            description = "There aren't any misc entries",
            status = NO_CONTENT,
        ),
        NotFound,
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    _identity:        ExtractIdentity,
    State(state):     State<AppState>,
    Path(project_id): Path<ProjectUuid>,
) -> Result<impl IntoResponse> {
    let data = list_misc(
            &state.pool,
            project_id,
        ).await?;

    if data.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(data),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(data),
            )
            .into_response()
        )
    }
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::header::HOST;
    use axum::http::StatusCode;
    use http_body_util::BodyExt;
    use sqlx::PgPool;
    use starfoundry_lib_gateway::{HEADER_CHARACTER_ID, HEADER_CORPORATION_ID};

    use crate::project::project_test_routes;
    use crate::project::service::{ProjectJobGroup, ProjectList};

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn happy_path_all(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000101/misc")
            .method("GET")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();

        let response = project_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<ProjectJobGroup> = serde_json::from_slice(
            &response.into_body().collect().await.unwrap().to_bytes()
        ).unwrap();
        assert_eq!(body.len(), 1);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn happy_path_empty(
        pool: PgPool,
    ) {
        // Empty
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000103/misc")
            .method("GET")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();

        let response = project_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let body: Vec<ProjectList> = serde_json::from_slice(
            &response.into_body().collect().await.unwrap().to_bytes()
        ).unwrap();
        assert_eq!(body.len(), 0);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn not_found(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000115/misc")
            .method("GET")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();

        let response = project_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn unauthorized(
        pool: PgPool,
    ) {
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
