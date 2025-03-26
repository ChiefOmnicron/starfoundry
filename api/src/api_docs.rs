use utoipa::{openapi, Modify, OpenApi};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

use crate::BadRequestPayload;

#[derive(OpenApi)]
#[openapi(
    info(description = "StarFoundry API"),
    modifiers(&Auth),
    paths(
        crate::appraisal::compression,
        crate::appraisal::create,
        crate::appraisal::fetch,
        crate::appraisal::markets,
        crate::appraisal::reprocessing,

        crate::healthcheck::livez,
        crate::healthcheck::readyz,

        crate::job_detection::fetch,
        crate::job_detection::update_job_add,
        crate::job_detection::update_job_delete,
        crate::job_detection::update_job_replace,

        crate::project::excess::fetch,
        crate::project::excess::update_price,
        crate::project::job::active,
        crate::project::job::delete,
        crate::project::job::fetch,
        crate::project::job::startable,
        crate::project::job::update,
        crate::project::job_assignment::create,
        crate::project::job_assignment::fetch,
        crate::project::job_assignment::update_job_state,
        crate::project::market::add,
        crate::project::market::delete,
        crate::project::market::fetch_prices_gas,
        crate::project::market::fetch_prices_minerals,
        crate::project::market::fetch_prices,
        crate::project::market::fetch,
        crate::project::market::last_fetch,
        crate::project::market::update,
        crate::project::market::update_bulk,
        crate::project::market::update_minerals,
        crate::project::misc::add,
        crate::project::misc::delete,
        crate::project::misc::fetch,
        crate::project::misc::update,
        crate::project::permission::can_write,
        crate::project::permission::is_owner,
        crate::project::product::fetch,
        crate::project::service::check_resources,
        crate::project::service::cost_estimate,
        crate::project::service::create,
        crate::project::service::delete,
        crate::project::service::fetch,
        crate::project::service::list,
        crate::project::service::update,
        crate::project::stock::fetch,
        crate::project::stock::update_price,
    ),
)]
pub struct ApiDoc;

/// the operation was successful, but does not return any data
#[derive(utoipa::IntoResponses)]
#[response(status = NO_CONTENT)]
pub struct NoContent;

/// the given parameters are incorrect
#[derive(utoipa::IntoResponses)]
#[response(status = BAD_REQUEST)]
pub struct BadRequest;

/// the given parameters are incorrect
#[derive(utoipa::IntoResponses)]
#[response(
    status = BAD_REQUEST,
    example = json!({
        "error": "NO_SOLUTION",
        "description": "No solution found for compression request. Adjust the parameters."
    })
)]
pub struct BadRequestWithPayload(pub BadRequestPayload);

/// the ressource was not found
#[derive(utoipa::IntoResponses)]
#[response(status = NOT_FOUND)]
pub struct NotFound;

/// authenticate and then try again
#[derive(utoipa::IntoResponses)]
#[response(status = UNAUTHORIZED)]
pub struct Unauthorized;

/// the user is not allowed to use the ressource
#[derive(utoipa::IntoResponses)]
#[response(status = FORBIDDEN)]
pub struct Forbidden;

/// there was an unknown error
#[derive(utoipa::IntoResponses)]
#[response(status = INTERNAL_SERVER_ERROR)]
pub struct InternalServerError;

/// wrong media type, must be application/json
#[derive(utoipa::IntoResponses)]
#[response(status = UNSUPPORTED_MEDIA_TYPE)]
pub struct UnsupportedMediaType;

#[derive(Debug)]
struct Auth;

impl Modify for Auth {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        if let Some(schema) = openapi.components.as_mut() {
            schema.add_security_scheme(
                "jwt",
                SecurityScheme::Http
                (
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}
