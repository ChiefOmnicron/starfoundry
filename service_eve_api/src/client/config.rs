use std::fmt::Debug;
use std::ops::Deref;

const EVE_CALLBACK: &str   = "STARFOUNDRY_EVE_CALLBACK";
const EVE_CLIENT_ID: &str  = "STARFOUNDRY_EVE_CLIENT_ID";
const EVE_SECRET_KEY: &str = "STARFOUNDRY_EVE_SECRET_KEY";

#[derive(Debug)]
pub struct ConfigEveApi {
    pub callback:   String,

    pub client_id:  EveClientId,
    pub secret_key: EveSecretKey,
}

impl ConfigEveApi {
    pub async fn load() -> Result<Self, Box<dyn std::error::Error>> {
        if Self::validate_env() > 0 {
            return Err("Not all required variables are set. Check logs".into());
        }

        let callback = std::env::var(EVE_CALLBACK)?;
        let client_id = std::env::var(EVE_CLIENT_ID)?;
        let secret_key = std::env::var(EVE_SECRET_KEY)?;

        Ok(Self {
            callback,
            client_id:  EveClientId(client_id),
            secret_key: EveSecretKey(secret_key),
        })
    }

    fn validate_env() -> usize {
        vec![
            EVE_CALLBACK,
            EVE_CLIENT_ID,
            EVE_SECRET_KEY,
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

#[derive(Clone)]
pub struct EveClientId(String);

impl Deref for EveClientId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for EveClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct EveSecretKey(String);

impl Deref for EveSecretKey {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for EveSecretKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "****")
    }
}

#[derive(Clone)]
pub struct UserAgent(String);

impl Deref for UserAgent {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for UserAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
