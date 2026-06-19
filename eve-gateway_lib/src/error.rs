use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("{0:?}")]
    GatewayClientError(Box<starfoundry_lib_gateway::error::Error>),

    #[error("the env {0} is not set")]
    EnvNotSet(&'static str),
    #[error("error while parsing url. Validate the environment variables, error: '{0}'")]
    UrlParseError(url::ParseError),

    #[error("the given category '{0}' is not valid, it must be one of: 'agent', 'alliance', 'character', 'constellation', 'corporation', 'faction', 'inventory_type', 'region', 'solar_system', 'station', 'structure'")]
    InvalidSearchCategory(String),
}

// Remove once this_error implements it
// https://github.com/dtolnay/thiserror/issues/424
// https://github.com/dtolnay/thiserror/pull/431
#[macro_export]
macro_rules! boxed_from {
    ($dst_ty:ident :: $variant:ident, $src_ty:ty) => {
        impl From<$src_ty> for $dst_ty {
            fn from(value: $src_ty) -> Self {
                Self::$variant(Box::new(value))
            }
        }
    };
}

boxed_from!(Error::GatewayClientError, starfoundry_lib_gateway::error::Error);
