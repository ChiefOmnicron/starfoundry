use chrono::{Days, NaiveDateTime, NaiveTime, Timelike, Utc};
use serde::de::DeserializeOwned;
use sqlx::PgPool;
use std::time::Duration;
use uuid::Uuid;

use crate::error::{Error, Result};
use serde::Serialize;
use prometheus_client::encoding::EncodeLabelValue;

/// Fetches any available task, respects other workers that already are working
/// on the tasks
/// https://stackoverflow.com/questions/6507475/job-queue-as-sql-table-with-multiple-consumers-postgresql
pub async fn fetch_task(
    pool:      &PgPool,
    worker_id: &Uuid,
) -> Result<Option<Task>> {
    sqlx::query!(r#"
            UPDATE event_queue
            SET
                worker_id = $1,
                status = 'IN_PROGRESS',
                started_at = NOW()
            WHERE id = (
                SELECT id
                FROM event_queue
                WHERE worker_id IS NULL
                  AND status = 'WAITING'
                  AND process_after < NOW()
                LIMIT 1
                FOR UPDATE SKIP LOCKED
            )
            RETURNING
                task AS "task!: WorkerTask",
                additional_data,
                id
        "#,
            worker_id,
        )
        .fetch_optional(pool)
        .await
        .map(|x| {
            x.map(|x| Task {
                task:            x.task,
                id:              x.id,
                additional_data: x.additional_data,
                error:           None,
                logs:            None,
            })
        })
        .map_err(Error::FetchTask)
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, EncodeLabelValue, sqlx::Type)]
#[sqlx(type_name = "EVENT_WORKER_TASK")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkerTask {
    /// check if all asset tasks are in the queue
    AssetCheck,
    /// pulls the blueprints the character posses
    AssetCharacterBlueprints,
    /// pulls the blueprints the corporation posses
    AssetCorporationBlueprints,

    /// checks if all cleanup related events are in the queue
    /// CleanupIndustryIndex
    /// CleanupSelf
    CleanupCheck,
    /// cleanup appraisals
    CleanupAppraisals,
    /// cleanup event queue
    CleanupSelf,
    /// compresses the industry index table
    CleanupIndustryIndex,

    /// checks if all industry related events are in the queue
    /// IndustryJobsCharacter
    /// IndustryJobsCorporation
    IndustryCheck,
    /// pulls the indy jobs for a specific character and runs the job detection on it
    IndustryJobsCharacter,
    /// pulls the indy jobs for a specific coproration nd runs the job detection on it
    IndustryJobsCorporation,
    /// fetches the current industry index
    IndustryIndex,

    /// checks if all market related events are in the queue
    MarketCheck,
    /// fetches the latest NPC orders
    MarketLatestNpc,
    /// fetches the latest player orders
    MarketLatestPlayer,
    /// fetches the latest market prices
    MarketPrices,

    /// checks if all sde tasks are in the queue
    /// sde_download
    SdeCheck,
    /// downloads and imports the latest sde
    SdeDownload,

    /// checks if all industry related events are in the queue
    /// StockBlueprint,
    StockCheck,
    /// checks the bpc stock and sends out a warning if necessary
    StockBlueprint,
}

impl WorkerTask {
    /// Determines how long the task will be idle until it can be run again
    pub fn timeout(
        &self,
    ) -> NaiveDateTime {
        match self {
            Self::AssetCheck                 => self.add_minutes(5),
            Self::AssetCharacterBlueprints   => self.add_minutes(60),
            Self::AssetCorporationBlueprints => self.add_minutes(60),

            Self::CleanupCheck               => self.add_minutes(120),
            Self::CleanupAppraisals          => self.during_downtime(),
            Self::CleanupSelf                => self.during_downtime(),
            Self::CleanupIndustryIndex       => self.during_downtime(),

            Self::IndustryCheck              => self.add_minutes(5),
            Self::IndustryJobsCharacter      => self.add_minutes(5),
            Self::IndustryJobsCorporation    => self.add_minutes(5),
            Self::IndustryIndex              => self.add_minutes(60),

            Self::MarketCheck                => self.add_minutes(5),
            Self::MarketLatestNpc            => self.add_minutes(5),
            Self::MarketLatestPlayer         => self.add_minutes(5),
            Self::MarketPrices               => self.add_minutes(60),

            Self::SdeCheck                   => self.add_minutes(120),
            Self::SdeDownload                => self.during_downtime(),

            Self::StockCheck                 => self.add_minutes(5),
            Self::StockBlueprint             => self.after_downtime(),
        }
    }

