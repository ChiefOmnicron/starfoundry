use serde::Serialize;
use std::fmt;
use utoipa::ToSchema;

#[derive(Debug)]
pub enum ReplyError {
    BadRequest,
    Unauthorized,
    Forbidden,
    Internal,
    NotFound,

    Validation(String),
    BadRequestWithPayload(BadRequestPayload),
}

impl warp::reject::Reject for ReplyError { }

impl fmt::Display for ReplyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BadRequestPayload {
    pub error:       String,
    pub description: String,
}
