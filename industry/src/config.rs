mod env;

use std::net::TcpListener as StdTcpListener;
use tokio::net::TcpListener as TokioTcpListener;

use crate::config::env::ConfigEnv;

#[derive(Debug)]
pub struct Config {
    pub database_url:     String,

    pub app_address:      StdTcpListener,
    pub service_address:  TokioTcpListener,
    pub internal_address: StdTcpListener,

    pub mtls_cert:        String,
    pub mtls_priv:        String,
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

            app_address:        env.app_address,
            service_address:    env.service_address,
            internal_address:   env.internal_address,

            mtls_cert:          env.mtls_cert,
            mtls_priv:          env.mtls_priv,
        }
    }
}
