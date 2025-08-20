mod env;
mod file;

pub use self::file::*;

use std::collections::HashMap;
use tokio::net::TcpListener;

use crate::config::env::ConfigEnv;

#[derive(Debug)]
pub struct Config {
    pub database_uri:    String,

    pub app_address:     TcpListener,
    pub service_address: TcpListener,

    pub domains:         HashMap<String, ConfigFileDomain>,
}

impl Config {
    pub async fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let env = ConfigEnv::load().await?;
        let file = ConfigFile::load().await?;

        Ok(Self::from((env, file)))
    }
}

impl From<(ConfigEnv, ConfigFile)> for Config {
    fn from((
        env,
        file
    ): (
        ConfigEnv,
        ConfigFile,
    )) -> Self {
        Self {
            database_uri:       env.database_uri,

            app_address:        env.app_address,
            service_address:    env.service_address,

            domains:            file.domains,
        }
    }
}
