use starfoundry_lib_types::{CharacterId, StructureId, TypeId};
use thiserror::Error;
use uuid::Uuid;

use crate::{StructureDynamicGroup, StructureDynamicGroupUuid, StructureGroupUuid, StructureListFilter, StructureUuid};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("the character '{1}' is not allowed to accedd '{0}'")]
    Forbidden(Uuid, CharacterId),
    #[error("error while fetching permissions for project '{1}', error: '{0}'")]
    FetchPermissions(sqlx::Error, StructureUuid),
    #[error("structure could not be found: '{0}'")]
    StructureNotFound(StructureUuid),

    #[error("error creating new strucutre, error: '{0}'")]
    Create(sqlx::Error),

    #[error("error listing structure ids from owner '{1}', filter: {2:?}, error: {0}")]
    ListStructureIds(sqlx::Error, CharacterId, StructureListFilter),

    #[error("error fetching structure '{1}', error: {0}")]
    FetchStructure(sqlx::Error, StructureUuid),

    #[error("error deleting structure by id '{1}', error: {0}")]
    DeleteStructure(sqlx::Error, StructureUuid),

    #[error("error updating structure by id '{1}', error: {0}")]
    UpdateStructure(sqlx::Error, StructureUuid),

    #[error("error fetching structure rigs for TypeId '{1}', error: {0}")]
    FetchRigsByStructureTypeId(sqlx::Error, TypeId),
    #[error("error fetching rigs bonuses for TypeId '{1}', error: {0}")]
    FetchRigBonusByTypeId(sqlx::Error, TypeId),
    #[error("error fetching rigs name for TypeId '{1}', error: {0}")]
    FetchRigNameByTypeId(sqlx::Error, TypeId),

    #[error("error fetching structure permissions for '{1}' and character {2}, error: {0}")]
    FetchStructurePermission(sqlx::Error, StructureUuid, CharacterId),

    #[error("error fetching player structure from eve '{1}', error: {0}")]
    FetchPlayerStructureFromEve(starfoundry_lib_eve_api::Error, StructureId),
    #[error("error fetching player structure system security '{1}', error: {0}")]
    FetchPlayerStructureSystemSecurity(sqlx::Error, StructureId),
    #[error("the strucutreId {0} is not a valid structureId")]
    InvalidStructureId(StructureId),

    // group
    #[error("error creating structure group, error: {0}")]
    CreateGroup(sqlx::Error),
    #[error("error fetching structure group by id '{1}', error: {0}")]
    FetchGroup(sqlx::Error, StructureGroupUuid),
    #[error("error deleting structure group by id '{1}', error: {0}")]
    DeleteGroup(sqlx::Error, StructureGroupUuid),
    #[error("error listing strucutre groups for character '{1}', error: {0}")]
    ListGroups(sqlx::Error, CharacterId),

    // dynamic group
    #[error("error fetching structures by owner id '{1}', error: {0}")]
    #[deprecated]
    FetchStructuresByOwner(sqlx::Error, CharacterId),
    #[error("error fetching structures by id '{1}', error: {0}")]
    #[deprecated]
    FetchStructuresById(sqlx::Error, Uuid),
    #[error("error listing dynamic structures ids by owner id '{1}', error: {0}")]
    ListGroupIds(sqlx::Error, CharacterId),
    #[error("error getting structure group by id '{1}', error: {0}")]
    GroupIdById(sqlx::Error, StructureDynamicGroupUuid),
    #[error("error creating group '{1}', error: {0}")]
    CreateDynamicGroup(sqlx::Error, StructureDynamicGroup),
    #[error("error updating group '{1}' with '{2}', error: {0}")]
    UpdateDynamicGroup(sqlx::Error, StructureDynamicGroupUuid, StructureDynamicGroup),
    #[error("error deleting group '{1}', error: {0}")]
    DeleteDynamicGroup(sqlx::Error, StructureDynamicGroupUuid),

    #[error("error while fetching structure project groups for strucutre '{1}, error: '{0}'")]
    FetchStructureProjectGroups(sqlx::Error, StructureUuid),
}
