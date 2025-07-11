use std::net::SocketAddr;

#[derive(Debug)]
pub struct Config {
    pub database_url:   String,
    pub server_address: SocketAddr,
}

impl Config {
    pub fn load() -> Self {
        let mut has_all = true;

        let server_address = if let Ok(x) = std::env::var("SERVER_ADDRESS") {
            x.parse().unwrap()
        } else {
            has_all = false;
            tracing::error!("Missing ENV 'SERVER_ADDRESS'");
            "0.0.0.0:8080".parse().unwrap()
        };

        let database_url: String = if let Ok(x) = std::env::var("DATABASE_URL") {
            x.into()
        } else {
            has_all = false;
            tracing::error!("Missing ENV 'DATABASE_URL'");
            String::new()
        };

        if !has_all {
            panic!("Not all ENV are set.")
        }

        Self {
            database_url,
            server_address
        }
    }
}
