use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::{Result, StructureDynamicGroupUuid, StructureDynamicGroup};

pub struct StructureDynamicGroupService(StructureDynamicGroupUuid);

impl StructureDynamicGroupService {
    pub fn new(
        dynamic_group_uuid: StructureDynamicGroupUuid,
    ) -> Self {
        StructureDynamicGroupService(dynamic_group_uuid)
    }

    pub async fn list(
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<Vec<StructureDynamicGroupUuid>> {
        crate::dynamic_group::list(
                pool,
                character_id,
            )
            .await
    }

    pub async fn fetch(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<Option<StructureDynamicGroup>> {
        crate::dynamic_group::fetch(
                pool,
                character_id,
                self.0,
            )
            .await
    }

    pub async fn create(
        pool:         &PgPool,
        character_id: CharacterId,
        structure:    StructureDynamicGroup,
    ) -> Result<StructureDynamicGroupUuid> {
        crate::dynamic_group::create(
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
    ) -> Result<StructureDynamicGroupUuid> {
        crate::dynamic_group::delete(
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
        structure:    StructureDynamicGroup,
    ) -> Result<()> {
        crate::dynamic_group::update(
                pool,
                character_id,
                self.0,
                structure,
            )
            .await
    }
}
