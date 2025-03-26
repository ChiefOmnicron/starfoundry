use serde::{Deserialize, Serialize};

/// Determines in what security status the system is located in
/// 
#[derive(Clone, Debug, Deserialize, Serialize, sqlx::Type, utoipa::ToSchema)]
#[sqlx(type_name = "SYSTEM_SECURITY")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Security {
    Highsec,
    Lowsec,
    Nullsec,
}
