use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use starfoundry_lib_gateway::ExtractIdentity;
use utoipa::ToSchema;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::AppState;
use crate::project::error::Result;
use crate::project::ProjectUuid;
use crate::project::service::{CreateProject, create};

/// List Groups
/// 
/// - Alternative route: `/latest/projects`
/// - Alternative route: `/v1/projects`
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
    post,
    path = "/",
    tag = "projects",
    request_body = CreateProjectResponse,
    responses(
        (
            body = ProjectUuid,
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
    identity:           ExtractIdentity,
    State(state):       State<AppState>,
    Json(project_info): Json<CreateProject>,
) -> Result<impl IntoResponse> {
    let id = create(
            &state.pool,
            identity.character_id,
            project_info,
        ).await?;

    Ok(
        (
            StatusCode::CREATED,
            Json(CreateProjectResponse {
                id: id.into(),
            })
        )
    )
}

#[derive(Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "fd324c9f-ecda-49c8-948e-18f4b4b23bff"
    })
)]
pub struct CreateProjectResponse {
    id: ProjectUuid,
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
    use crate::project::service::ProjectList;

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

        let response = project_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<ProjectList> = serde_json::from_slice(
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

        let response = project_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<ProjectList> = serde_json::from_slice(
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
            .uri("/?name=SomeGibberish")
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
