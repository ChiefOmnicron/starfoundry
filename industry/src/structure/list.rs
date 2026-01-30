use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_industry::Structure;

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::structure::service::{list, StructureFilter};
use crate::{eve_gateway_api_client, AppState};
use crate::structure::error::Result;

/// List Structures
/// 
/// - Alternative route: `/latest/structures`
/// - Alternative route: `/v1/structures`
/// 
/// ---
/// 
/// Lists all available structures
/// 
/// ## Security
/// - authenticated
/// - structure:read
/// 
#[utoipa::path(
    get,
    path = "/",
    tag = "Structures",
    params(StructureFilter),
    responses(
        (
            body = Vec<Structure>,
            description = "All structures the user has access to",
            status = OK,
        ),
        (
            description = "No results for your request",
            status = NO_CONTENT
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
    identity:      ExtractIdentity,
    State(state):  State<AppState>,
    Query(filter): Query<StructureFilter>,
) -> Result<impl IntoResponse> {
    let data = list(
            &state.pool,
            &eve_gateway_api_client()?,
            identity.character_id,
            filter,
        )
        .await?;

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
    use starfoundry_lib_industry::Structure;

    use crate::structure::structure_test_routes;

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
        let response = structure_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<Structure> = serde_json::from_slice(
            &response.into_body().collect().await.unwrap().to_bytes()
        ).unwrap();
        assert_eq!(body.len(), 2);
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
        let response = structure_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<Structure> = serde_json::from_slice(
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
        let response = structure_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let body: Vec<Structure> = serde_json::from_slice(
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
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
