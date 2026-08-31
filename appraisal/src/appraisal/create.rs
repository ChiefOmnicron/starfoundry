mod model;
mod service;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::api_docs::{BadRequest, InternalServerError, UnsupportedMediaType};
use crate::appraisal::create::model::{CreateAppraisalRequest, Appraisal};
use crate::appraisal::create::service::create;
use crate::appraisal::Result;
use crate::AppState;

/// Create Appraisal
/// 
/// - Alternative route: `/latest/appraisals`
/// - Alternative route: `/v1/appraisals`
/// 
/// ---
/// 
/// Creates a new appraisal
/// 
#[utoipa::path(
    post,
    path = "/",
    tag = "appraisal",
    request_body = CreateAppraisalRequest,
    responses(
        (
            body = Appraisal,
            description = "Newly created appraisal",
            status = CREATED,
        ),
        BadRequest,
        UnsupportedMediaType,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):           State<AppState>,
    Json(appraisal_info):   Json<CreateAppraisalRequest>,
) -> Result<impl IntoResponse> {
    appraisal_info.validate()?;

    let result = create(
            &state.postgres,
            appraisal_info,
        ).await?;

    Ok(
        (
            StatusCode::CREATED,
            Json(result)
        )
    )
}
