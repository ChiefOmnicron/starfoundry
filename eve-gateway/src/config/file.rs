use serde::Deserialize;
use starfoundry_lib_types::CharacterId;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub domains: HashMap<String, ConfigFileDomain>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigFileDomain {
    pub admins:             Vec<CharacterId>,
    pub whitelist:          Vec<i64>,
    pub character_scopes:   Vec<String>,
    pub corporation_scopes: Vec<String>,
    pub redirect:           String,
}

impl ConfigFile {
    pub async fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let pwd = env::current_dir().unwrap_or(".".into());
        let config_path = format!("{}/config.toml", pwd.display());

        let mut toml_config_file = File::open(config_path)?;
        let mut toml_config = String::new();
        toml_config_file.read_to_string(&mut toml_config)?;

        let config_file: ConfigFile = if let Ok(x) = toml::from_str(&toml_config) {
            x
        } else {
            tracing::error!("Invalid config.toml");
            Self {
                domains: HashMap::new(),
            }
        };

        Ok(config_file)
    }
}
