use std::fmt;

#[derive(Debug)]
pub enum SearchError {
    SearchSystems(sqlx::Error),
}

impl warp::reject::Reject for SearchError { }

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
