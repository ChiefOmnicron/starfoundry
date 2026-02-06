use starfoundry_lib_gateway::{ENV_EVE_GATEWAY_API, ENV_USER_AGENT};
use tokio::net::TcpListener;

const ENV_DATABASE_URL: &str    = "STARFOUNDRY_STORE_DATABASE_URL";
const ENV_DISCORD_URL: &str     = "STARFOUNDRY_STORE_DISCORD_URL";
const ENV_APP_ADDRESS: &str     = "STARFOUNDRY_STORE_APP_ADDRESS";
const ENV_SERVICE_ADDRESS: &str = "STARFOUNDRY_STORE_SERVICE_ADDRESS";

#[derive(Debug)]
pub struct ConfigEnv {
    pub database_url:    String,
    pub discord_url:     String,

    pub app_address:     TcpListener,
    pub service_address: TcpListener,
}

impl ConfigEnv {
    pub async fn load() -> Result<Self, Box<dyn std::error::Error>> {
        if Self::validate_env() > 0 {
            return Err("Not all required variables are set. Check logs".into());
        }

        let app_address = std::env::var(ENV_APP_ADDRESS)?;
        let app_address = match TcpListener::bind(app_address).await {
            Ok(x) => x,
            Err(e) => {
                tracing::error!("Error validating config {ENV_APP_ADDRESS}. Error: {}", e);
                return Err("Error while parsing address".into());
            }
        };

        let service_address = std::env::var(ENV_SERVICE_ADDRESS)?;
        let service_address = match TcpListener::bind(service_address).await {
            Ok(x) => x,
            Err(e) => {
                tracing::error!("Error validating config {ENV_SERVICE_ADDRESS}. Error: {}", e);
                return Err("Error while parsing address".into());
            }
        };

        let database_url = std::env::var(ENV_DATABASE_URL)?;
        let discord_url = std::env::var(ENV_DISCORD_URL)?;

        Ok(Self {
            database_url,
            discord_url,

            app_address,
            service_address,
        })
    }

    fn validate_env() -> usize {
        vec![
            ENV_DATABASE_URL,
            ENV_DISCORD_URL,
            ENV_APP_ADDRESS,
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
