use sqlx::PgPool;

use starfoundry_libs_types::CharacterId;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::permission::ProjectGroupPermission;
use crate::project_group::ProjectGroupUuid;
use crate::project_group::list_members::ProjectGroupMember;

pub async fn fetch_members_self(
    pool:               &PgPool,
    character_id:       CharacterId,
    project_group_uuid: ProjectGroupUuid,
) -> Result<ProjectGroupMember> {
    sqlx::query!(
        "
            SELECT
                character_name,
                c.character_id,
                accepted,
                permission,
                (pg.owner = c.character_id) AS is_owner
            FROM project_group_member pgm
            JOIN project_group pg ON pg.id = pgm.group_id
            JOIN character c ON c.character_id = pgm.character_id
            WHERE c.character_id = $1
            AND group_id = $2
            ORDER BY character_name ASC
        ",
            *character_id,
            *project_group_uuid,
        )
        .fetch_one(pool)
        .await
        .map(|entry| {
            ProjectGroupMember {
                character_name: entry.character_name,
                character_id:   entry.character_id.into(),

                accepted:       entry.accepted,
                permissions:    ProjectGroupPermission::new(entry.permission),
                is_owner:       entry.is_owner.unwrap_or(false),
            }
        })
        .map_err(|e| ProjectGroupError::FetchGroupMembersSelf(e, project_group_uuid).into())
}

#[cfg(test)]
mod fetch_members_self_test {
    use sqlx::PgPool;
    use starfoundry_libs_types::CharacterId;
    use std::str::FromStr;
    use uuid::Uuid;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base", "list_default"),
        ),
        migrator = "crate::test_util::MIGRATOR",
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let response = super::fetch_members_self(
                &pool,
                CharacterId(1),
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
            )
            .await;

        assert!(response.is_ok());
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base", "list_default"),
        ),
        migrator = "crate::test_util::MIGRATOR",
    )]
    async fn default_if_entry_does_not_exist(
        pool: PgPool,
    ) {
        let response = super::fetch_members_self(
                &pool,
                CharacterId(1),
                Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap().into(),
            )
            .await;

        assert!(response.is_err());
    }
}
