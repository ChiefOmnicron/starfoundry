use sqlx::PgPool;
use starfoundry_lib_eve_api::EveApiClient;
use starfoundry_lib_types::{CharacterId, StructureId, TypeId};

use crate::{CreateStructure, Error, ResolvedStructure, Result, Structure, StructureDatabase, StructureRig, StructureUuid, UpdateStructure};

pub struct StructureService(StructureUuid);

impl StructureService {
    pub fn new(
        structure_uuid: StructureUuid,
    ) -> Self {
        StructureService(structure_uuid)
    }

    pub async fn assert_owner(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<()> {
        let result = sqlx::query!("
                SELECT id
                FROM structure
                WHERE id = $1
                AND owner = $2
            ",
                *self.0,
                *character_id,
            )
            .fetch_optional(pool)
            .await
            .map_err(|e| Error::FetchPermissions(e, self.0))?;

        if result.is_none() {
            return Err(Error::Forbidden(*self.0, character_id));
        } else {
            Ok(())
        }
    }

    // TODO: wrong query!
    // TODO: remove hard coded values
    pub async fn assert_read_access(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<()> {
        let result = sqlx::query!("
                SELECT p.id
                FROM project p
                JOIN project_group_member pgm ON pgm.group_id = project_group_id
                WHERE p.id = $1
                AND pgm.character_id = $2
                AND (
                    permission & 1 = 1 OR
                    permission & 2 = 2
                )
            ",
                *self.0,
                *character_id,
            )
            .fetch_optional(pool)
            .await
            .map_err(|e| Error::FetchPermissions(e, self.0))?;

        if result.is_none() {
            return Err(Error::Forbidden(*self.0, character_id));
        } else {
            Ok(())
        }
    }

    // TODO: wrong query!
    pub async fn assert_write_access(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<()> {
        let result = sqlx::query!("
                SELECT p.id
                FROM project p
                JOIN project_group_member pgm ON pgm.group_id = project_group_id
                WHERE p.id = $1
                AND pgm.character_id = $2
                AND (
                    -- TODO: replace with enum values
                    permission & 1 = 1 OR
                    permission & 8 = 8
                )
            ",
                *self.0,
                *character_id,
            )
            .fetch_optional(pool)
            .await
            .map_err(|e| Error::FetchPermissions(e, self.0))?;

        if result.is_none() {
            return Err(Error::Forbidden(*self.0, character_id));
        } else {
            Ok(())
        }
    }

    pub async fn assert_exists(
        &self,
        pool: &PgPool,
    ) -> Result<()> {
        let project = sqlx::query!("
                SELECT id
                FROM structure
                WHERE id = $1
            ",
                *self.0,
            )
            .fetch_optional(pool)
            .await
            .map_err(|e| Error::FetchStructure(e, self.0))?;

        if project.is_some() {
            Ok(())
        } else {
            Err(Error::StructureNotFound(self.0))
        }
    }

    pub async fn list(
        pool:         &PgPool,
        character_id: CharacterId,
        filter:       crate::root::StructureListFilter,
    ) -> Result<Vec<StructureDatabase>> {
        crate::root::list(
                pool,
                character_id,
                filter
            )
            .await
    }

    pub async fn fetch(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<Option<StructureDatabase>> {
        crate::root::fetch(
                pool,
                character_id,
                self.0,
            )
            .await
    }

    pub async fn danger_no_permission_fetch(
        &self,
        pool:         &PgPool,
    ) -> Result<Option<Structure>> {
        crate::root::danger_no_permission_fetch(
                pool,
                self.0,
            )
            .await
    }

    pub async fn create(
        pool:         &PgPool,
        character_id: CharacterId,
        structure:    CreateStructure,
    ) -> Result<StructureUuid> {
        crate::root::create(
                pool,
                character_id,
                structure,
            )
            .await
    }

    pub async fn delete(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<()> {
        self.assert_exists(pool).await?;
        self.assert_owner(pool, character_id).await?;

        crate::root::delete(
                pool,
                character_id,
                self.0,
            )
            .await
    }

    pub async fn update(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
        structure:    UpdateStructure,
    ) -> Result<StructureUuid> {
        self.assert_exists(pool).await?;
        self.assert_owner(pool, character_id).await?;

        crate::root::update(
                pool,
                character_id,
                self.0,
                structure,
            )
            .await
    }

    pub async fn resolve_player_structure(
        pool:         &PgPool,
        eve_client:   EveApiClient,
        structure_id: StructureId,
    ) -> Result<ResolvedStructure> {
        crate::root::resolve_player_structure(
                pool,
                eve_client,
                structure_id,
            )
            .await
    }

    pub async fn rig_by_structure_type_id(
        pool:         &PgPool,
        structure_id: TypeId,
    ) -> Result<Vec<StructureRig>> {
        crate::root::rig_by_structure_type_id(
                pool,
                structure_id,
            )
            .await
    }
}
