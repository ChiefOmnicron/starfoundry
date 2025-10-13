mod env;
mod file;

pub use self::file::*;

use std::collections::HashMap;
use tokio::net::TcpListener;
use url::Url;

use crate::config::env::ConfigEnv;

/// General application config
#[derive(Debug)]
pub struct Config {
    /// address under which the application should be exposed
    pub app_address:         TcpListener,
    /// address under which health checks and metrics are exposed
    pub service_address:     TcpListener,

    pub eve_gateway_jwk_url: Url,


    /// list of domains that are allowed to use this service for authentication
    pub routes:              HashMap<String, ConfigFileRoute>,
}

impl Config {
    /// loads the environment and config file and joins them together into a
    /// single config
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
            eve_gateway_jwk_url: env.eve_gateway_jwk_url,

            app_address:         env.app_address,
            service_address:     env.service_address,

            routes:              file.routes,
        }
    }
}
