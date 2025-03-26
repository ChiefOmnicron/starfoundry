use serde::Serialize;
use utoipa::ToSchema;

#[derive(
    Clone, Debug, Copy, Hash,
    PartialEq, Eq, PartialOrd, Ord,
    sqlx::Type, Serialize, ToSchema
)]
#[sqlx(type_name = "PROJECT_STATUS")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IndustryActivity {
    Manufacturing,
    TimeEfficiencyResearch,
    MaterialEfficiencyResearch,
    Copying,
    Invention,
    Reactions,
    Unknown,
}
