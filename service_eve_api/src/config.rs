mod env;
mod file;

pub use self::file::*;

use std::collections::HashMap;
use tokio::net::TcpListener;

use crate::config::env::ConfigEnv;
use crate::client::ConfigEveApi;

#[derive(Debug)]
pub struct Config {
    pub database_uri:    String,
    pub jwt_secret:      String,

    pub app_address:     TcpListener,
    pub service_address: TcpListener,

    pub domains:         HashMap<String, ConfigFileDomain>,
    pub eve_config:      ConfigEveApi,
}

impl Config {
    pub async fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let env = ConfigEnv::load().await?;
        let file = ConfigFile::load().await?;
        let eve_api = ConfigEveApi::load().await?;

        Ok(Self::from((env, file, eve_api)))
    }
}

impl From<(ConfigEnv, ConfigFile, ConfigEveApi)> for Config {
    fn from((
        env,
        file,
        eve_api,
    ): (
        ConfigEnv,
        ConfigFile,
        ConfigEveApi,
    )) -> Self {
        Self {
            database_uri:       env.database_uri,

            jwt_secret:         env.jwt_secret,
            app_address:        env.app_address,
            service_address:    env.service_address,

            domains:            file.domains,
            eve_config:         eve_api,
        }
    }
}
