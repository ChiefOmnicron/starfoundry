use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, ResolveStructureResponse};
use starfoundry_lib_types::StructureId;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::eve_gateway_api_client;
use crate::structure::error::Result;
use crate::structure::StructureUuid;

/// Resolve Structure
/// 
/// - Alternative route: `/latest/structures/resolve/{StructureUuid}`
/// - Alternative route: `/v1/structures/resolve/{StructureUuid}`
/// 
/// ---
/// 
/// Resolves information about a given structure id.
/// 
/// Note: The eve character needs to have access to the structure.
/// If you can search for it in-game, and find it, you are good, otherwise
/// it won't show up and return an error.
/// 
/// The `StructureId` needs to be larger than 1_000_000_000_000.
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    get,
    path = "/resolve/{StructureUuid}",
    tag = "Structures",
    params(
        StructureUuid,
    ),
    responses(
        (
            body = ResolveStructureResponse,
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
    Path(structure_id): Path<StructureId>,
) -> Result<impl IntoResponse> {
    // TODO: proper return when this errors
    let entry = eve_gateway_api_client()?
        .resolve_structure(structure_id)
        .await?;

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::StatusCode;
    use sqlx::PgPool;
    use starfoundry_lib_gateway::{HEADER_CHARACTER_ID, HEADER_CORPORATION_ID};

    use crate::structure::structure_test_routes;

    #[sqlx::test(
        fixtures("DELETE_AFTER_NEW_MS", "base"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .method("GET")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[sqlx::test(
        fixtures("DELETE_AFTER_NEW_MS", "base"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn unauthorized(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .method("GET")
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(
        fixtures("DELETE_AFTER_NEW_MS", "base"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn forbidden(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000002")
            .method("GET")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[sqlx::test(
        fixtures("DELETE_AFTER_NEW_MS", "base"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn not_found(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000000")
            .method("GET")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
