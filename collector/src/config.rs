/*#[derive(Clone, Debug)]
pub struct Config {
    pub server_address: String,
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

        if std::env::var("DATABASE_URL").is_err() {
            has_all = false;
            tracing::error!("Missing ENV 'DATABASE_URL'");
        }

        if !has_all {
            panic!("Not all ENV are set.")
        }

        Self {
            server_address
        }
    }
}
*/
