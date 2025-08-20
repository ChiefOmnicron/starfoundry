use tokio::net::TcpListener;

use crate::auth::{ENV_JWT_ECDSA_PRIVATE, ENV_JWT_ECDSA_PUBLIC, ENV_JWT_ISSUER_DOMAIN};

const ENV_DATABASE_URL: &str    = "STARFOUNDRY_EVE_GATEWAY_DATABASE_URL";
const ENV_APP_ADDRESS: &str     = "STARFOUNDRY_EVE_GATEWAY_APP_ADDRESS";
const ENV_SERVICE_ADDRESS: &str = "STARFOUNDRY_EVE_GATEWAY_SERVICE_ADDRESS";

#[derive(Debug)]
pub struct ConfigEnv {
    pub database_uri:    String,

    pub app_address:     TcpListener,
    pub service_address: TcpListener,
}

impl ConfigEnv {
    pub async fn load() -> Result<Self, Box<dyn std::error::Error>> {
        if Self::validate_env() > 0 {
            return Err("Not all required variables are set. Check logs".into());
        }

        let app_address = std::env::var(ENV_APP_ADDRESS)?;
        let app_address = match tokio::net::TcpListener::bind(app_address).await {
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

        let database_uri = std::env::var(ENV_DATABASE_URL)?;

        Ok(Self {
            database_uri,
            app_address,
            service_address,
        })
    }

    fn validate_env() -> usize {
        vec![
            ENV_DATABASE_URL,
            ENV_APP_ADDRESS,
            ENV_SERVICE_ADDRESS,

            ENV_JWT_ECDSA_PRIVATE,
            ENV_JWT_ECDSA_PUBLIC,
            ENV_JWT_ISSUER_DOMAIN,

            crate::ENV_CLIENT_ID,
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
