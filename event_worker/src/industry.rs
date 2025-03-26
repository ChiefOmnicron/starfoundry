use chrono::{DateTime, Utc};
use starfoundry_libs_projects::ProjectJobStatus;
use starfoundry_libs_types::{CharacterId, JobId, TypeId};
use uuid::Uuid;

pub mod industry_index;
pub mod job_check;
pub mod job_character;
pub mod job_corporation;

mod job_detection;
mod utils;

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "PROJECT_STATUS")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobDetectionChange {
    /// the job was started
    Building,
    /// a job is now done
    Done,
}

#[derive(Clone, Debug)]
struct StartableIndustryJobs {
    pub project_name: String,
    pub project_id:   Uuid,
    pub id:           Uuid,
    pub type_id:      TypeId,
    pub runs:         i32,
    pub status:       ProjectJobStatus,
    /// JobId from CCP
    pub job_id:       Option<JobId>,
    pub created_at:   DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct UpdateJobRequest {
    pub id:             Uuid,
    pub character_id:   Option<CharacterId>,
    pub project_id:     Option<Uuid>,
    pub type_id:        TypeId,
    pub status:         ProjectJobStatus,
    pub cost:           Option<f32>,
    pub job_id:         Option<i32>,
}
