use serde::Deserialize;
use starfoundry_libs_types::TypeId;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use crate::{FOLDER_INPUT, Error};
use super::type_ids::TypeIdEntry;

pub fn parse(
    directory: &str,
) -> Result<HashMap<TypeId, BlueprintEntry>, Error> {
    tracing::info!("Parsing blueprints.yaml");
    let start = Instant::now();

    let path = format!(
        "{}/{}/fsd/blueprints.yaml",
        directory,
        FOLDER_INPUT,
    );

    if !Path::new(&path).exists() {
        return Err(Error::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| Error::CannotOpenBlueprintsFile(x, path))?;

    serde_yaml::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing blueprints.yaml, task took {:.2}s",
                start.elapsed().as_secs_f64()
            );
            x
        })
        .map_err(Error::ParseBlueprints)
}

/// Instead of having the blueprint type id, have the product of
/// the blueprint as key.
/// 
pub fn product_type_id_as_key(
    blueprints: &HashMap<TypeId, BlueprintEntry>,
    type_ids:   &HashMap<TypeId, TypeIdEntry>,
) -> HashMap<TypeId, BlueprintEntry> {
    // Map with the product as key
    blueprints
        .clone()
        .into_iter()
        .filter(|(btype_id, _)| {
            if let Some(y) = type_ids.get(&btype_id) {
                y.published
            } else {
                false
            }
        })
        .map(|(_, e)| e)
        .filter(|e| e.product().is_some())
        .filter(|e| e.materials().len() > 1)
        .map(|e| {
            // Unwrap is save because of the filter
            let ptype_id = e.product().unwrap();
            (ptype_id, e)
        })
        .collect::<HashMap<_, _>>()
}

/// Represents a blueprint taken from SDE
#[derive(Clone, Debug, Deserialize)]
pub struct BlueprintEntry {
    /// Holds all activities that are possible with that blueprint
    pub activities:           HashMap<ActivityName, Activity>,
    /// TypeId of the blueprint
    #[serde(rename(deserialize = "blueprintTypeID"))]
    #[allow(dead_code)]
    pub blueprint_type_id:    TypeId,
    /// Maximum number of runs that this blueprint can perform
    #[serde(rename(deserialize = "maxProductionLimit"))]
    pub max_production_limit: u32,
}

impl BlueprintEntry {
    /// Checks if the activity has reaction.
    ///
    /// # Returns
    ///
    /// * `true`  -> If the entry is a reaction
    /// * `false` -> If there are not reactions
    ///
    pub fn is_reaction(&self) -> bool {
        self.activities.get(&ActivityName::Reaction).is_some()
    }

    /// Checks if the blueprint has a manufacture or reaction job.
    ///
    /// # Returns
    ///
    /// * `true`  -> If there is either a manufacture or reaction job
    /// * `false` -> If there are not jobs
    ///
    pub fn has_job(&self) -> bool {
        let manufacture = self.activities.get(&ActivityName::Manufacturing);
        let reaction = self.activities.get(&ActivityName::Reaction);
        manufacture.is_some() || reaction.is_some()
    }

    /// Gets the product either from the manufacture job or the reaction job.
    ///
    /// # Returns
    ///
    /// * `None` -> If there is no product
    /// * `Some` -> TypeId of the product
    ///
    pub fn product(&self) -> Option<TypeId> {
        if let Some(x) = self.activities.get(&ActivityName::Manufacturing) {
            Some(x.products.get(0)?.type_id)
        } else if let Some(x) = self.activities.get(&ActivityName::Reaction) {
            Some(x.products.get(0)?.type_id)
        } else {
            None
        }
    }

    /// Gets the produced quantity of either a manufacturing or reaction job.
    ///
    /// # Returns
    ///
    /// * `None` -> If there is no product
    /// * `Some` -> Quantity of the produced product
    ///
    pub fn product_quantity(&self) -> Option<i32> {
        if let Some(x) = self.activities.get(&ActivityName::Manufacturing) {
            Some(x.products.get(0)?.quantity)
        } else if let Some(x) = self.activities.get(&ActivityName::Reaction) {
            Some(x.products.get(0)?.quantity)
        } else {
            None
        }
    }

    /// Gets the materials required for either manufacturing or reaction.
    ///
    /// # Returns
    ///
    /// List of all required materials. If there is no manufacturing or reaction
    /// job, an empty vec is returned.
    ///
    pub fn materials(&self) -> Vec<Material> {
        if let Some(x) = self.activities.get(&ActivityName::Manufacturing) {
            x.materials.clone()
        } else if let Some(x) = self.activities.get(&ActivityName::Reaction) {
            x.materials.clone()
        } else {
            Vec::new()
        }
    }

    /// Gets the time for a manufacture job.
    ///
    /// # Returns
    ///
    /// - `None` -> If the BP has no manufacture job
    /// - `Some` -> Time of the action
    ///
    pub fn manufacture_time(&self) -> Option<i32> {
        if !self.has_job() {
            return None;
        }

        if let Some(x) = self.activities.get(&ActivityName::Manufacturing) {
            Some(x.time)
        } else if let Some(x) = self.activities.get(&ActivityName::Reaction) {
            Some(x.time)
        } else {
            None
        }
    }
}

/// All possible activity that a blueprint can have
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActivityName {
    /// Copy
    Copying,
    /// Invention
    Invention,
    /// Manufacture
    Manufacturing,
    /// Reaction
    Reaction,
    /// Material research
    ResearchMaterial,
    /// Time research
    ResearchTime,
}

/// Represents a sinble blueprints activity
#[derive(Clone, Debug, Deserialize)]
pub struct Activity {
    /// Time it takes to perform the activity
    pub time:      i32,
    /// Required materials for the activity, will be an empty Vector if not
    /// materials are required
    #[serde(default)]
    pub materials: Vec<Material>,
    /// Products that are produced by this blueprint, will be an empty Vec if
    /// nothing is produced by this activity
    #[serde(default)]
    pub products:  Vec<Material>,
}

/// Represents a material required for an activity
#[derive(Clone, Debug, Deserialize)]
pub struct Material {
    /// Quantity that is required
    pub quantity: i32,
    /// TypeId of the material that is required
    #[serde(rename = "typeID")]
    pub type_id: TypeId,
}

