use chrono::NaiveDateTime;
use prometheus_client::encoding::EncodeLabelValue;
use starfoundry_lib_worker::WorkerTask;

use crate::error::IndustryWorkerError;

#[derive(
    Clone, Copy, Debug, Hash, PartialEq, Eq, EncodeLabelValue, sqlx::Type,
)]
#[sqlx(type_name = "WORKER_TASK")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkerIndustryTask {
    /// sync the markets with the structure in the industry tool
    Sync,

    JobCharacter,
    JobCorporation,
}

impl WorkerTask for WorkerIndustryTask {
    fn wait_until(&self) -> Option<NaiveDateTime> {
        match self {
            Self::Sync => self.add_minutes(5),
            Self::JobCharacter => self.add_minutes(5),
            Self::JobCorporation => self.add_minutes(5),
        }
    }
}

impl TryFrom<String> for WorkerIndustryTask {
    type Error = IndustryWorkerError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "SYNC" => Ok(Self::Sync),
            "JOB_CHARACTER" => Ok(Self::JobCharacter),
            "JOB_CORPORATION" => Ok(Self::JobCorporation),
            _ => Err(IndustryWorkerError::InvalidWorkerTask(value)),
        }
    }
}

impl From<WorkerIndustryTask> for String {
    fn from(value: WorkerIndustryTask) -> String {
        match value {
            WorkerIndustryTask::Sync => "SYNC",
            WorkerIndustryTask::JobCharacter => "JOB_CHARACTER",
            WorkerIndustryTask::JobCorporation => "JOB_CORPORATION",
        }
        .into()
    }
}
