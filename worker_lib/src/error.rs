use thiserror::Error;
use uuid::Uuid;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error registering worker in the database, '{0}'")]
    RegisterWorker(sqlx::Error),
    #[error("error deleting dead workers, '{0}'")]
    DeleteDeadWorker(sqlx::Error),
    #[error("error updating worker last seen, worker: '{1}' - '{0}'")]
    UpdateWorkerLastSeen(sqlx::Error, Uuid),
    #[error("error updating the tasks handled by a dead worker, '{0}'")]
    UpdateTaskFromDeadWorker(sqlx::Error),

    #[error("error fetching task, '{0}'")]
    FetchTask(sqlx::Error),
    #[error("error inserting task, '{0}'")]
    InsertTask(sqlx::Error),
    #[error("error updating task: '{1}', '{0}'")]
    UpdateTask(sqlx::Error, Uuid),

    #[error("error while parsing additional data: {0}")]
    ParseAdditionalData(serde_json::Error),

    #[error("error while cleaning up old tasks: '{0}'")]
    CleanupOldTasks(sqlx::Error),
}
