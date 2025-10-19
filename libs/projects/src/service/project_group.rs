use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::{CreateProjectGroup, Error, ProjectGroup, ProjectGroupMember, ProjectGroupPermission, ProjectGroupPermissionCode, ProjectGroupUuid, Result, UpdateProjectGroup};
use crate::group::{ProjectGroupDefault, ProjectGroupFilter};

#[deprecated]
pub struct ProjectGroupService(ProjectGroupUuid);

impl ProjectGroupService {
    pub fn new(
        project_uuid: ProjectGroupUuid,
    ) -> Self {
        ProjectGroupService(project_uuid)
    }

    #[deprecated]
    pub async fn assert_owner(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<()> {
        let result = sqlx::query!("
                SELECT id
                FROM project_group
                WHERE id = $1
                AND owner = $2
            ",
                *self.0,
                *character_id,
            )
            .fetch_optional(pool)
            .await
            .map_err(|e| Error::FetchGroupPermissions(e, self.0))?;

        if result.is_none() {
            return Err(Error::Forbidden(*self.0, character_id));
        } else {
            Ok(())
        }
    }

    pub async fn assert_read_access(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<()> {
        let result = sqlx::query!("
                SELECT pg.id
                FROM project_group pg
                JOIN project_group_member pgm ON pgm.group_id = pg.id
                WHERE pg.id = $1
                AND pgm.character_id = $2
                --AND (
                --    permission & $3 = $3 OR
                --    permission & $4 = $4
                --)
            ",
                *self.0,
                *character_id,
                //*ProjectGroupPermissionCode::Owner,
                //*ProjectGroupPermissionCode::ReadGroup,
            )
            .fetch_optional(pool)
            .await
            .map_err(|e| Error::FetchGroupPermissions(e, self.0))?;

        if result.is_none() {
            return Err(Error::Forbidden(*self.0, character_id));
        } else {
            Ok(())
        }
    }

    pub async fn assert_write_access(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
        permission:   ProjectGroupPermissionCode,
    ) -> Result<()> {
        let result = sqlx::query!("
                SELECT pg.id
                FROM project_group pg
                JOIN project_group_member pgm ON pgm.group_id = pg.id
                WHERE pg.id = $1
                AND pgm.character_id = $2
                --AND (
                --    permission & $3 = $3 OR
                --    permission & $4 = $4
                --)
            ",
                *self.0,
                *character_id,
                //*ProjectGroupPermissionCode::Owner,
                //*permission,
            )
            .fetch_optional(pool)
            .await
            .map_err(|e| Error::FetchGroupPermissions(e, self.0))?;

        if result.is_none() {
            return Err(Error::Forbidden(*self.0, character_id));
        } else {
            Ok(())
        }
    }

    pub async fn assert_exists(
        &self,
        pool: &PgPool,
    ) -> Result<Option<()>> {
        let project = sqlx::query!("
                SELECT id
                FROM project_group
                WHERE id = $1
            ",
                *self.0,
            )
            .fetch_optional(pool)
            .await
            .map_err(|e| Error::FetchGroup(e, self.0))?;

        if project.is_some() {
            Ok(Some(()))
        } else {
            Err(Error::ProjectGroupNotFound(self.0))
        }
    }

    pub async fn list(
        pool:         &PgPool,
        character_id: CharacterId,
        filter:       ProjectGroupFilter,
    ) -> Result<Vec<ProjectGroup>> {
        crate::group::list(
            pool,
            character_id,
            filter,
        )
        .await
    }

    pub async fn fetch(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<ProjectGroup> {
        self.assert_exists(pool).await?;
        self.assert_read_access(pool, character_id).await?;

        crate::group::fetch(
            pool,
            character_id,
            self.0,
        )
        .await
    }

    pub async fn accept_invite(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<()> {
        self.assert_exists(pool).await?;

        crate::group::accept_invite(
            pool,
            character_id,
            self.0,
        )
        .await
    }

    pub async fn accept_member(
        &self,
        pool:                   &PgPool,
        character_id:           CharacterId,
        requester_character_id: CharacterId,
    ) -> Result<()> {
        self.assert_exists(pool).await?;
        self.assert_write_access(
            pool,
            character_id,
            ProjectGroupPermissionCode::WriteMember,
        ).await?;

        crate::group::accept_member(
            pool,
            requester_character_id,
            self.0,
        )
        .await
    }

    pub async fn create(
        pool:         &PgPool,
        character_id: CharacterId,
        info:         CreateProjectGroup,
    ) -> Result<ProjectGroupUuid> {
        crate::group::create(
            pool,
            character_id,
            info,
        )
        .await
    }

    pub async fn delete(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<ProjectGroupUuid> {
        self.assert_exists(pool).await?;
        self.assert_write_access(
            pool,
            character_id,
            ProjectGroupPermissionCode::WriteGroup,
        ).await?;

        crate::group::delete(
            pool,
            self.0,
        )
        .await
    }

    pub async fn update(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
        info:         UpdateProjectGroup,
    ) -> Result<()> {
        self.assert_exists(pool).await?;
        self.assert_write_access(
            pool,
            character_id,
            ProjectGroupPermissionCode::WriteGroup,
        ).await?;

        crate::group::update(
            pool,
            self.0,
            info,
        )
        .await
    }

    pub async fn fetch_defaults(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<ProjectGroupDefault> {
        self.assert_exists(pool).await?;
        self.assert_read_access(pool, character_id).await?;

        crate::group::fetch_defaults(
            pool,
            self.0,
        )
        .await
    }

    pub async fn update_default(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
        defaults:     ProjectGroupDefault,
    ) -> Result<()> {
        self.assert_exists(pool).await?;
        self.assert_write_access(
            pool,
            character_id,
            ProjectGroupPermissionCode::WriteDefault,
        ).await?;

        crate::group::update_default(
            pool,
            self.0,
            defaults,
        )
        .await
    }

    pub async fn fetch_members(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<Vec<ProjectGroupMember>> {
        self.assert_exists(pool).await?;
        self.assert_read_access(pool, character_id).await?;

        crate::group::fetch_members(
            pool,
            self.0,
        )
        .await
    }

    pub async fn update_member_permission(
        &self,
        pool:                &PgPool,
        character_id:        CharacterId,
        member_character_id: CharacterId,
        permission:          ProjectGroupPermission,
    ) -> Result<()> {
        self.assert_exists(pool).await?;
        self.assert_write_access(
            pool,
            character_id,
            ProjectGroupPermissionCode::WriteMember,
        ).await?;

        crate::group::update_member_permission(
            pool,
            self.0,
            member_character_id,
            permission,
        )
        .await
    }

    pub async fn remove_member(
        &self,
        pool:                &PgPool,
        character_id:        CharacterId,
        member_character_id: CharacterId
    ) -> Result<()> {
        self.assert_exists(pool).await?;
        self.assert_write_access(
            pool,
            character_id,
            ProjectGroupPermissionCode::WriteMember,
        ).await?;

        crate::group::remove_member(
            pool,
            self.0,
            member_character_id,
        )
        .await
    }
}
