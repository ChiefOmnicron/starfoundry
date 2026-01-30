use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

/// Represents a domain configuration
#[derive(Clone, Debug, Deserialize)]
pub struct ConfigFile {
    /// Lists of hosts that can be used to obtain data
    pub hosts: HashMap<String, ConfigFileHostEntry>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigFileHostEntry {
    pub address: String,
}

impl ConfigFile {
    /// Loads the configuration file.
    /// The configuration file should be in the same folder as the binary
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let pwd = std::env::current_dir().unwrap_or(".".into());
        let config_path = format!("{}/config.toml", pwd.display());

        let mut toml_config_file = File::open(config_path)?;
        let mut toml_config = String::new();
        toml_config_file.read_to_string(&mut toml_config)?;

        let config_file: ConfigFile = if let Ok(x) = toml::from_str(&toml_config) {
            x
        } else {
            tracing::error!("Invalid config.toml");
            Self {
                hosts: HashMap::new(),
            }
        };

        Ok(config_file)
    }
}
