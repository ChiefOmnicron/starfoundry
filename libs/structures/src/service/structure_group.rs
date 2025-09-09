use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::{CreateGroup, Result, StructureGroup, StructureGroupUuid};

pub struct StructureGroupService(StructureGroupUuid);

impl StructureGroupService {
    pub fn new(
        group_uuid: StructureGroupUuid,
    ) -> Self {
        StructureGroupService(group_uuid)
    }

    pub async fn list(
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<Vec<StructureGroupUuid>> {
        crate::group::list(
                pool,
                character_id,
            )
            .await
    }

    pub async fn fetch(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<StructureGroup> {
        crate::group::fetch(
                pool,
                character_id,
                self.0,
            )
            .await
    }

    pub async fn create(
        pool:         &PgPool,
        character_id: CharacterId,
        structure:    CreateGroup,
    ) -> Result<StructureGroupUuid> {
        crate::group::create(
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
    ) -> Result<StructureGroupUuid> {
        crate::group::delete(
                pool,
                character_id,
                self.0,
            )
            .await
    }
}
