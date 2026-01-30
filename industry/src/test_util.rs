use axum::http::HeaderMap;
use serde::Serialize;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, EveGatewayApiClientAsset, EveGatewayApiClientIndustry, EveGatewayApiClientItem};
use starfoundry_lib_gateway::ApiClient;
use starfoundry_lib_types::{SystemId, TypeId};
use starfoundry_lib_eve_gateway::market::EveGatewayApiClientMarket;
use starfoundry_lib_eve_gateway::contract::EveGatewayApiClientContract;
use starfoundry_lib_market::{MarketApiClient, MarketApiClientPrice};

#[derive(Clone)]
pub struct EveGatewayTestApiClient;

impl EveGatewayTestApiClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl EveGatewayApiClient for EveGatewayTestApiClient {}
impl EveGatewayApiClientAsset for EveGatewayTestApiClient {}
impl EveGatewayApiClientContract for EveGatewayTestApiClient {}
impl EveGatewayApiClientIndustry for EveGatewayTestApiClient {}
impl EveGatewayApiClientItem for EveGatewayTestApiClient {}
impl EveGatewayApiClientMarket for EveGatewayTestApiClient {}

#[derive(Clone)]
pub struct MarketTestApiClient;

impl MarketTestApiClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl MarketApiClient for MarketTestApiClient {}
impl MarketApiClientPrice for MarketTestApiClient {}

impl ApiClient for EveGatewayTestApiClient {
    async fn fetch<Q: Serialize, T>(
            &self,
            path:   impl Into<String>,
            _query: &Q,
        ) -> starfoundry_lib_gateway::error::Result<T>
        where
            T: serde::de::DeserializeOwned {

        let path = dbg!(path.into());
        let response = match path.as_ref() {
            "characters/1" |
            "characters/2" => {
                serde_json::json!({
                    "alliance_id": 1,
                    "alliance_name": "SomeAlliance",
                    "character_id": 1,
                    "character_name": "SomeCharacter",
                    "corporation_id": 1,
                    "corporation_name": "SomeCorporation",
                })
            },
            "items/21027" => {
                serde_json::json!({
                    "base_price": null,
                    "meta_group_id": null,
                    "name": "Capital Cargo Bay",
                    "repackaged": 10000000,
                    "type_id": 21027,
                    "volume": 100000000,
                    "category": {
                        "category_id": 0,
                        "name": "#System"
                    },
                    "group": {
                        "group_id": 0,
                        "category_id": 0,
                        "name": "#System"
                    }
                })
            },
            "items/35892" => {
                serde_json::json!({
                    "base_price": null,
                    "meta_group_id": null,
                    "name": "Standup Market Hub I",
                    "repackaged": 10000000,
                    "type_id": 35892,
                    "volume": 100000000,
                    "category": {
                        "category_id": 0,
                        "name": "#System"
                    },
                    "group": {
                        "group_id": 0,
                        "category_id": 0,
                        "name": "#System"
                    }
                })
            },
            "structures/rigs/46497" => {
                serde_json::json!({
                    "item": {
                        "base_price": null,
                        "meta_group_id": null,
                        "name": "Standup L-Set Reactor Efficiency II",
                        "repackaged": 10000000,
                        "type_id": 46497,
                        "volume": 100000000,
                        "category": {
                            "category_id": 0,
                            "name": "#System"
                        },
                        "group": {
                            "group_id": 0,
                            "category_id": 0,
                            "name": "#System"
                        }
                    },
                    "excludes": [
                        46496
                    ],
                    "material": 2.4,
                    "time": 24,
                    "categories": [{
                        "category_id": 0,
                        "name": "#System"
                    }],
                    "groups": [{
                        "group_id": 0,
                        "category_id": 0,
                        "name": "#System"
                    }]
                })
            },
            "universe/systems/30004759" => {
                serde_json::json!({
                    "region_id": 10000060,
                    "constellation_id": 20000696,
                    "system_id": 30004759,
                    "region_name": "Delve",
                    "constellation_name": "O-EIMK",
                    "system_name": "1DQ1-A",
                    "security": -0.385782,
                    "security_str": "NULLSEC"
                })
            },
            _ => {
                serde_json::json!({})
            }
        };

        Ok(
            serde_json::from_value(response).unwrap(),
        )
    }

    async fn fetch_auth<Q: Serialize, T>(
        &self,
        _path:    impl Into<String>,
        _query:   &Q,
        _headers: HeaderMap,
    ) -> starfoundry_lib_gateway::error::Result<T>
    where
        T: serde::de::DeserializeOwned {
        unimplemented!()
    }

