use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{CharacterId, TypeId};
use uuid::Uuid;

/// Represents a ProjectId
#[derive(
    Clone, Copy, Debug, Hash,
    Deserialize, Serialize,
)]
#[serde(transparent)]
pub struct ProjectId(Uuid);

impl From<Uuid> for ProjectId {
    fn from(x: Uuid) -> Self {
        Self(x)
    }
}

impl std::ops::Deref for ProjectId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Represents a single project
#[derive(Debug, Serialize)]
pub struct Project {
    /// Unique identifier for the project
    pub project:     ProjectId,
    /// Every project belongs to exactly one person
    pub owner:       CharacterId,
    /// Name of the project
    pub name:        String,
    /// Status of the project
    pub status:      Status,
    /// All projects that should be produced in this project
    pub products:    Vec<Product>,
}

/// Represents a product that is build within the project
#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    /// Name of the product
    pub name:                String,
    /// Number of items that should be build
    pub quantity:            u32,
    /// Material efficiency
    pub material_efficiency: u32,
    /// TypeId of the product
    pub type_id:             TypeId,
}

/// Determines what status a project currently has
#[derive(Debug, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "PROJECT_STATUS")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    /// The project is aborted
    Aborted,
    /// The project is finished
    Done,
    /// The project is currently in progress
    InProgress,
    /// The project is currently paused
    Paused
}
