use chrono::{Days, NaiveDateTime, NaiveTime, Timelike, Utc};
use serde::de::DeserializeOwned;
use serde::Serialize;
use sqlx::{PgPool, Postgres};
use std::time::{Duration, Instant};
use uuid::Uuid;

use crate::{Error, Result, TaskMetric};
use crate::metric::InternalMetric;

/// Fetches any available task, respects other workers that already are working
/// on the tasks
/// https://stackoverflow.com/questions/6507475/job-queue-as-sql-table-with-multiple-consumers-postgresql
pub async fn fetch_task<M, WT>(
    pool:      &PgPool,
    worker_id: &Uuid,

    metric:          M,
    internal_metric: InternalMetric,
) -> Result<Option<Task<M, WT>>>
    where
        M: TaskMetric,
        WT: WorkerTask,
        WT: std::fmt::Debug + sqlx::Type<Postgres> + Into<String>,
        <WT as TryFrom<String>>::Error: std::fmt::Debug {

    sqlx::query!(r#"
            UPDATE worker_queue
            SET
                worker_id = $1,
                status = 'IN_PROGRESS',
                started_at = NOW()
            WHERE id = (
                SELECT id
                FROM worker_queue
                WHERE worker_id IS NULL
                  AND status = 'WAITING'
                  AND process_after < NOW()
                ORDER BY is_subtask ASC
                LIMIT 1
                FOR UPDATE SKIP LOCKED
            )
            RETURNING
                task,
                additional_data,
                id
        "#,
            worker_id,
        )
        .fetch_optional(pool)
        .await
        .map(|x| {
            x.map(|x| Task {
                task:            WT::try_from(x.task).unwrap(),
                id:              x.id,
                additional_data: x.additional_data,
                error:           None,
                logs:            None,

                metric:          metric,
                internal_metric: internal_metric,
                start:           Instant::now(),
            })
        })
        .map_err(Error::FetchTask)
}

pub trait WorkerTask: Clone + std::fmt::Debug + sqlx::Type<Postgres> + TryFrom<String> + Into<String> {
    /// Waits until the returned time is reached
    /// 
    /// Returning [Option::None] will prevent the task from being queued again
    /// 
    fn wait_until(
        &self,
    ) -> Option<NaiveDateTime>;

    /// Adds the given amount of minutes until the next task execution
    /// If a task is within 11:00 and 11:29, it will always be set to 11:30
    /// 
    fn add_minutes(
        &self,
        minutes: u64,
    ) -> Option<NaiveDateTime> {
        let now = Utc::now();
        // take the given minutes time 60 seconds, and add 30 seconds as an additional
        // wiggle room to prevent data that isn't yet expired
        let date = now.naive_utc() + Duration::from_secs(minutes * 60 + 30);

        // if it's between 11:00 and 11:30 delay it, otherwise add the minutes
        let date = if date.time().hour() == 11 && date.time().minute() < 30 {
            let minutes = (30 - date.time().minute()) as u64;
            date + Duration::from_secs(minutes * 60)
        } else {
            date
        };

        Some(date)
    }

    /// Tasks after downtime will run at 11:30, with the downtime being at 11:00
    /// 
    fn after_downtime(
        &self,
    ) -> Option<NaiveDateTime> {
        let now = Utc::now();

        let day = if now.hour() < 11 {
            now.date_naive()
        } else {
            let today = now.date_naive();
            today.checked_add_days(Days::new(1)).unwrap()
        };

        let time = NaiveTime::from_hms_opt(11, 30, 0).unwrap();
        Some(NaiveDateTime::new(day, time))
    }

    /// These tasks will run at 11:00
    /// 
    fn during_downtime(
        &self,
    ) -> Option<NaiveDateTime> {
        let now = Utc::now();

        let day = if now.hour() < 11 {
            now.date_naive()
        } else {
            let today = now.date_naive();
            today.checked_add_days(Days::new(1)).unwrap()
        };

        let time = NaiveTime::from_hms_opt(11, 0, 0).unwrap();
        Some(NaiveDateTime::new(day, time))
    }

    /// Helper function for oneshot tasks
    /// 
    fn oneshot(
        &self
    ) -> Option<NaiveDateTime> {
        None
    }
}

#[derive(Clone, Debug, sqlx::Type)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "WORKER_TASK_STATUS")]
pub enum TaskStatus {
    Waiting,
    InProgress,
    Done,
    Error,
    Timeout,
}

