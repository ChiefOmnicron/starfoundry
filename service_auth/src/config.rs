use tokio::net::TcpListener;

const ENV_DATABASE_URL: &str = "DATABASE_URL";

const ENV_APP_ADDRESS: &str = "STARFOUNDRY_AUTH_APP_ADDRESS";
const ENV_SERVICE_ADDRESS: &str = "STARFOUNDRY_AUTH_SERVICE_ADDRESS";

#[derive(Debug)]
pub struct Config {
    pub database_url:    String,

    pub app_address:     TcpListener,
    pub service_address: TcpListener,
}

impl Config {
    pub async fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let app_address = if let Ok(x) = std::env::var(ENV_APP_ADDRESS) {
            match tokio::net::TcpListener::bind(x).await {
                Ok(x) => x,
                Err(e) => {
                    tracing::error!("Error validating config {ENV_APP_ADDRESS}. Error: {}", e);
                    tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap()
                }
            }
        } else {
            tracing::error!("Missing ENV '{ENV_APP_ADDRESS}'");
            tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap()
        };

        let service_address = if let Ok(x) = std::env::var(ENV_SERVICE_ADDRESS) {
            match tokio::net::TcpListener::bind(x).await {
                Ok(x) => x,
                Err(e) => {
                    tracing::error!("Error validating config {ENV_SERVICE_ADDRESS}. Error: {}", e);
                    tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap()
                }
            }
        } else {
            tracing::error!("Missing ENV '{ENV_SERVICE_ADDRESS}'");
            tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap()
        };

        let database_url = if let Ok(x) = std::env::var(ENV_DATABASE_URL) {
            x
        } else {
            tracing::error!("Missing ENV '{ENV_DATABASE_URL}'");
            String::new()
        };

        Ok(Self {
            database_url,
            app_address,
            service_address,
        })
    }
}
