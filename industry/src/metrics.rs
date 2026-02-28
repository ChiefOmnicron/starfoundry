mod middleware;

pub use self::middleware::*;

use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::encoding::text::encode;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::histogram::Histogram;
use prometheus_client::registry::Registry;
use std::sync::Arc;

const HTTP_DURATION_BUCKETS: [f64; 12] = [
    0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0,
];

/// Exports prometheus metrics
pub fn route(
    registry: Arc<Registry>,
) -> impl IntoResponse {
    let mut buffer = String::new();
    encode(&mut buffer, &registry).unwrap();

    (
        StatusCode::OK,
        [(
            CONTENT_TYPE,
            "text/plain; version=0.0.4"
        )],
        buffer,
    )
}

pub struct Metric {
    host_count:         Family<MetricHostLabel, Counter>,
    http_request_total: Family<MetricHttpRequestLabel, Counter>,
    http_duration:      Family<MetricDurationLabel, Histogram>,
}

impl Metric {
    pub fn new() -> Self {
        Self  {
            host_count:         Family::<MetricHostLabel, Counter>::default(),
            http_request_total: Family::<MetricHttpRequestLabel, Counter>::default(),
            http_duration:      Family::new_with_constructor(|| {
                Histogram::new(HTTP_DURATION_BUCKETS.into_iter())
            }),
        }
    }

    pub fn increase_host_counter<S: Into<String>>(
        &self,
        host: S,
    ) {
        self
            .host_count
            .get_or_create(&MetricHostLabel {
                host: host.into(),
            })
            .inc();
    }

    pub fn increase_http_request_total<S: Into<String>>(
        &self,
        method: S,
        path:   S,
        status: S,
        agent:  S,
    ) {
        self
            .http_request_total
            .get_or_create(&MetricHttpRequestLabel {
                method: method.into(),
                path:   path.into(),
                status: status.into(),
                agent:  agent.into(),
            })
            .inc();
    }

    pub fn record_http_request_total<S: Into<String>>(
        &self,
        method:   S,
        path:     S,
        status:   S,
        agent:    S,
        duration: f64,
    ) {
        self
            .http_duration
            .get_or_create(&MetricDurationLabel {
                method: method.into(),
                path:   path.into(),
                status: status.into(),
                agent:  agent.into(),
            })
            .observe(duration);
    }

    pub fn register(
        &self,
        registry: &mut Registry,
    ) {
        registry.register(
            "host_count",
            "API calls by hostname",
            self.host_count.clone()
        );
        registry.register(
            "http_duration",
            "Time it took to fullfil a request",
            self.http_duration.clone()
        );
        registry.register(
            "http_request_total",
            "Requests responses by path",
            self.http_request_total.clone()
        );
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct MetricHostLabel {
    host: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct MetricHttpRequestLabel {
    method: String,
    path:   String,
    status: String,
    agent:  String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct MetricDurationLabel {
    method: String,
    path:   String,
    status: String,
    agent:  String,
}
