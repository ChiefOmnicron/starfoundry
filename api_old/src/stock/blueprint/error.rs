use thiserror::Error;

use super::BlueprintStockUuid;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BlueprintStockError {
    #[error("error fetching bpc stock info for '{1}', error: '{0}'")]
    FetchById(sqlx::Error, BlueprintStockUuid),
    #[error("error fetching bpc stock notifications for '{1}', error: '{0}'")]
    FetchNotifications(sqlx::Error, BlueprintStockUuid),
    #[error("error fetching bpc stock thresholds for '{1}', error: '{0}'")]
    FetchThresholds(sqlx::Error, BlueprintStockUuid),
    #[error("error creating new blueprint stock, error: '{0}'")]
    CreateNewStock(sqlx::Error),
    #[error("error updating blueprint stock for '{1}', error: '{0}'")]
    UpdateStock(sqlx::Error, BlueprintStockUuid),
    #[error("error deleting blueprint stock for '{1}', error: '{0}'")]
    DeleteStock(sqlx::Error, BlueprintStockUuid),
    #[error("error loading blueprint stocks, error: '{0}'")]
    ListStocks(sqlx::Error),

    #[error("error adding notification for '{1}', error: '{0}'")]
    AddNotification(sqlx::Error, BlueprintStockUuid),
    #[error("error deleting notification for '{1}', error: '{0}'")]
    DeleteNotification(sqlx::Error, BlueprintStockUuid),
    #[error("error updating notification for '{1}', error: '{0}'")]
    UpdateNotification(sqlx::Error, BlueprintStockUuid),

    #[error("error adding threshold for '{1}', error: '{0}'")]
    AddThreshold(sqlx::Error, BlueprintStockUuid),
    #[error("error deleting threshold for '{1}', error: '{0}'")]
    DeleteThreshold(sqlx::Error, BlueprintStockUuid),
    #[error("error updating threshold for '{1}', error: '{0}'")]
    UpdateThreshold(sqlx::Error, BlueprintStockUuid),

    #[error("could not find blueprint stock '{0}'")]
    NotFound(BlueprintStockUuid),
    #[error("could not find notification for blueprint stock '{0}'")]
    NotificationNotFound(BlueprintStockUuid),
    #[error("could not find threshold for blueprint stock '{0}'")]
    ThresholdNotFound(BlueprintStockUuid),
}
