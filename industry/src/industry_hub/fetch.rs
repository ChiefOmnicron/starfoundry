use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::AppState;
use crate::eve_gateway_api_client;
use crate::industry_hub::error::Result;
use crate::industry_hub::IndustryHubUuid;
use crate::industry_hub::service::{IndustryHub, fetch};

/// Fetch Industry Hub
/// 
/// - Alternative route: `/latest/industry-hub/{IndustryHubUuid}`
/// - Alternative route: `/v1/industry-hub/{IndustryHubUuid}`
/// 
/// ---
/// 
/// Fetches information about a industry hub
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    get,
    path = "/{IndustryHubUuid}",
    tag = "Industry Hubs",
    params(
        IndustryHubUuid,
    ),
    responses(
        (
            body = IndustryHub,
            description = "Information about the industry hub",
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
    identity:                ExtractIdentity,
    State(state):            State<AppState>,
    Path(industry_hub_uuid): Path<IndustryHubUuid>,
) -> Result<impl IntoResponse> {
    let entry = fetch(
            &state.pool,
            &eve_gateway_api_client()?,
            identity.character_id,
            industry_hub_uuid,
        )
        .await?;

    if let Some(x) = entry {
        Ok(
            (
                StatusCode::OK,
                Json(x)
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::NO_CONTENT,
                ()
            )
            .into_response()
        )
    }
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::StatusCode;
    use sqlx::PgPool;
    use starfoundry_lib_gateway::{HEADER_CHARACTER_ID, HEADER_CORPORATION_ID};

    use crate::industry_hub::industry_hub_test_routes;
    use axum::http::header::HOST;

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-100000000001")
            .method("GET")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = industry_hub_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn unauthorized(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-100000000001")
            .method("GET")
            .body(Body::empty())
            .unwrap();
        let response = industry_hub_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn forbidden(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-100000000002")
            .method("GET")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = industry_hub_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn not_found(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-100000000000")
            .method("GET")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = industry_hub_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
