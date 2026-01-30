use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::AppState;
use crate::eve_gateway_api_client;
use crate::project_group::error::Result;
use crate::project_group::service::{ProjectGroup, ProjectGroupFilter, list};

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
/// 
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
    State(state):  State<AppState>,
    Query(filter): Query<ProjectGroupFilter>,
    identity:      ExtractIdentity,
) -> Result<impl IntoResponse> {
    let data = list(
            &state.pool,
            identity.character_id,
            &eve_gateway_api_client()?,
            filter,
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

    use crate::project_group::list::ProjectGroup;
    use crate::project_group::project_group_test_routes;

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn happy_path_all(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/")
            .method("GET")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();

        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<ProjectGroup> = serde_json::from_slice(
            &response.into_body().collect().await.unwrap().to_bytes()
        ).unwrap();
        assert_eq!(body.len(), 4);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn happy_path_filter(
        pool: PgPool,
    ) {
        // Filter
        let request = Request::builder()
            .uri("/?name=Filter")
            .method("GET")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();

        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<ProjectGroup> = serde_json::from_slice(
            &response.into_body().collect().await.unwrap().to_bytes()
        ).unwrap();
        assert_eq!(body.len(), 2);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn happy_path_empty(
        pool: PgPool,
    ) {
        // Empty
        let request = Request::builder()
            .uri("/?name=SomeGibberish")
            .method("GET")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();

        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let body: Vec<ProjectGroup> = serde_json::from_slice(
            &response.into_body().collect().await.unwrap().to_bytes()
        ).unwrap();
        assert_eq!(body.len(), 0);
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

        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
