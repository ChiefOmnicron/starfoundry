use serde::Deserialize;
use uuid::Uuid;
use starfoundry_lib_types::{CharacterId, TypeId};
use std::path::Path;
use std::fs::{DirEntry, File};
use std::io::Read;

pub fn load(
    folder: String,
) -> Result<Vec<Build>, Box<dyn std::error::Error>> {
    let load_config = |file: &DirEntry| -> Result<Build, String> {
        tracing::info!("Adding {}", file.path().display());

        let mut toml_config_file = if let Ok(x) = File::open(file.path()) {
            x
        } else {
            tracing::error!("Could not open file {}. Skipping", file.path().display());
            return Err(format!("Could not open file {}. Skipping", file.path().display()));
        };

        let mut toml_config = String::new();
        if let Err(_) = toml_config_file.read_to_string(&mut toml_config) {
            tracing::error!("Could not read file {}. Skipping", file.path().display());
            return Err(format!("Could not read file {}. Skipping", file.path().display()));
        };

        let config = match toml::from_str(&toml_config) {
            Ok(x) => x,
            Err(e) => {
                tracing::error!("Invalid config {}. {e} Skipping", file.path().display());
                return Err(format!("Invalid config {}. {e} Skipping", file.path().display()));
            }
        };

        Ok(config)
    };

    let configs = visit_dirs(Path::new(&folder), &load_config)?;
    Ok(configs)
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry) -> Result<Build, String>) -> Result<Vec<Build>, Box<dyn std::error::Error>> {
    let mut configs = Vec::new();

    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_symlink() {
                continue;
            } if path.is_dir() {
                configs.extend(visit_dirs(&path, cb)?);
            } else {
                let config = cb(&entry)?;
                configs.push(config);
            }
        }
    }

    Ok(configs)
}

#[derive(Clone, Debug, Deserialize)]
pub struct Build {
    pub id:                     Uuid,
    pub name:                   String,
    pub structure_group:        Uuid,
    #[serde(default)]
    pub category:               String,
    #[serde(default)]
    pub image_type:             String,
    #[serde(default)]
    pub image_type_id:          i32,
    #[serde(default)]
    pub tags:                   Vec<String>,
    #[serde(default)]
    pub bpc:                    Vec<BuildBpc>,
    #[serde(default)]
    pub additional_products:    Vec<Uuid>,
    pub store:                  String,
    pub project:                String,
    pub market:                 String,
    pub delivery_time:          String,
    pub delivery_location:      Vec<i32>,

    pub message:                Option<String>,
    /// Can be a CharacterId, CorporationId or AllianceId
    pub whitelist:              Option<Vec<CharacterId>>,
    /// Can be a CharacterId, CorporationId or AllianceId
    pub blacklist:              Option<Vec<CharacterId>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BuildBpc {
    pub type_id: TypeId,
    pub price:   i64,
}
