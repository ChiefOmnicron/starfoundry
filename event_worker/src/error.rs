use starfoundry_libs_types::{CharacterId, CorporationId, StationId};
use thiserror::Error;
use uuid::Uuid;

use crate::task::WorkerTask;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    // workers
    #[error("error registering worker in the database, '{0}'")]
    RegisterWorker(sqlx::Error),
    #[error("error deleting dead workers, '{0}'")]
    DeleteDeadWorker(sqlx::Error),
    #[error("error updating worker last seen, worker: '{1}' - '{0}'")]
    UpdateWorkerLastSeen(sqlx::Error, Uuid),
    #[error("error updating the tasks handled by a dead worker, '{0}'")]
    UpdateTaskFromDeadWorker(sqlx::Error),

    // task general
    #[error("error while fetching a new task, '{0}'")]
    FetchTask(sqlx::Error),
    #[error("failed to fetch check task {1:?}, error: {0}")]
    FetchCheck(sqlx::Error, WorkerTask),
    #[error("failed to insert check task {1:?}, error '{0}'")]
    InsertCheck(sqlx::Error, WorkerTask),
    #[error("error while parsing additional data for task, task: '{1}' - '{0}'")]
    ParseAdditionalData(serde_json::Error, Uuid),
    #[error("error while updating task, task: '{1}' - '{0}'")]
    UpdateTask(sqlx::Error, Uuid),
    #[error("unrecoverable error during task execution, check logs")]
    NoOp,
    #[error("no valid credentials were found for the id {0}")]
    NoCredentials(i32),
    #[error("error while requesting data from eve, {0}")]
    ApiError(starfoundry_libs_eve_api::Error),
    #[error("error creating transaction, '{0}'")]
    BeginTransaction(sqlx::Error),
    #[error("error committing transaction, '{0}'")]
    CommitTransaction(sqlx::Error),

    // assets
    #[error("error while inserting asset blueprints, error: '{0}'")]
    InsertAssetBlueprints(sqlx::Error),
    #[error("error while deleting asset blueprints, error: '{0}'")]
    DeleteAssetBlueprints(sqlx::Error),

    // cleanup
    #[error("error while deleting redundant industry index entries, '{0}'")]
    DeleteRedundantIndustryIndexEntries(sqlx::Error),
    #[error("error while deleting event queue entries, '{0}'")]
    DeleteRedundantEventQueue(sqlx::Error),
    #[error("error while deleting appraisals, '{0}'")]
    DeleteAppraisals(sqlx::Error),

    // indy tasks
    #[error("error fetching characterr ids, '{0}'")]
    FetchCharacterIds(sqlx::Error),
    #[error("error fetching characterr ids from queue, '{0}'")]
    FetchCharacterIdsQueue(sqlx::Error),
    #[error("error while inserting new jobs into the queue, '{0}'")]
    InsertNewJobs(sqlx::Error),
    #[error("error while fetching done job ids, '{0}'")]
    FetchDoneJobIds(sqlx::Error),
    #[error("error while fetching active jobs, '{0}'")]
    FetchActiveJobs(sqlx::Error),
    #[error("error while updating job for corporation '{0}'")]
    UpdateCorporationJobEntry(sqlx::Error),
    #[error("error while fetching main character by corporation '{1}', '{0}'")]
    FetchMainCharacterByCorporation(sqlx::Error, CorporationId),
    #[error("error while fetching main character by character '{1}', '{0}'")]
    FetchMainCharacterByCharacter(sqlx::Error, CharacterId),
    #[error("error getting bpc_stocks ids, '{0}'")]
    FetchBpcStockIds(sqlx::Error),
    #[error("error while inserting industy index, '{0}'")]
    InsertIndustryIndex(sqlx::Error),
    #[error("error while inserting industy jobs, '{0}'")]
    InsertIndustryJob(sqlx::Error),
    #[error("error while getting industry index systems '{0}'")]
    GetIndustryIndexSystems(sqlx::Error),
    #[error("error while getting industry index grouped by date, '{0}'")]
    GetIndustryIndexGroupedByDate(sqlx::Error),
    #[error("error while inserting industry index by date, '{0}'")]
    InsertIndustryIndexByDate(sqlx::Error),
    #[error("error while updating delivered jobs, error: '{0}'")]
    UpdateDeliveredJobs(sqlx::Error),
    #[error("error while updating already done jobs, error: '{0}'")]
    UpdateAlreadyDoneJobs(sqlx::Error),

    // market
    #[error("error while deleting latest orders for station '{1}', error: '{0}'")]
    DeleteLatestOrders(sqlx::Error, StationId),
    #[error("error while inserting latest orders for station '{1}', error: '{0}'")]
    InsertLatestOrders(sqlx::Error, StationId),
    #[error("error while fetching npc market queue, error: '{0}'")]
    FetchMarketNpcQueue(sqlx::Error),
    #[error("error while fetching player market queue, error: '{0}'")]
    FetchMarketPlayerQueue(sqlx::Error),
    #[error("error while fetching player markets, error: '{0}'")]
    FetchMarketstationsPlayer(sqlx::Error),
    #[error("error while inserting market prices, error: '{0}'")]
    InsertMarketPrices(sqlx::Error),

    #[error("sde error {0}")]
    SdeError(starfoundry_libs_eve_sde_parser::Error),
    #[error("general sqlx error: {0}")]
    GenericSqlxError(sqlx::Error),

    // stock
    #[error("error whilw fetching blueprint stocks, error: '{0}'")]
    FetchStockBlueprints(sqlx::Error),
    #[error("error while sending messages")]
    SendMessageError
}
