use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_industry::{Structure, StructureUuid};

use crate::eve_gateway_api_client;
use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::structure::service::{fetch, FetchStructureQuery};
use crate::AppState;
use crate::structure::error::Result;

/// Fetch Structure
/// 
/// - Alternative route: `/latest/structures/{StructureUuid}`
/// - Alternative route: `/v1/structures/{StructureUuid}`
/// 
/// ---
/// 
/// Fetches information about a structure
/// 
/// ## Security
/// - authenticated
/// - structure:read
/// 
#[utoipa::path(
    get,
    path = "/{StructureUuid}",
    tag = "Structures",
    params(
        StructureUuid,
        FetchStructureQuery,
    ),
    responses(
        (
            body = Structure,
            description = "Information about the structure",
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
    identity:             ExtractIdentity,
    State(state):         State<AppState>,
    Path(structure_uuid): Path<StructureUuid>,
    Query(options):       Query<FetchStructureQuery>,
) -> Result<impl IntoResponse> {
    let entry = fetch(
            &state.pool,
            &eve_gateway_api_client()?,
            identity.character_id,
            structure_uuid,
            options,
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
    use axum::http::header::HOST;
    use axum::http::StatusCode;
    use sqlx::PgPool;
    use starfoundry_lib_gateway::{HEADER_CHARACTER_ID, HEADER_CORPORATION_ID};

    use crate::structure::structure_test_routes;

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .method("GET")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn unauthorized(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .method("GET")
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn forbidden(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000002")
            .method("GET")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn not_found(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000000")
            .method("GET")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
