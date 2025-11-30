use serde::Deserialize;
use starfoundry_lib_types::CharacterId;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

/// Represents the configuration file with all it's fields
#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    /// list of all domains that are allowed to login
    pub domains: HashMap<String, ConfigFileDomain>,
}

/// Represents a domain configuration
#[derive(Debug, Deserialize)]
pub struct ConfigFileDomain {
    /// List of [CharacterId]s that are admins in the domain
    pub admins:             Vec<CharacterId>,
    /// Whitelist of CharacterIds, CorporationIds or AllianceIds that are allowed
    /// to login
    /// If kept empty, everybody can login
    pub whitelist:          Vec<i64>,
    /// Scopes that are requested when a user logs in
    /// It is recommended to always add `publicData`
    pub character_scopes:   Vec<String>,
    /// Scopes that are requested when a corporation logs in
    /// It is recommended to always add `publicData`
    pub corporation_scopes: Vec<String>,
    /// Link to redirect back
    /// This must be the application the user is currently using
    pub redirect:           String,
}

impl ConfigFile {
    /// Loads the configuration file.
    /// The configuration file should be in the same folder as the binary
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