    async fn post<D, T>(
        &self,
        path: impl Into<String>,
        data: D,
    ) -> starfoundry_lib_gateway::error::Result<T>
        where
            D: std::fmt::Debug + serde::Serialize + Send + Sync,
            T: serde::de::DeserializeOwned {

        let path = path.into();
        let response = match path.as_ref() {
            "items" => {
                let mut result = Vec::new();

                let type_ids: Vec<TypeId> = serde_json::from_value(serde_json::to_value(&data).unwrap()).unwrap();
                for type_id in type_ids {
                    if type_id == TypeId(21027) {
                        result.push(serde_json::json!({
                            "base_price": null,
                            "meta_group_id": null,
                            "name": "Capital Cargo Bay",
                            "repackaged": 10000000,
                            "type_id": 21027,
                            "volume": 100000000,
                            "category": {
                                "category_id": 0,
                                "name": "#System"
                            },
                            "group": {
                                "group_id": 0,
                                "category_id": 0,
                                "name": "#System"
                            }
                        }));
                    } else if type_id == TypeId(20185) {
                        result.push(serde_json::json!({
                            "base_price": null,
                            "meta_group_id": null,
                            "name": "Charon",
                            "repackaged": 10000000,
                            "type_id": 20185,
                            "volume": 100000000,
                            "category": {
                                "category_id": 0,
                                "name": "#System"
                            },
                            "group": {
                                "group_id": 0,
                                "category_id": 0,
                                "name": "#System"
                            }
                        }));
                    } else if type_id == TypeId(4051) {
                        result.push(serde_json::json!({
                            "base_price": null,
                            "meta_group_id": null,
                            "name": "Nitrogen Fuel Block",
                            "repackaged": 10000000,
                            "type_id": 4051,
                            "volume": 100000000,
                            "category": {
                                "category_id": 0,
                                "name": "#System"
                            },
                            "group": {
                                "group_id": 0,
                                "category_id": 0,
                                "name": "#System"
                            }
                        }));
                    } else if type_id == TypeId(4246) {
                        result.push(serde_json::json!({
                            "base_price": null,
                            "meta_group_id": null,
                            "name": "Hydrogen Fuel Block",
                            "repackaged": 10000000,
                            "type_id": 4246,
                            "volume": 100000000,
                            "category": {
                                "category_id": 0,
                                "name": "#System"
                            },
                            "group": {
                                "group_id": 0,
                                "category_id": 0,
                                "name": "#System"
                            }
                        }));
                    } else if type_id == TypeId(4247) {
                        result.push(serde_json::json!({
                            "base_price": null,
                            "meta_group_id": null,
                            "name": "Helium Fuel Block",
                            "repackaged": 10000000,
                            "type_id": 4247,
                            "volume": 100000000,
                            "category": {
                                "category_id": 0,
                                "name": "#System"
                            },
                            "group": {
                                "group_id": 0,
                                "category_id": 0,
                                "name": "#System"
                            }
                        }));
                    } else if type_id == TypeId(4312) {
                        result.push(serde_json::json!({
                            "base_price": null,
                            "meta_group_id": null,
                            "name": "Oxygen Fuel Block",
                            "repackaged": 10000000,
                            "type_id": 4312,
                            "volume": 100000000,
                            "category": {
                                "category_id": 0,
                                "name": "#System"
                            },
                            "group": {
                                "group_id": 0,
                                "category_id": 0,
                                "name": "#System"
                            }
                        }));
                    } else if type_id == TypeId(35892) {
                        result.push(serde_json::json!({
                            "base_price": null,
                            "meta_group_id": null,
                            "name": "Standup Market Hub I",
                            "repackaged": 10000000,
                            "type_id": 35892,
                            "volume": 100000000,
                            "category": {
                                "category_id": 0,
                                "name": "#System"
                            },
                            "group": {
                                "group_id": 0,
                                "category_id": 0,
                                "name": "#System"
                            }
                        }));
                    }
                }

                serde_json::json!(result)
            },
            "universe/systems" => {
                let data: Vec<SystemId> = serde_json::from_value(serde_json::to_value(&data).unwrap()).unwrap();
                if data.is_empty() {
                    serde_json::json!([])
                } else {
                    serde_json::json!([{
                        "region_id": 10000060,
                        "constellation_id": 20000696,
                        "system_id": 30004759,
                        "region_name": "Delve",
                        "constellation_name": "O-EIMK",
                        "system_name": "1DQ1-A",
                        "security": -0.385782,
                        "security_str": "NULLSEC"
                    }])
                }
            },
            _ => {
                dbg!(path, data);
                serde_json::json!({})
            }
        };

        Ok(
            serde_json::from_value(response).unwrap(),
        )
    }
}

impl ApiClient for MarketTestApiClient {
    async fn fetch<Q: Serialize, T>(
            &self,
            path:  impl Into<String>,
            query: &Q,
        ) -> starfoundry_lib_gateway::Result<T>
        where
            T: serde::de::DeserializeOwned {
        unimplemented!()
    }

    async fn fetch_auth<Q: Serialize, T>(
            &self,
            path:    impl Into<String>,
            query:   &Q,
            headers: HeaderMap,
        ) -> starfoundry_lib_gateway::Result<T>
        where
            T: serde::de::DeserializeOwned {
        unimplemented!()
    }

    async fn post<D, T>(
            &self,
            path: impl Into<String>,
            data: D,
        ) -> starfoundry_lib_gateway::Result<T>
        where
            D: std::fmt::Debug + Serialize + Send + Sync,
            T: serde::de::DeserializeOwned {
        unimplemented!()
    }
}
