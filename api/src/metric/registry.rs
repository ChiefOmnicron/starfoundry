use once_cell::sync::Lazy;
use prometheus_client::encoding::{EncodeLabelSet, EncodeLabelValue};
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::histogram::Histogram;
use prometheus_client::registry::Registry;
use regex::Regex;

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelValue)]
pub enum Method {
    Delete,
    Get,
    Post,
    Put,
    Undefined,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct RouteLabels {
    pub method: Method,
    pub status: u16,
    pub route:  String,
}

#[derive(Clone, Debug)]
pub struct Metric {
    route_count:    Family<RouteLabels, Counter>,
    route_status:   Family<RouteLabels, Counter>,
    route_duration: Family<RouteLabels, Histogram>,
}

impl Metric {
    pub fn new() -> Self {
        Self {
            route_count: Family::new_with_constructor(|| {
                Counter::default()
            }),
            route_status: Family::new_with_constructor(|| {
                Counter::default()
            }),
            route_duration: Family::new_with_constructor(|| {
                Histogram::new(vec![
                    0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
                ].into_iter())
            }),
        }
    }

    pub fn register(
        &self,
        registry: &mut Registry,
    ) {
        registry.register(
            "routes_count",
            "Number of times a route was called",
            self.route_count.clone(),
        );
        registry.register(
            "routes_status",
            "Count of different statuscodes",
            self.route_status.clone(),
        );
        registry.register(
            "routes_duration",
            "Time it took for a route to answer",
            self.route_duration.clone(),
        );
    }

    pub fn inc_route_count(
        &self,
        method: &warp::http::Method,
        status: &warp::http::StatusCode,
        path:   &str,
    ) {
        let method = from_warp_method(method);
        let status = status.as_u16();
        let route = sanitize_path(path);

        self.route_count.get_or_create(
            &RouteLabels {
                method,
                status,
                route,
            }
        ).inc();
    }

    pub fn add_route_duration(
        &self,
        method:   &warp::http::Method,
        status:   &warp::http::StatusCode,
        path:     &str,
        duration: f64,
    ) {
        let method = from_warp_method(method);
        let status = status.as_u16();
        let route = sanitize_path(path);

        self.route_duration.get_or_create(
            &RouteLabels {
                method,
                status,
                route,
            }
        ).observe(duration);
    }
}

fn from_warp_method(
    method: &warp::http::Method,
) -> Method {
    match method.as_str() {
        "DELETE" => Method::Delete,
        "GET"    => Method::Get,
        "POST"   => Method::Post,
        "PUT"    => Method::Put,
        _        => Method::Undefined,
    }
}

static REGEX_APPRAISAL_CODE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\/appraisal\/[0-9a-zA-Z]{10}").unwrap());
static REGEX_UUID: Lazy<Regex> = Lazy::new(|| Regex::new(r"[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}").unwrap());
fn sanitize_path(
    path: &str,
) -> String {
    replace_uuid(
        &replace_appraisal_code(
            path
        )
    )
}

fn replace_appraisal_code(
    path: &str
) -> String {
    REGEX_APPRAISAL_CODE.replace_all(path, "/appraisal/<code>").to_string()
}

fn replace_uuid(
    path: &str
) -> String {
    REGEX_UUID.replace_all(path, "<uuid>").to_string()
}
