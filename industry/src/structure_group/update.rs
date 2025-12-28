use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::AppState;
use crate::structure_group::error::Result;
use crate::structure_group::service::{UpdateStructureGroup, update};
use crate::structure_group::StructureGroupUuid;

/// Update Structure Groups
/// 
/// - Alternative route: `/latest/structures-groups/{StructureUuid}`
/// - Alternative route: `/v1/structures-groups/{StructureUuid}`
/// 
/// ---
/// 
/// Lists all available structure groups
/// 
/// ## Security
/// - authenticated
/// - structure-group:read
/// 
#[utoipa::path(
    put,
    path = "/{StructureUuid}",
    tag = "Structure Groups",
    params(
        StructureGroupUuid,
    ),
    responses(
        (
            description = "The structure group was successfully updated",
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
    identity:                   ExtractIdentity,
    State(state):               State<AppState>,
    Path(structure_group_uuid): Path<StructureGroupUuid>,
    Json(structure_group):      Json<UpdateStructureGroup>,
) -> Result<impl IntoResponse> {
    let data = update(
            &state.pool,
            identity.character_id,
            structure_group_uuid,
            structure_group,
        )
        .await?;

    Ok(
        (
            StatusCode::NO_CONTENT,
            Json(data),
        )
        .into_response()
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

    use crate::structure_group::service::UpdateStructureGroup;
    use crate::structure_group::structure_group_test_routes;

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn happy_path_all(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .header(CONTENT_TYPE, "application/json")
            .uri("/00000000-0000-0000-0000-100000000001")
            .method("PUT")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::new(
                serde_json::to_string(&UpdateStructureGroup {
                    name: "My cool structure group".into(),
                    structures: Vec::new(),
                }).unwrap()
            ))
            .unwrap();
        let response = structure_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn unauthorized(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-100000000001")
            .method("PUT")
            .body(Body::empty())
            .unwrap();
        let response = structure_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn forbidden(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .header(CONTENT_TYPE, "application/json")
            .uri("/00000000-0000-0000-0000-100000000002")
            .method("PUT")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::new(
                serde_json::to_string(&UpdateStructureGroup {
                    name: "My cool structure group".into(),
                    structures: Vec::new(),
                }).unwrap()
            ))
            .unwrap();
        let response = structure_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
}
