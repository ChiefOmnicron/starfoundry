use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

use crate::api_docs::{
    Forbidden, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType
};
use crate::project_group::ProjectGroupError;
use crate::project_group::error::Result;
use crate::{AppState, ProjectGroupUuid};

/// Update Default Job Splitting
///
/// - Alternative route:
///   `/v1/project-groups/{ProjectGroupUuid}/defaults/job-splitting`
/// - Alternative route:
///   `/latest/project-groups/{ProjectGroupUuid}/defaults/job-splitting`
///
/// ---
///
/// Updates a default job splitting
///
/// ## Security
/// - authenticated
/// - project_group:write
#[utoipa::path(
    put,
    path = "/{ProjectGroupUuid}/defaults/job-splitting",
    tag = "Project Groups",
    request_body = Vec<UpdateProjectGroupDefaultJobSplitting>,
    params(
        ProjectGroupUuid,
    ),
    responses(
        (
            description = "The group was updated",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
        UnsupportedMediaType,
        UnprocessableEntity,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
    Json(update_info): Json<Vec<UpdateProjectGroupDefaultJobSplitting>>,
) -> Result<impl IntoResponse> {
    update_default_job_splitting(
        &state.postgres,
        project_group_uuid,
        update_info,
    )
    .await?;

    Ok((StatusCode::NO_CONTENT,))
}

pub async fn update_default_job_splitting(
    pool: &PgPool,
    project_group_uuid: ProjectGroupUuid,
    job_splitting: Vec<UpdateProjectGroupDefaultJobSplitting>,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(ProjectGroupError::TransactionError)?;

    sqlx::query!(
        "
            DELETE FROM project_group_default_job_splitting_run
            WHERE project_group_id = $1
        ",
        *project_group_uuid,
    )
    .execute(&mut *transaction)
    .await
    .map_err(|e| {
        ProjectGroupError::DeleteGroupDefaults(e, project_group_uuid)
    })?;

    sqlx::query!(
        "
            INSERT INTO project_group_default_job_splitting_run
            (
                project_group_id,
                type_id,
                max_runs
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[]
            )
        ",
        *project_group_uuid,
        &job_splitting
            .iter()
            .map(|x| *x.type_id)
            .collect::<Vec<_>>(),
        &job_splitting
            .iter()
            .map(|x| x.max_runs)
            .collect::<Vec<_>>(),
    )
    .execute(&mut *transaction)
    .await
    .map_err(|e| {
        ProjectGroupError::UpdateGroupDefaults(e, project_group_uuid)
    })?;

    transaction
        .commit()
        .await
        .map_err(ProjectGroupError::TransactionError)
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "material_efficiency": 10,
        "type_id": 23773
    })
)]
pub struct UpdateProjectGroupDefaultJobSplitting {
    pub type_id:  TypeId,
    pub max_runs: i32,
}

#[cfg(test)]
mod update_default_blueprint_overwrite_project_group_test {
    use std::str::FromStr;

    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::StatusCode;
    use axum::http::header::{CONTENT_TYPE, HOST};
    use sqlx::PgPool;
    use starfoundry_gateway::{
        HEADER_CHARACTER_ID, HEADER_CORPORATION_ID, HEADER_SERVICE
    };
    use starfoundry_lib_types::TypeId;
    use uuid::Uuid;

    use crate::project_group::project_group_test_routes;
    use crate::project_group::update_default_job_splitting::UpdateProjectGroupDefaultJobSplitting;

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base"),))]
    async fn direct_happy_path(pool: PgPool) {
        let response = super::update_default_job_splitting(
            &pool,
            Uuid::from_str("00000000-0000-0000-0000-000000000001")
                .unwrap()
                .into(),
            vec![UpdateProjectGroupDefaultJobSplitting {
                type_id:  TypeId(1),
                max_runs: 1,
            }],
        )
        .await;
        assert!(response.is_ok());
    }

    #[sqlx::test(fixtures("base"))]
    async fn happy_path(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001/defaults/job-splitting")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&vec![
                    UpdateProjectGroupDefaultJobSplitting {
                        max_runs: 40,
                        type_id:  TypeId(21027),
                    },
                ])
                .unwrap(),
            ))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let entry = sqlx::query!("
                SELECT pg.*
                FROM project_group_default_job_splitting_run pg
                WHERE pg.project_group_id = '00000000-0000-0000-0000-000000000001'
            ")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(entry.max_runs, 40);
    }

    #[sqlx::test(fixtures("base"))]
    async fn unsupported_media_type(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001/defaults/job-splitting")
            .header(CONTENT_TYPE, "text/plain")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::new(serde_json::to_string(&vec![TypeId(1)]).unwrap()))
            .unwrap();
        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[sqlx::test(fixtures("base"))]
    async fn bad_request_no_body(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001/defaults/job-splitting")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(fixtures("base"))]
    async fn unauthorized(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001/defaults/job-splitting")
            .header(CONTENT_TYPE, "application/json")
            .method("PUT")
            .body(Body::new(serde_json::to_string(&vec![TypeId(1)]).unwrap()))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(fixtures("base"))]
    async fn forbidden(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001/defaults/job-splitting")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 2)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::new(serde_json::to_string(&vec![TypeId(1)]).unwrap()))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[sqlx::test(fixtures("base"))]
    async fn not_found(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000010/defaults/job-splitting")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::new(serde_json::to_string(&vec![TypeId(1)]).unwrap()))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
