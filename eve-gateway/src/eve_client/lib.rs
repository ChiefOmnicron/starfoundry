//! Exposes the EVE-API and EVE-SDE as a single library without making a
//! difference between those two.
//!
//! For EVE-API-Authentication an EVE-Auth-Client is exposed.

#![forbid(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
)]
#![warn(
    clippy::await_holding_lock,
    clippy::get_unwrap,
    clippy::map_unwrap_or,
    clippy::unwrap_in_result,
    clippy::unwrap_used
)]
#![allow(clippy::redundant_field_names)]

/// Groups all requests together that are in the industry group
mod industry;

/// Module for handling characters
mod character;
/// Module containing clients to the EVE-API
mod client;
/// Module for handling corporations
mod corporation;
/// Cache that holds all characters and their keys
mod credential_cache;
/// Module containing possible errors
mod error;
/// Module for handling market operations
mod market;
/// Model for the OAuthToken send by eve
mod oauth_token;
/// Module for all universe api requests
mod universe;
/// Module for all wallet api requests
mod wallet;

pub use self::character::*;
pub use self::client::*;
pub use self::corporation::*;
pub use self::credential_cache::*;
pub use self::error::*;
pub use self::market::*;
pub use self::oauth_token::*;
pub use self::universe::*;

use serde::{Deserialize, Deserializer, Serialize};
use starfoundry_lib_types::{CharacterId, ItemId, LocationId, TypeId, CorporationId, JobId};

/// Represents a single character blueprint
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlueprintEntry {
    /// Unique ID of the asset
    pub item_id: ItemId,
    /// Id of the location the asset is located in
    pub location_id: LocationId,
    /// Material efficiency of the blueprint, max 10
    pub material_efficiency: i32,
    /// Time efficiency of the blueprint, max 20
    pub time_efficiency: i32,
    /// A range of numbers with a minimum of -2 and no maximum value where -1
    /// is an original and -2 is a copy. It can be a positive integer if it is
    /// a stack of blueprint originals fresh from the market (e.g. no
    /// activities performed on them yet).
    pub quantity: i32,
    /// Number of runs remaining if the blueprint is a copy, -1 if it is an original
    pub runs: i32,
    /// Type id of the asset
    pub type_id: TypeId,
}

/// Represents a transaction entry
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndustryJobEntry {
    /// Activity of the job
    #[serde(deserialize_with = "IndustryActivity::from")]
    #[serde(rename = "activity_id")]
    pub activity: IndustryActivity,
    /// Asset ID of the blueprint
    pub blueprint_id: ItemId,
    /// ID of the blueprint
    pub blueprint_type_id: TypeId,
    /// ID of the blueprint
    pub product_type_id: TypeId,
    /// Station where the blueprint is located, can also be containers
    pub blueprint_location_id: LocationId,
    /// When the job is doen
    pub end_date: String,
    /// Cost of the job
    pub cost: Option<f32>,
    /// Number of runs
    pub runs: i32,
    /// Number of runs blueprint is licensed for
    /// For copying for example this will be the amount of copies you are creating
    pub licensed_runs: i32,
    /// Unique id for the job
    pub job_id: JobId,
    /// ID of the facility the job was started in
    pub facility_id: i64,
    /// CharacterId of the character that strted the job
    pub installer_id: CharacterId,
    /// Status of the manufacturing entry
    pub status: String,
    /// ID of the location for the industry facility
    pub location_id: LocationId,
    /// Location ID of the location to which the output of the job will be delivered. Normally a station ID, but can also be a corporation facility
    pub output_location_id: LocationId,
    /// [CorporationId] of the corporation that owns this job
    #[serde(default)]
    pub corporation_id: Option<CorporationId>,
}

/// List of all industry activities
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, sqlx::Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "INDUSTRY_ACTIVITY")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IndustryActivity {
    /// Manufacturing of things
    Manufacturing,
    /// Researching time efficiency
    MaterialEfficiencyResearch,
    /// Researching material efficiency
    TimeEfficiencyResearch,
    /// Making blueprint copies
    Copying,
    /// The process of creating a more advanced item based on an existing item
    Invention,
    /// The process of combining raw and intermediate materials to create advanced components
    Reactions,
    /// No matches were found
    Unknown,
}

impl IndustryActivity {
    /// deserializes industry activity ids into their actual activitiy name
    fn from<'de, D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match u8::deserialize(d)? {
            1 => Self::Manufacturing,
            3 => Self::MaterialEfficiencyResearch,
            4 => Self::TimeEfficiencyResearch,
            5 => Self::Copying,
            8 => Self::Invention,
            9 => Self::Reactions,
            _ => Self::Unknown,
        })
    }
}

/// decides how cache rules are followed
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Cache {
    /// follows the rules given by the server
    Follow,
    /// ignores any cache rules returned by the server
    Ignore,
}
