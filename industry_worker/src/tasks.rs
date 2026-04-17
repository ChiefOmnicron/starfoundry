use chrono::NaiveDateTime;
use prometheus_client::encoding::EncodeLabelValue;
use starfoundry_lib_worker::WorkerTask;
use crate::error::Error;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, EncodeLabelValue, sqlx::Type)]
#[sqlx(type_name = "WORKER_TASK")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkerIndustryTask {
    /// sync the markets with the structure in the industry tool
    Sync,

    JobCharacter,
    JobCorporation,
}

impl WorkerTask for WorkerIndustryTask {
    fn wait_until(
        &self,
    ) -> Option<NaiveDateTime> {
        match self {
            Self::Sync              => self.add_minutes(5),
            Self::JobCharacter      => self.add_minutes(5),
            Self::JobCorporation    => self.add_minutes(5),
        }
    }
}

impl TryFrom<String> for WorkerIndustryTask {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "SYNC"                  => Ok(Self::Sync),
            "JOB_CHARACTER"         => Ok(Self::JobCharacter),
            "JOB_CORPORATION"       => Ok(Self::JobCorporation),
            _                       => Err(Error::InvalidWorkerTask(value)),
        }
    }
}

impl Into<String> for WorkerIndustryTask {
    fn into(self) -> String {
        match self {
            Self::Sync              => "SYNC",
            Self::JobCharacter      => "JOB_CHARACTER",
            Self::JobCorporation    => "JOB_CORPORATION",
        }.into()
    }
}
