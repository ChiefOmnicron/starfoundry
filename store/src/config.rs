mod env;
mod file;

pub use self::file::*;

use starfoundry_lib_types::starfoundry_uuid;
use tokio::net::TcpListener;

use crate::config::env::ConfigEnv;

#[derive(Debug)]
pub struct Config {
    pub app_address:        TcpListener,
    pub service_address:    TcpListener,

    pub database_url:       String,
    pub discord_url:        String,
    pub gateway_jwk_url:    String,

    pub shop_config:        ShopConfig,
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

            database_url:       env.database_url,
            discord_url:        env.discord_url,
            gateway_jwk_url:    env.gateway_jwk_url,

            shop_config:        shop_config,
        }
    }
}

starfoundry_uuid!(ProductUuid, "ProductUuid");
starfoundry_uuid!(OrderUuid, "OrderUuid");
