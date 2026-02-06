use starfoundry_lib_gateway::{ENV_EVE_GATEWAY_API, ENV_USER_AGENT};
use std::net::SocketAddr;

const ENV_DATABASE_URL: &str    = "STARFOUNDRY_EVE_GATEWAY_WORKER_DATABASE_URL";
const ENV_SERVICE_ADDRESS: &str = "STARFOUNDRY_EVE_GATEWAY_WORKER_SERVICE_ADDRESS";

#[derive(Debug)]
pub struct Config {
    pub database_url:    String,

    pub service_address: SocketAddr,
}

impl Config {
    pub async fn load() -> Result<Self, Box<dyn std::error::Error>> {
        if Self::validate_env() > 0 {
            return Err("Not all required variables are set. Check logs".into());
        }

        let service_address = std::env::var(ENV_SERVICE_ADDRESS)?;
        let service_address: SocketAddr = match service_address.parse() {
            Ok(x) => x,
            Err(e) => {
                tracing::error!("Error validating config {ENV_SERVICE_ADDRESS}. Error: {}", e);
                return Err("Error while parsing address".into());
            }
        };

        let database_url = std::env::var(ENV_DATABASE_URL)?;

        Ok(Self {
            database_url,
            service_address,
        })
    }

    fn validate_env() -> usize {
        vec![
            ENV_DATABASE_URL,
            ENV_SERVICE_ADDRESS,

            ENV_USER_AGENT,

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
