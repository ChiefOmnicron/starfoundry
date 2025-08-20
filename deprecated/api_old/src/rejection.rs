use std::convert::Infallible;
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::api_docs::BadRequestError;
use crate::ReplyError;

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let json;

    if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        json = warp::reply::json(&serde_json::json!({
            "error": BadRequestError::Deserialization,
            "description": "The body could not be parsed, make sure it's valid json and validate the routes requires parameters"
        }));
    } else if let Some(ReplyError::Validation(e)) = err.find() {
        code = StatusCode::BAD_REQUEST;
        json = warp::reply::json(&serde_json::json!({
            "error": BadRequestError::Validation,
            "description": e
        }));
    } else if let Some(ReplyError::Unauthorized) = err.find() {
        code = StatusCode::UNAUTHORIZED;
        json = warp::reply::json(&serde_json::json!({
            "error": "UNAUTHORIZED",
            "description": "Authenticate and try again"
        }));
    } else if let Some(ReplyError::Forbidden) = err.find() {
        code = StatusCode::FORBIDDEN;
        json = warp::reply::json(&serde_json::json!({
            "error": "FORBIDDEN",
            "description": "You are not allowed to perform this action"
        }));
    } else if let Some(ReplyError::NotFound) = err.find() {
        code = StatusCode::NOT_FOUND;
        json = warp::reply::json(&serde_json::json!({
            "error": "NOT_FOUND",
            "description": "The requested resource was not found"
        }));
    } else if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        json = warp::reply::json(&serde_json::json!({
            "error": "NOT_FOUND",
            "description": "The requested resource was not found"
        }));
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        json = warp::reply::json(&serde_json::json!({
            "error": "UNKNOWN",
            "description": "An unknown error occurred"
        }));
    }

    Ok(warp::reply::with_status(json, code))
}
