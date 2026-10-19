use std::collections::HashMap;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_eve_gateway::EveGatewayApiClient;
use starfoundry_eve_gateway::asset::Item;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::project_group::ProjectGroupError;
use crate::project_group::error::Result;
use crate::{
    AppState, ProjectGroupUuid, eve_gateway_api_client, sort_by_job_flat
};

/// List Job Splittings
///
/// - Alternative route:
///   `/latest/project-groups/{ProjectGroupUuid}/defaults/job-splitting`
/// - Alternative route:
///   `/v1/project-groups/{ProjectGroupUuid}/defaults/job-splitting`
///
/// ---
///
/// Fetches the defaults configured for job splitting
///
/// ## Security
/// - authenticated
/// - project_group:read
#[utoipa::path(
    get,
    path = "/{ProjectGroupUuid}/defaults/job-splitting",
    tag = "Project Groups",
    params(
        ProjectGroupUuid,
    ),
    responses(
        (
            body = Vec<JobSplitting>,
            description = "All job splitting configurations that are configured for a project group",
            status = OK,
        ),
        NotFound,
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
) -> Result<impl IntoResponse> {
    let data = list_default_job_splitting(
        &state.postgres,
        &eve_gateway_api_client()?,
        project_group_uuid,
    )
    .await?;

    Ok((StatusCode::OK, Json(data)).into_response())
}

/// 3 days in seconds
const DEFAULT_TIME_JOB_SPLITTING: i32 = 259_200i32;

pub async fn list_default_job_splitting(
    pool: &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    project_group_uuid: ProjectGroupUuid,
) -> Result<JobSplitting> {
    let general = sqlx::query!(
        "
            SELECT
                manufacturing,
                reaction
            FROM project_group_default_job_splitting_general
            WHERE project_group_id = $1
        ",
        *project_group_uuid,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        ProjectGroupError::FetchGroupDefaults(e, project_group_uuid)
    })?;

    let general = if let Some(x) = general {
        JobSplittingGeneral {
            manufacturing: x.manufacturing,
            reaction:      x.reaction,
        }
    } else {
        JobSplittingGeneral {
            manufacturing: DEFAULT_TIME_JOB_SPLITTING,
            reaction:      DEFAULT_TIME_JOB_SPLITTING,
        }
    };

    let run_entries = sqlx::query!(
        "
            SELECT
                type_id,
                max_runs
            FROM project_group_default_job_splitting_run
            WHERE project_group_id = $1
        ",
        *project_group_uuid,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        ProjectGroupError::FetchGroupDefaults(e, project_group_uuid)
    })?;

    let type_ids = run_entries
        .iter()
        .map(|x| {
            x.type_id
                .into()
        })
        .collect::<Vec<_>>();
    let items = eve_gateway_api_client
        .fetch_item_bulk(type_ids)
        .await?
        .into_iter()
        .map(|x| (x.type_id, x))
        .collect::<HashMap<_, _>>();

    let mut runs = Vec::new();
    for entry in run_entries {
        if let Some(x) = items.get(
            &entry
                .type_id
                .into(),
        ) {
            runs.push(JobSplittingRun {
                id:       Uuid::now_v7(),
                max_runs: entry.max_runs,
                item:     x.clone(),
                runs:     1i32,
            })
        } else {
            // silently ignore errors
            tracing::debug!("Couldn't find item {}", entry.type_id);
            continue
        }
    }

    let runs = sort_entries(runs);

    Ok(JobSplitting {
        general,
        runs,
    })
}

sort_by_job_flat!(sort_entries, JobSplittingRun);

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct JobSplitting {
    pub general: JobSplittingGeneral,
    pub runs:    Vec<JobSplittingRun>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "manufacturing": 259200,
        "reaction": 259200
    })
)]
pub struct JobSplittingGeneral {
    pub manufacturing: i32,
    pub reaction:      i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "max_runs": 40,
        "item": {
            "base_price": null,
            "category": {
                "category_id": 0,
                "name": "#System"
            },
            "group": {
                "group_id": 0,
                "category_id": 0,
                "name": "#System"
            },
            "meta_group_id": null,
            "name": "Ragnarok",
            "repackaged": 10000000,
            "type_id": 23773,
            "volume": 100000000
        }
    })
)]
pub struct JobSplittingRun {
    // only needed for sorting
    #[serde(skip)]
    pub id:       Uuid,
    pub max_runs: i32,
    pub item:     Item,
    // for sorting
    #[serde(skip)]
    pub runs:     i32,
}

#[cfg(test)]
mod list_default_market_project_group_test {
    use std::str::FromStr;

    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::StatusCode;
    use axum::http::header::HOST;
    use http_body_util::BodyExt;
    use sqlx::PgPool;
    use starfoundry_gateway::{
        HEADER_CHARACTER_ID, HEADER_CORPORATION_ID, HEADER_SERVICE
    };
    use uuid::Uuid;

    use crate::eve_gateway_api_client;
    use crate::project_group::list_default_job_splitting::JobSplitting;
    use crate::project_group::project_group_test_routes;

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base"),))]
    async fn direct_happy_path(pool: PgPool) {
        let response = super::list_default_job_splitting(
            &pool,
            &eve_gateway_api_client().unwrap(),
            Uuid::from_str("00000000-0000-0000-0000-000000000001")
                .unwrap()
                .into(),
        )
        .await
        .unwrap();

        assert_eq!(
            response
                .general
                .manufacturing,
            259200
        );
        assert_eq!(
            response
                .general
                .reaction,
            259200
        );
        assert_eq!(
            response
                .runs
                .len(),
            1
        );
    }

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base"),))]
    async fn default_if_entry_does_not_exist(pool: PgPool) {
        let response = super::list_default_job_splitting(
            &pool,
            &eve_gateway_api_client().unwrap(),
            Uuid::from_str("00000000-0000-0000-0000-000000000000")
                .unwrap()
                .into(),
        )
        .await
        .unwrap();

        assert_eq!(
            response
                .runs
                .len(),
            0
        );
    }

    #[sqlx::test(fixtures("base"))]
    async fn happy_path_all(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001/defaults/job-splitting")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();

        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: JobSplitting = serde_json::from_slice(
            &response
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes(),
        )
        .unwrap();
        assert_eq!(body.runs.len(), 1);
    }

    #[sqlx::test(fixtures("base"))]
    async fn unauthorized(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001/defaults/job-splitting")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(fixtures("base"))]
    async fn forbidden(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000005/defaults/job-splitting")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();

        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[sqlx::test(fixtures("base"))]
    async fn not_found(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000000/defaults/job-splitting")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();

        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
