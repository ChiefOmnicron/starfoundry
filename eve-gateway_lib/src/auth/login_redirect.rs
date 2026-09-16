use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Contains the redirection URL including scopes
/// for the request to redirect to
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginRedirect {
    pub url: String,
}