use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;

use crate::{Error, ProjectFilter, ProjectUuid, Result};

pub async fn list(
    pool:         &PgPool,
    character_id: CharacterId,
    filter:       ProjectFilter,
) -> Result<Vec<ProjectUuid>> {
    let character_groups = if let Some(x) = filter.project_group {
        vec![*x]
    } else {
        crate::group::list(pool, character_id, Default::default())
            .await?
            .into_iter()
            .map(|x| *x)
            .collect::<Vec<_>>()
    };

    let filter_status: Vec<String> = if let Some(x) = filter.status.clone() {
        x
            .split(",")
            .map(|x| x.into())
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let filter_name = if let Some(x) = filter.name.clone() {
        x
    } else {
        String::new()
    };

    let ids = sqlx::query!(r#"
            SELECT id
            FROM project
            WHERE
                (
                    NOT (LOWER(name) LIKE '%' || LOWER($2) || '%') IS FALSE AND
                    NOT (status = ANY($3::PROJECT_STATUS[])) IS FALSE
                )
                AND
                (
                    -- check if the character is in the group
                    (
                        NOT (project_group_id = ANY($4::UUID[])) IS FALSE
                    )
                    OR
                    -- if the group is a uuid::default, make sure that the owner is the current character
                    (
                        project_group_id = '00000000-0000-0000-0000-000000000000' AND
                        owner = $1
                    )
                    OR
                    -- as a fallback check if the character is the owner
                    (
                        owner = $1
                    )
                )
            ORDER BY name
        "#,
            *character_id,
            &filter_name,
            &filter_status as _,
            &character_groups as _,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::ListProjectIds(e, character_id, filter))?
        .into_iter()
        .map(|x| ProjectUuid::new(x.id))
        .collect::<Vec<_>>();
    Ok(ids)
}

