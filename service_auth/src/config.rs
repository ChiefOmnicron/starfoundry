use tokio::net::TcpListener;

const ENV_DATABASE_URL: &str = "STARFOUNDRY_DATABASE_URL";
const ENV_SECRET_KEY: &str = "STARFOUNDRY_SECRET_KEY";

const ENV_APP_ADDRESS: &str = "STARFOUNDRY_AUTH_APP_ADDRESS";
const ENV_SERVICE_ADDRESS: &str = "STARFOUNDRY_AUTH_SERVICE_ADDRESS";

#[derive(Debug)]
pub struct Config {
    pub database_url:    String,
    pub secret_key:      String,

    pub app_address:     TcpListener,
    pub service_address: TcpListener,
}

impl Config {
    pub async fn load() -> Result<Self, Box<dyn std::error::Error>> {
        if Self::validate_env() > 0 {
            return Err("Not all required variables are set. Check logs".into());
        }

        let app_address = std::env::var(ENV_APP_ADDRESS).unwrap();
        let app_address = match tokio::net::TcpListener::bind(app_address).await {
            Ok(x) => x,
            Err(e) => {
                tracing::error!("Error validating config {ENV_APP_ADDRESS}. Error: {}", e);
                return Err("Error while parsing address".into());
            }
        };

        let service_address = std::env::var(ENV_SERVICE_ADDRESS).unwrap();
        let service_address = match tokio::net::TcpListener::bind(service_address).await {
            Ok(x) => x,
            Err(e) => {
                tracing::error!("Error validating config {ENV_SERVICE_ADDRESS}. Error: {}", e);
                return Err("Error while parsing address".into());
            }
        };

        let database_url = std::env::var(ENV_DATABASE_URL).unwrap();
        let secret_key = std::env::var(ENV_SECRET_KEY).unwrap();

        Ok(Self {
            database_url,
            secret_key,
            app_address,
            service_address,
        })
    }

    fn validate_env() -> usize {
        vec![
            ENV_APP_ADDRESS,
            ENV_SERVICE_ADDRESS,
            ENV_DATABASE_URL,
            ENV_SECRET_KEY,
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
