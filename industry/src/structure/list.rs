mod filter;
mod service;

pub use self::service::*;

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::AppState;
use crate::structure::error::Result;
use crate::structure::Structure;
use crate::structure::list::filter::StructureFilter;
use starfoundry_lib_eve_gateway::ExtractIdentity;

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
    State(state):  State<AppState>,
    Query(filter): Query<StructureFilter>,
    identity:      ExtractIdentity,
) -> Result<impl IntoResponse> {
    let data = list(
            &state.pool,
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
    use axum::http::header::{AUTHORIZATION, HOST};
    use axum::http::StatusCode;
    use http_body_util::BodyExt;
    use sqlx::PgPool;
    use starfoundry_lib_eve_gateway::{HEADER_CHARACTER_ID, HEADER_CORPORATION_ID};
    use starfoundry_lib_eve_gateway::test::JwtTokenForTesting;
    use starfoundry_lib_types::CharacterId;

    use crate::structure::{structure_test_routes, Structure};

    #[sqlx::test(
        fixtures("DELETE_AFTER_NEW_MS", "base"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn happy_path_all(
        pool: PgPool,
    ) {
        let token = JwtTokenForTesting::new(CharacterId(1));
        let request = Request::builder()
            .uri("/")
            .method("GET")
            .header(AUTHORIZATION, token.generate())
            .header(HOST, "test.starfoundry.space")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
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
        fixtures("DELETE_AFTER_NEW_MS", "base"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn happy_path_filter(
        pool: PgPool,
    ) {
        // Filter
        let token = JwtTokenForTesting::new(CharacterId(1));
        let request = Request::builder()
            .uri("/?name=Filter")
            .method("GET")
            .header(AUTHORIZATION, token.generate())
            .header(HOST, "test.starfoundry.space")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
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
        fixtures("DELETE_AFTER_NEW_MS", "base"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn happy_path_empty(
        pool: PgPool,
    ) {
        // Empty
        let token = JwtTokenForTesting::new(CharacterId(1));
        let request = Request::builder()
            .uri("/?name=SomeGibberish")
            .method("GET")
            .header(AUTHORIZATION, token.generate())
            .header(HOST, "test.starfoundry.space")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
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
        fixtures("DELETE_AFTER_NEW_MS", "base"),
        migrator = "crate::test_util::MIGRATOR"
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
        let response = structure_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
