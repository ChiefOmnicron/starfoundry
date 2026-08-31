use axum::extract::{MatchedPath, Request, State};
use axum::http::header::{HOST, USER_AGENT};
use axum::middleware::Next;
use axum::response::IntoResponse;
use std::time::Instant;

use crate::state::AppState;

/// Collects statistics for incoming requests and adds them into the global
/// metrics object for later consumption
pub async fn path_metrics(
    State(state): State<AppState>,
    req:          Request,
    next:         Next,
) -> impl IntoResponse {
    let start = Instant::now();

    let path = if let Some(matched_path) = &req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };

    let headers = req.headers().clone();
    let agent = if let Some(x) = headers.get(USER_AGENT) {
        x.to_str().unwrap_or("invalid agent")
    } else {
        "header not present"
    };
    let host = if let Some(x) = headers.get(HOST) {
        x.to_str().unwrap_or("invalid host")
    } else {
        "header not present"
    };

    let method = req.method().clone();

    // Run the rest of the request handling first, so that the duration can be
    // recorded
    let response = next.run(req).await;

    let duration = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();
    let method = method.to_string();

    state.metric.increase_host_counter(host);
    state.metric.increase_http_request_total(
        method.to_string(),
        path.clone(),
        status.clone(),
        agent.into(),
    );state.metric.record_http_request_total(
        method.to_string(),
        path,
        status,
        agent.into(),
        duration,
    );

    response
}
