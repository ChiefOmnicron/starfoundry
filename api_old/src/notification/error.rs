use starfoundry_libs_types::CharacterId;
use thiserror::Error;

use super::NotificationUuid;
use super::list::NotificationFilter;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum NotificationError {
    #[error("error creating new notification, error: '{0}'")]
    Create(sqlx::Error),

    #[error("error fetching notification for id '{1}', error: '{0}'")]
    Fetch(sqlx::Error, NotificationUuid),

    #[error("error listing notification for character '{1}', with filter '{2:?}', error: '{0}'")]
    List(sqlx::Error, CharacterId, NotificationFilter),

    #[error("error deleting notification for id '{1}', error: '{0}'")]
    Delete(sqlx::Error, NotificationUuid),

    #[error("error updating notification for id '{1}', error: '{0}'")]
    Update(sqlx::Error, NotificationUuid),

    #[error("notification not found '{0}'")]
    NotFound(NotificationUuid),
}
