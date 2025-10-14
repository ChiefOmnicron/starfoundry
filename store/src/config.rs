mod env;
mod file;

pub use self::file::*;

use starfoundry_lib_types::starfoundry_uuid;
use std::net::TcpListener as StdTcpListener;
use tokio::net::TcpListener as TokioTcpListener;

use crate::config::env::ConfigEnv;

#[derive(Debug)]
pub struct Config {
    /// address under which the application should be exposed
    pub app_address:     StdTcpListener,
    /// address under which health checks and metrics are exposed
    pub service_address: TokioTcpListener,

    pub mtls_cert:       String,
    pub mtls_priv:       String,

    pub database_url:    String,
    pub discord_url:     String,

    pub shop_config:     ShopConfig,
}

impl Config {
    pub async fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let env = ConfigEnv::load().await?;
        let file = ShopConfig::load().await?;

        Ok(Self::from((env, file)))
    }
}

impl From<(ConfigEnv, ShopConfig)> for Config {
    fn from((
        env,
        shop_config,
    ): (
        ConfigEnv,
        ShopConfig,
    )) -> Self {
        Self {
            app_address:        env.app_address,
            service_address:    env.service_address,

            mtls_cert:          env.mtls_cert,
            mtls_priv:          env.mtls_priv,

            database_url:       env.database_url,
            discord_url:        env.discord_url,

            shop_config:        shop_config,
        }
    }
}

starfoundry_uuid!(ProductUuid, "ProductUuid");
starfoundry_uuid!(OrderUuid, "OrderUuid");
