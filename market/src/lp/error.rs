use thiserror::Error;

pub type Result<T, E = LpError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum LpError {
    #[error("no valid solution was found")]
    NoSolution,
}
