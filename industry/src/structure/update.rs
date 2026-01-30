use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_industry::StructureUuid;

use crate::structure::service::{update, UpdateStructure};
use crate::AppState;
use crate::structure::error::Result;
use crate::api_docs::{BadRequest, InternalServerError, Unauthorized, UnprocessableEntity, UnsupportedMediaType};

/// Create Structure
/// 
/// - Alternative route: `/latest/structure/{StructureUuid}`
/// - Alternative route: `/v1/structure/{StructureUuid}`
/// 
/// ---
/// 
/// Creates a new structure
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    put,
    path = "/{StructureUuid}",
    tag = "Structures",
    request_body = UpdateStructure,
    responses(
        (
            description = "The structure was updated",
            status = NO_CONTENT,
        ),
        BadRequest,
        Unauthorized,
        UnsupportedMediaType,
        UnprocessableEntity,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):         State<AppState>,
    Path(structure_uuid): Path<StructureUuid>,
    Json(data):           Json<UpdateStructure>,
) -> Result<impl IntoResponse> {
    update(
        &state.pool,
        structure_uuid,
        data,
    ).await?;

    Ok(
        (
            StatusCode::NO_CONTENT,
            ()
        )
    )
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::header::{CONTENT_TYPE, HOST};
    use axum::http::StatusCode;
    use sqlx::PgPool;
    use starfoundry_lib_gateway::{HEADER_CHARACTER_ID, HEADER_CORPORATION_ID};

    use crate::structure::service::UpdateStructure;
    use crate::structure::structure_test_routes;

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&UpdateStructure {
                    rigs: vec![1.into(), 2.into(), 3.into()],
                    services: vec![1.into(), 2.into(), 3.into(), 4.into(), 5.into()],
                }).unwrap()
            ))
            .unwrap();
        let response = structure_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let entry = sqlx::query!("
                SELECT pg.*
                FROM structure pg
                WHERE pg.id = '00000000-0000-0000-0000-000000000001'
            ")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(entry.rigs.len(), 3);
        assert_eq!(entry.rigs, vec![1, 2, 3]);
        assert_eq!(entry.services.len(), 5);
        assert_eq!(entry.services, vec![1, 2, 3, 4, 5]);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn unsupported_media_type(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .header(CONTENT_TYPE, "text/plain")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&UpdateStructure {
                    rigs: vec![1.into()],
                    services: vec![1.into()],
                }).unwrap()
            ))
            .unwrap();
        let response = structure_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn bad_request_no_body(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