    /// Adds the given amount of minutes until the next task execution
    /// If a task is within 11:00 and 11:29, it will always be set to 11:30
    /// 
    fn add_minutes(
        &self,
        minutes: u64,
    ) -> NaiveDateTime {
        let now = Utc::now();
        let date = now.naive_utc() + Duration::from_mins(minutes);

        // if it's between 11:00 and 11:30 delay it, otherwise add the minutes
        let date = if date.time().hour() == 11 && date.time().minute() < 30 {
            let minutes = (30 - date.time().minute()) as u64;
            date + Duration::from_mins(minutes)
        } else {
            date
        };

        date
    }

    /// Tasks after downtime will run at 11:30, with the downtime being at 11:00
    /// 
    fn after_downtime(
        &self,
    ) -> NaiveDateTime {
        let now = Utc::now();

        let day = if now.hour() < 11 {
            now.date_naive()
        } else {
            let today = now.date_naive();
            today.checked_add_days(Days::new(1)).unwrap()
        };

        let time = NaiveTime::from_hms_opt(11, 30, 0).unwrap();
        NaiveDateTime::new(day, time)
    }

    /// These tasks will run at 11:00
    /// 
    fn during_downtime(
        &self,
    ) -> NaiveDateTime {
        let now = Utc::now();

        let day = if now.hour() < 11 {
            now.date_naive()
        } else {
            let today = now.date_naive();
            today.checked_add_days(Days::new(1)).unwrap()
        };

        let time = NaiveTime::from_hms_opt(11, 0, 0).unwrap();
        NaiveDateTime::new(day, time)
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "EVENT_TASK_STATUS")]
pub enum TaskStatus {
    Waiting,
    InProgress,
    Done,
    Error,
}

#[derive(Debug)]
pub struct Task {
    pub task:            WorkerTask,
    pub id:              Uuid,
    pub additional_data: Option<serde_json::Value>,
    pub error:           Option<String>,
    pub logs:            Option<String>,
}

impl Task {
    /// Fetches the additional data as a json from the given type.
    /// The given type must implement [std::fmt::Debug] and [serde::de::DeserializeOwned]
    /// 
    pub fn additional_data<T>(
        &self,
    ) -> Option<T>
        where T: std::fmt::Debug + DeserializeOwned
    {
        if let Some(x) = self.additional_data.clone() {
            serde_json::from_value(x)
                .map_err(|e| Error::ParseAdditionalData(e, self.id))
                .ok()
        } else {
            None
        }
    }

    /// Sets the additional data for a task
    /// The given type must implement [std::fmt::Debug] and [serde::de::DeserializeOwned]
    /// 
    pub fn set_additional_data<T>(
        &mut self,
        additional_data: Option<T>,
    )
        where T: std::fmt::Debug + Serialize
    {
        self.additional_data = serde_json::to_value(additional_data).ok();
    }

    /// Adds an additional error line to the task error log
    /// 
    pub fn add_error<S: Into<String>>(
        &mut self,
        error: S
    ) {
        if let Some(x) = self.error.as_mut() {
            x.push_str(&format!("{}\n", error.into()));
        } else {
            self.error = Some(format!("{}\n", error.into()));
        }
    }

    /// Adds a standard log to the task
    /// 
    pub fn add_log<S: Into<String>>(
        &mut self,
        log: S
    ) {
        if let Some(x) = self.logs.as_mut() {
            x.push_str(&format!("{}\n", log.into()));
        } else {
            self.logs = Some(format!("{}\n", log.into()));
        }
    }

    /// Finishes the task and sets the logs and error logs.
    /// Additionally it will create a new task.
    /// 
    pub async fn finish(
        self,
        pool:   &PgPool,
        status: TaskStatus
    ) -> Result<()> {
        // update the current task to be done
        sqlx::query!("
                UPDATE event_queue
                SET
                    status = $2,
                    logs = $3,
                    error = $4,
                    finished_at = NOW()
                WHERE id = $1
            ",
                self.id,
                status as _,
                self.logs,
                self.error,
            )
            .execute(pool)
            .await
            .map_err(|e| Error::UpdateTask(e, self.id.clone()))
            .map(drop)?;

        // insert a new task
        sqlx::query!("
                INSERT INTO event_queue (
                    task,
                    process_after,
                    additional_data
                )
                VALUES ($1, $2, $3)
            ",
                self.task as _,
                self.task.timeout(),
                self.additional_data,
            )
            .execute(pool)
            .await
            .map(drop)
            .map_err(Error::InsertNewJobs)
    }
}

