use starfoundry_lib_eve_gateway::{ENV_EVE_GATEWAY_API, ENV_MTLS_IDENTITY, ENV_MTLS_ROOT_CA, ENV_USER_AGENT};
use std::net::TcpListener as StdTcpListener;
use tokio::net::TcpListener as TokioTcpListener;

const ENV_DATABASE_URL: &str    = "STARFOUNDRY_STORE_DATABASE_URL";
const ENV_DISCORD_URL: &str     = "STARFOUNDRY_STORE_DISCORD_URL";
const ENV_APP_ADDRESS: &str     = "STARFOUNDRY_STORE_APP_ADDRESS";
const ENV_SERVICE_ADDRESS: &str = "STARFOUNDRY_STORE_SERVICE_ADDRESS";

pub const ENV_REDIRECT: &str    = "STARFOUNDRY_STORE_REDIRECT";

const ENV_MTLS_CERT: &str       = "STARFOUNDRY_STORE_MTLS_CERT";
const ENV_MTLS_PRIV: &str       = "STARFOUNDRY_STORE_MTLS_PRIV";

#[derive(Debug)]
pub struct ConfigEnv {
    pub database_url:    String,
    pub discord_url:     String,

    pub app_address:     StdTcpListener,
    pub service_address: TokioTcpListener,

    pub mtls_cert:       String,
    pub mtls_priv:       String,
}

impl ConfigEnv {
    pub async fn load() -> Result<Self, Box<dyn std::error::Error>> {
        if Self::validate_env() > 0 {
            return Err("Not all required variables are set. Check logs".into());
        }

        let app_address = std::env::var(ENV_APP_ADDRESS)?;
        let app_address = match std::net::TcpListener::bind(app_address) {
            Ok(x) => x,
            Err(e) => {
                tracing::error!("Error validating config {ENV_APP_ADDRESS}. Error: {}", e);
                return Err("Error while parsing address".into());
            }
        };

        let service_address = std::env::var(ENV_SERVICE_ADDRESS)?;
        let service_address = match tokio::net::TcpListener::bind(service_address).await {
            Ok(x) => x,
            Err(e) => {
                tracing::error!("Error validating config {ENV_SERVICE_ADDRESS}. Error: {}", e);
                return Err("Error while parsing address".into());
            }
        };

        let database_url = std::env::var(ENV_DATABASE_URL)?;
        let discord_url = std::env::var(ENV_DISCORD_URL)?;

        let mtls_cert = std::env::var(ENV_MTLS_CERT)?;
        let mtls_priv = std::env::var(ENV_MTLS_PRIV)?;

        Ok(Self {
            database_url,
            discord_url,

            app_address,
            service_address,

            mtls_cert,
            mtls_priv,
        })
    }

    fn validate_env() -> usize {
        vec![
            ENV_DATABASE_URL,
            ENV_DISCORD_URL,
            ENV_APP_ADDRESS,
            ENV_SERVICE_ADDRESS,

            ENV_REDIRECT,

            ENV_MTLS_ROOT_CA,
            ENV_MTLS_IDENTITY,
            ENV_USER_AGENT,

            ENV_MTLS_CERT,
            ENV_MTLS_PRIV,

            ENV_EVE_GATEWAY_API,
        ]
        .iter()
        .map(|x| {
            let var = std::env::var(x);
            if var.is_err() {
                tracing::error!("Missing required ENV {x}");
            }
            var
        })
        .filter(|x| x.is_err())
        .count()
    }
}
