use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error fetching station '{1}', error: '{0}'")]
    FetchStation(sqlx::Error, Uuid),
    #[error("error inserting station '{1}', error: '{0}'")]
    InsertStation(sqlx::Error, Uuid),
    #[error("error inserting default user, error: '{0}'")]
    InsertDefaultUser(sqlx::Error),
    #[error("error inserting default project group, error: '{0}'")]
    InsertDefaultProjectGroup(sqlx::Error),
    #[error("error inserting default project group member, error: '{0}'")]
    InsertDefaultProjectGroupMember(sqlx::Error),
}
