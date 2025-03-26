use sqlx::PgPool;
use starfoundry_libs_types::TypeId;

use crate::{Error, ProjectGroupDefault, ProjectGroupUuid, Result};

pub async fn fetch_defaults(
    pool:       &PgPool,
    group_uuid: ProjectGroupUuid,
) -> Result<ProjectGroupDefault> {
    let blacklist: Vec<TypeId> = sqlx::query!("
            SELECT type_id
            FROM project_group_default_blacklist
            WHERE project_group_id = $1
        ",
            *group_uuid,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FetchGroupDefaults(e, group_uuid))?
        .into_iter()
        .map(|x| x.type_id.into())
        .collect::<Vec<_>>();

    let markets = sqlx::query!("
            SELECT structure_id
            FROM project_group_default_markets
            WHERE project_group_id = $1
        ",
            *group_uuid,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FetchGroupDefaults(e, group_uuid))?
        .into_iter()
        .map(|x| x.structure_id.into())
        .collect::<Vec<_>>();

    Ok(ProjectGroupDefault {
        blacklist,
        markets,
    })
}
