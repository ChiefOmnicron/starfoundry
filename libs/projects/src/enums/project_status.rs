use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Different states of the project
/// 
/// A newly created project will always be in the status `Preparing`.
/// When the projects switches into `InProgress` the job detection gets active
/// for that project.
/// Afterwards the project is either `Done` or `Closed`. Job detection then gets
/// deaktivated again.
/// 
#[derive(
    Clone, Debug, Copy, Hash,
    PartialEq, Eq, PartialOrd, Ord,
    sqlx::Type, Deserialize, Serialize, ToSchema,
)]
#[sqlx(type_name = "PROJECT_STATUS")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProjectStatus {
    /// the project has not started yet, but materials are gathered
    /// job detection not active
    Preparing,
    /// the project is currently in progress, and job detection is active
    InProgress,
    /// the project is currently paused, job detection not active
    Paused,
    /// the project is finished, industry job detection is no longer active
    Done,
}
