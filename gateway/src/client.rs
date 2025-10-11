use reqwest::Client;

use crate::error::{Error, Result};

// either a full chain or the intermediate ca
pub const ENV_MTLS_ROOT_CA: &str    = "STARFOUNDRY_MTLS_ROOT_CA";
pub const ENV_MTLS_IDENTITY: &str   = "STARFOUNDRY_MTLS_IDENTITY";
pub const ENV_USER_AGENT: &str      = "STARFOUNDRY_USER_AGENT";

pub fn mtls_client() -> Result<Client> {
    let root_ca = reqwest::Certificate::from_pem(
        root_ca()?.as_bytes()
    )?;

    let identity = reqwest::Identity::from_pem(
        identity()?.as_bytes()
    )?;

    Client::builder()
        //.tls_built_in_root_certs(false)
        .add_root_certificate(root_ca)
        .use_rustls_tls()
        .identity(identity)
        .user_agent(user_agent()?)
        .https_only(true)
        .build()
        .map_err(Into::into)
}

fn user_agent() -> Result<String> {
    std::env::var(ENV_USER_AGENT)
        .map_err(|_| Error::EnvNotSet(ENV_USER_AGENT))
        .map_err(Into::into)
}

fn root_ca() -> Result<String> {
    std::env::var(ENV_MTLS_ROOT_CA)
        .map_err(|_| Error::EnvNotSet(ENV_MTLS_ROOT_CA))
        .map_err(Into::into)
}

fn identity() -> Result<String> {
    std::env::var(ENV_MTLS_IDENTITY)
        .map_err(|_| Error::EnvNotSet(ENV_MTLS_IDENTITY))
        .map_err(Into::into)
}
