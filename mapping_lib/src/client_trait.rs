use starfoundry_lib_gateway::ApiClient;

use crate::MappingApiClientRoute;

pub trait MappingApiClient:
    ApiClient +
    MappingApiClientRoute {}
