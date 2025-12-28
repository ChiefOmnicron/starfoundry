use starfoundry_lib_gateway::{ENV_MTLS_IDENTITY, ENV_MTLS_ROOT_CA, ENV_USER_AGENT};
use tokio::net::TcpListener;
use url::Url;

use crate::auth::{ENV_EVE_GATEWAY_JWK_URL, ENV_EVE_GATEWAY_JWT_SIGN};

const ENV_APP_ADDRESS: &str     = "STARFOUNDRY_GATEWAY_APP_ADDRESS";
const ENV_SERVICE_ADDRESS: &str = "STARFOUNDRY_GATEWAY_SERVICE_ADDRESS";

#[derive(Debug)]
pub struct ConfigEnv {
    pub eve_gateway_jwk_url: Url,

    pub app_address:         TcpListener,
    pub service_address:     TcpListener,
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

        let eve_gateway_jwk_url = match Url::parse(
            &std::env::var(ENV_EVE_GATEWAY_JWK_URL)?
        ) {
            Ok(x) => x,
            Err(e) => {
                tracing::error!("Error validating config {ENV_EVE_GATEWAY_JWK_URL}. Error: {}", e);
                return Err("Error while parsing address".into());
            }
        };

        Ok(Self {
            eve_gateway_jwk_url,

            app_address,
            service_address,
        })
    }

    fn validate_env() -> usize {
        vec![
            ENV_APP_ADDRESS,
            ENV_SERVICE_ADDRESS,

            ENV_EVE_GATEWAY_JWK_URL,
            ENV_EVE_GATEWAY_JWT_SIGN,

            ENV_MTLS_ROOT_CA,
            ENV_MTLS_IDENTITY,
            ENV_USER_AGENT,
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
