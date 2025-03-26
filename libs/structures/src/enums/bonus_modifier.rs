use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "BONUS_MODIFIER")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BonusModifier {
    ManufactureMaterial,
    ManufactureTime,

    ReactionMaterial,
    ReactionTime,
}
