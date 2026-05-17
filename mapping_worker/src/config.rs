mod env;

use tokio::net::TcpListener;

use crate::config::env::ConfigEnv;

#[derive(Debug)]
pub struct Config {
    pub database_url:     String,

    pub service_address:  TcpListener,
}

impl Config {
    pub async fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let env = ConfigEnv::load().await?;

        Ok(Self::from(env))
    }
}

impl From<ConfigEnv> for Config {
    fn from(env: ConfigEnv) -> Self {
        Self {
            database_url:       env.database_url,

            service_address:    env.service_address,
        }
    }
}
