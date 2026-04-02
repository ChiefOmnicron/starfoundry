use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_industry::StructureUuid;
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

use crate::project::ProjectUuid;
use crate::project::error::{ProjectError, Result};

pub async fn add_job(
    pool:       &PgPool,
    project_id: ProjectUuid,
    entries:    Vec<AddJobEntryRequest>,
) -> Result<()> {
    let type_ids = entries
        .iter()
        .map(|x| *x.type_id)
        .collect::<Vec<_>>();
    let runs = entries
        .iter()
        .map(|x| x.runs)
        .collect::<Vec<_>>();
    let structure_ids = entries
        .iter()
        .map(|x| *x.structure_id)
        .collect::<Vec<_>>();

    sqlx::query!("
            INSERT INTO project_job
            (
                project_id,
                type_id,
                runs,
                structure_id
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[],
                $4::UUID[]
            )
        ",
            *project_id,
            &type_ids,
            &runs,
            &structure_ids,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(ProjectError::AddJobEntry)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddJobEntryRequest {
    pub type_id:        TypeId,
    pub runs:           i32,
    pub structure_id:   StructureUuid,
}
