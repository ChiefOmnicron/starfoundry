use serde::{Deserialize, Serialize};
use starfoundry_lib_types::RegionId;
use std::env;
use std::fs::File;
use std::io::Read;
use utoipa::ToSchema;

#[derive(Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ShopConfig {
    pub info:        ShopConfigInfo,
    pub restriction: ShopConfigRestriction,
}

#[derive(Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ShopConfigRestriction {
    /// Character, Corporation, Alliance that is blacklisted from using the service
    pub blacklist:    Vec<u64>,
    /// Regions where the service delivers to
    pub region_ids:   Vec<RegionId>,
    /// Character, Corporation, Alliance that can use this service
    pub whitelist:    Vec<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ShopConfigInfo {
    /// UUID of the offer
    pub name: String,
}

impl Default for ShopConfigInfo {
    fn default() -> Self {
        Self {
            name: "StarFoundry Store".into(),
        }
    }
}

impl ShopConfig {
    pub async fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let pwd = env::current_dir().unwrap_or(".".into());
        let config_path = format!("{}/config.toml", pwd.display());

        let mut toml_config_file = File::open(config_path)?;
        let mut toml_config = String::new();
        toml_config_file.read_to_string(&mut toml_config)?;

        let config_file: ShopConfig = if let Ok(x) = toml::from_str(&toml_config) {
            x
        } else {
            tracing::error!("Invalid config.toml");
            return Err("Invalid config".into());
        };

        Ok(config_file)
    }
}