impl Into<String> for TaskStatus {
    fn into(self) -> String {
        match self {
            Self::Done       => "DONE",
            Self::Error      => "ERROR",
            Self::InProgress => "IN_PROGRESS",
            Self::Timeout    => "TIMEOUT",
            Self::Waiting    => "WAITING",
        }
        .into()
    }
}

#[derive(Clone)]
pub struct Task<M, WT>
    where
        M: TaskMetric,
        WT: WorkerTask {

    pub task:            WT,
    pub id:              Uuid,
    pub additional_data: Option<serde_json::Value>,
    pub error:           Option<String>,
    pub logs:            Option<String>,

    pub metric:          M,
    internal_metric:     InternalMetric,
    start:               Instant,
}

impl<M, WT> Task<M, WT>
    where
        WT: WorkerTask,
        M: TaskMetric {

    /// Fetches the additional data as a json from the given type.
    /// The given type must implement [std::fmt::Debug] and [serde::de::DeserializeOwned]
    /// 
    pub fn additional_data<T>(
        &self,
    ) -> Result<Option<T>>
        where T: std::fmt::Debug + DeserializeOwned
    {
        if let Some(x) = self.additional_data.clone() {
            serde_json::from_value::<T>(x)
                .map(|x| Some(x))
                .map_err(Error::ParseAdditionalData)
        } else {
            Ok(None)
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
    pub fn append_error<S: Into<String>>(
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
    pub fn append_log<S: Into<String>>(
        &mut self,
        log: S
    ) {
        if let Some(x) = self.logs.as_mut() {
            x.push_str(&format!("{}\n", log.into()));
        } else {
            self.logs = Some(format!("{}\n", log.into()));
        }
    }

    pub async fn add_subtask<T>(
        &self,
        pool:            &PgPool,
        task:            WT,
        additional_data: Option<T>,
    ) -> Result<()>
        where T: std::fmt::Debug + Serialize {

        let additional_data = serde_json::to_value(additional_data)
            .map_err(Error::ParseAdditionalData)?;
        sqlx::query!("
                INSERT INTO worker_queue (
                    task,
                    additional_data,
                    process_after,
                    is_subtask
                )
                VALUES ($1, $2, NOW(), true)
            ",
                task.into(),
                additional_data,
            )
            .execute(pool)
            .await
            .map(drop)
            .map_err(Error::InsertTask)?;

        Ok(())
    }
    pub async fn add_subtask_bulk<T>(
        &self,
        pool:            &PgPool,
        task:            WT,
        additional_data: Vec<Option<T>>,
    ) -> Result<()>
        where T: std::fmt::Debug + Serialize {

        let additional_data = additional_data
            .into_iter()
            .map(|x| serde_json::to_value(x))
            .map(|x| {
                if let Ok(x) = x {
                    Some(x)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        sqlx::query!("
                INSERT INTO worker_queue (
                    process_after,
                    is_subtask,
                    task,
                    additional_data
                )
                SELECT NOW(), true, $1, * FROM UNNEST(
                    $2::JSONB[]
                )
            ",
                task.into(),
                &additional_data as _,
            )
            .execute(pool)
            .await
            .map(drop)
            .map_err(Error::InsertTask)?;

        Ok(())
    }

    /// Finishes the task and sets the logs and error logs.
    /// Additionally it will create a new task.
    /// 
    pub async fn finish(
        self,
        pool:   &PgPool,
        status: TaskStatus
    ) -> Result<()> {
        let task_name = self.task.clone().into();
        self
            .internal_metric
            .add_task_duration(
                task_name.clone(),
                self.start.elapsed().as_secs_f64(),
            );
        self
            .internal_metric
            .increase_task_counter(
                task_name.clone(),
                status.clone(),
            );

        // update the current task to be done
        sqlx::query!("
                UPDATE worker_queue
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

        // insert a new task if it has a next time
        if let Some(task_wait_until) = self.task.wait_until() {
            sqlx::query!("
                    INSERT INTO worker_queue (
                        task,
                        process_after,
                        additional_data
                    )
                    VALUES ($1, $2, $3)
                ",
                    self.task.into(),
                    task_wait_until,
                    self.additional_data,
                )
                .execute(pool)
                .await
                .map(drop)
                .map_err(Error::InsertTask)
        } else {
            Ok(())
        }
    }
}
