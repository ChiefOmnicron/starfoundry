use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;

use crate::{Error, ProjectGroupFilter, ProjectGroupUuid, Result};

pub async fn list(
    pool:         &PgPool,
    character_id: CharacterId,
    filter:       ProjectGroupFilter,
) -> Result<Vec<ProjectGroupUuid>> {
    let filter_projects: Vec<String> = if filter.projects.is_empty() {
        Vec::new()
    } else {
        filter.projects
            .split(",")
            .map(|x| x.into())
            .collect::<Vec<_>>()
    };

    let filter_structures: Vec<String> = if filter.structures.is_empty() {
        Vec::new()
    } else {
        filter.structures
            .split(",")
            .map(|x| x.into())
            .collect::<Vec<_>>()
    };

    sqlx::query!(
        "
            SELECT pg.id
            FROM project_groups pg
            JOIN project_group_members pgm ON pg.id = pgm.group_id
            WHERE pgm.character_id = $1 AND
                NOT (projects = ANY($2::VARCHAR[])) IS FALSE AND
                NOT (structures = ANY($3::VARCHAR[])) IS FALSE AND
                accepted = TRUE
            ORDER BY pg.name ASC
        ",
            *character_id,
            &filter_projects,
            &filter_structures,
        )
        .fetch_all(pool)
        .await
        .map(|entries| {
            entries
                .into_iter()
                .map(|x| ProjectGroupUuid::new(x.id))
                .collect::<Vec<_>>()
        })
        .map_err(Error::ListGroups)
}
