use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::market::error::Result;
use crate::market::service::{MarketBulkRequest, bulk};
use crate::AppState;

/// Bulk Market Data
/// 
/// - Alternative route: `/latest/market/bulk`
/// - Alternative route: `/v1/market/bulk`
/// 
/// ---
/// 
/// Bulk get data from the markets
/// 
/// ## Security
/// - authenticated
/// - structure:read
/// 
#[utoipa::path(
    get,
    path = "/",
    tag = "Markets",
    request_body = MarketBulkRequest,
    responses(
        (
            body = Vec<String>,
            description = "List of all matching market entries",
            status = OK,
        ),
        (
            description = "No markets have the requested amount available",
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
    _identity:     ExtractIdentity,
    State(state):  State<AppState>,
    Json(request): Json<MarketBulkRequest>,
) -> Result<impl IntoResponse> {
    let data = bulk(
            &state.pool,
            request,
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

/*#[cfg(test)]
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
*/
