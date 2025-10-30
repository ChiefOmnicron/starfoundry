use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use url::Url;

/// Represents the configuration file with all it's fields
#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    /// list of all routes that are allowed to be routed internally
    pub routes: HashMap<String, ConfigFileRoute>,
}

/// Represents a domain configuration
#[derive(Debug, Deserialize)]
pub struct ConfigFileRoute {
    /// service the request gets routed to
    pub service_url:  Url,
    /// drops the first prefix instead of redirecting it to the target
    #[serde(default)]
    pub drop_prefix:  bool,
    /// if the route requires the user to be authenticated or not, per default true
    #[serde(default = "require_auth_default")]
    pub require_auth: bool,
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
                routes: HashMap::new(),
            }
        };

        Ok(config_file)
    }
}

fn require_auth_default() -> bool {
    true
}
