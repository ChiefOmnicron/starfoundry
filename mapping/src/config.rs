use tokio::net::TcpListener;

const ENV_DATABASE_URL: &str     = "STARFOUNDRY_MAPPING_DATABASE_URL";
const ENV_APP_ADDRESS: &str      = "STARFOUNDRY_MAPPING_APP_ADDRESS";
const ENV_SERVICE_ADDRESS: &str  = "STARFOUNDRY_MAPPING_SERVICE_ADDRESS";

/// General application config
#[derive(Debug)]
pub struct Config {
    /// postgres connection string, containing the username, password, address and database name
    pub database_url:     String,

    /// address under which the application should be exposed
    pub app_address:      TcpListener,
    /// address under which health checks and metrics are exposed
    pub service_address:  TcpListener,
}

impl Config {
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

        Ok(Self {
            database_url,
            app_address,
            service_address,
        })
    }

    fn validate_env() -> usize {
        [
            ENV_DATABASE_URL,
            ENV_APP_ADDRESS,
            ENV_SERVICE_ADDRESS,
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
