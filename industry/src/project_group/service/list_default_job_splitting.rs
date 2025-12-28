use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, Item};
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;

/// 3 days in seconds
const DEFAULT_TIME_JOB_SPLITTING: i32 = 259_200i32;

pub async fn list_default_job_splitting(
    pool:                   &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    project_group_uuid:     ProjectGroupUuid,
) -> Result<JobSplitting> {
    let general = sqlx::query!("
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
        .map_err(|e| ProjectGroupError::FetchGroupDefaults(e, project_group_uuid))?;

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

    let run_entries = sqlx::query!("
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
        .map_err(|e| ProjectGroupError::FetchGroupDefaults(e, project_group_uuid))?;

    let type_ids = run_entries
        .iter()
        .map(|x| x.type_id.into())
        .collect::<Vec<_>>();
    let items = eve_gateway_api_client
        .fetch_item_bulk(type_ids)
        .await?
        .into_iter()
        .map(|x| (x.type_id, x))
        .collect::<HashMap<_, _>>();

    let mut runs = Vec::new();
    for entry in run_entries {
        if let Some(x) = items.get(&entry.type_id.into()) {
            runs.push(JobSplittingRun {
                max_runs: entry.max_runs,
                item:     x.clone(),
            })
        } else {
            // silently ignore errors
            tracing::debug!("Couldn't find item {}", entry.type_id);
            continue
        }
    }

    Ok(JobSplitting {
        general,
        runs,
    })
}

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

#[derive(Debug, Deserialize, Serialize, ToSchema)]
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
    pub max_runs: i32,
    pub item:     Item,
}

#[cfg(test)]
mod list_default_market_project_group_test {
    use sqlx::PgPool;
    use std::str::FromStr;
    use uuid::Uuid;

    use crate::eve_gateway_api_client;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base"),
        ),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let response = super::list_default_job_splitting(
                &pool,
                &eve_gateway_api_client().unwrap(),
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
            )
            .await
            .unwrap();

        assert_eq!(response.general.manufacturing, 259200);
        assert_eq!(response.general.reaction, 259200);
        assert_eq!(response.runs.len(), 1);
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base"),
        ),
    )]
    async fn default_if_entry_does_not_exist(
        pool: PgPool,
    ) {
        let response = super::list_default_job_splitting(
                &pool,
                &eve_gateway_api_client().unwrap(),
                Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap().into(),
            )
            .await
            .unwrap();

        assert_eq!(response.runs.len(), 0);
    }
}
