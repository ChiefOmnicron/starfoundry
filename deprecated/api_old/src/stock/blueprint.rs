use serde::Deserialize;
use serde::Serialize;
use starfoundry_lib_types::TypeId;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use utoipa::ToSchema;
use uuid::Uuid;

mod add_threshold;
mod create;
mod delete;
mod delete_threshold;
mod error;
mod fetch;
mod fetch_thresholds;
mod list;
mod update;
mod update_threshold;

pub use self::add_threshold::*;
pub use self::create::*;
pub use self::delete::*;
pub use self::delete_threshold::*;
pub use self::error::*;
pub use self::fetch::*;
pub use self::fetch_thresholds::*;
pub use self::list::*;
pub use self::update::*;
pub use self::update_threshold::*;

#[derive(Copy, Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(transparent)]
#[schema(
    example = json!(Uuid::new_v4()),
    value_type = Uuid,
)]
pub struct BlueprintStockUuid(Uuid);

impl BlueprintStockUuid {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl Deref for BlueprintStockUuid {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for BlueprintStockUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromStr for BlueprintStockUuid {
    type Err = uuid::Error;

    fn from_str(uuid_str: &str) -> Result<Self, Self::Err> {
        Ok(BlueprintStockUuid(Uuid::parse_str(uuid_str)?))
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct BlueprintStock {
    /// id of the stock
    #[serde(default)]
    pub id:            Option<BlueprintStockUuid>,
    pub name:          String,
    pub description:   String,
    pub notifications: Vec<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct BlueprintStockThreshold {
    /// id of the threshold
    #[serde(default)]
    pub id:       Option<BlueprintStockUuid>,
    pub type_id:  TypeId,
    /// how many blueprints are at least wanted
    pub want:     i32,
    /// below this point they definitly need to be stocked back up
    pub critical: i32,
    /// minimum number of runs the blueprint needs to have
    /// default: 0
    #[serde(default)]
    pub min_runs: i32,
    /// minimum material efficiency the blueprint needs to have
    /// default: 0
    #[serde(default)]
    pub min_me:   i32,
    /// minimum time efficiency the blueprint needs to have
    /// default: 0
    #[serde(default)]
    pub min_te:   i32,
}
