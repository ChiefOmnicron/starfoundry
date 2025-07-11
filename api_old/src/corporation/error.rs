use std::fmt;

#[derive(Debug)]
pub enum CorporationError {
    CreateEveClient(starfoundry_libs_eve_api::Error),
    FetchInfo(starfoundry_libs_eve_api::Error),
}

impl warp::reject::Reject for CorporationError { }

impl fmt::Display for CorporationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
