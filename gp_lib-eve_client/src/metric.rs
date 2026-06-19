use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;
use starfoundry_lib_types::CharacterId;

pub struct EveApiClientMetric {
    eve_api_rate_limit: Family<MetricEveRateLimitLabel, Gauge>,
    eve_api_status:     Family<MetricEveStatusLabel, Counter>,
}

impl EveApiClientMetric {
    pub fn new() -> Self {
        Self  {
            eve_api_rate_limit: Family::<MetricEveRateLimitLabel, Gauge>::default(),
            eve_api_status:     Family::<MetricEveStatusLabel, Counter>::default(),
        }
    }

    pub fn set_eve_rate_limit<S: Into<String>>(
        &self,
        group:        S,
        character_id: Option<CharacterId>,
        value:        i64,
    ) {
        self
            .eve_api_rate_limit
            .get_or_create(&MetricEveRateLimitLabel {
                group:        group.into(),
                character_id: character_id.map(|x| x.to_string()),
            })
            .set(value);
    }

    pub fn increase_eve_status(
        &self,
        status_code: String,
        path:        String,
    ) {
        self
            .eve_api_status
            .get_or_create(&MetricEveStatusLabel {
                status_code,
                path,
            })
            .inc();
    }

    pub fn register(
        &self,
        registry: &mut Registry,
    ) {
        registry.register(
            "eve_api_rate_limit",
            "Rate limit by group and characterId",
            self.eve_api_rate_limit.clone()
        );
        registry.register(
            "eve_api_status",
            "Statuses returned by the EVE API",
            self.eve_api_status.clone()
        );
    }
}

impl Default for EveApiClientMetric {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct MetricEveRateLimitLabel {
    group:        String,
    character_id: Option<String>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct MetricEveStatusLabel {
    status_code: String,
    path:        String,
}
