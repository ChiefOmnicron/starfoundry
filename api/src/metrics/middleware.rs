use axum::extract::{MatchedPath, Request};
use axum::http::header::USER_AGENT;
use axum::middleware::Next;
use axum::response::IntoResponse;
use std::time::Instant;

pub async fn path_metrics(req: Request, next: Next) -> impl IntoResponse {
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
 
    let method = req.method().clone();
 
    // Run the rest of the request handling first, so we can measure it and get response
    // codes.
    let response = next.run(req).await;
 
    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();
 
    let labels_count = [
        ("method", method.to_string()),
        ("path", path.clone()),
        ("status", status.clone()),
        ("agent", agent.into())
    ];
    let labels_duration = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];
 
    metrics::counter!("http_requests_total", &labels_count).increment(1);
    metrics::histogram!("http_requests_duration_seconds", &labels_duration).record(latency);
 
    response
}
