use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_gateway::ApiClient;
use starfoundry_lib_types::{SystemId, TypeId};

#[derive(Clone)]
pub struct EveGatewayTestApiClient;

impl EveGatewayTestApiClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl EveGatewayApiClient for EveGatewayTestApiClient {}

impl ApiClient for EveGatewayTestApiClient {
    async fn fetch<T>(
            &self,
            path: impl Into<String>,
        ) -> starfoundry_lib_gateway::error::Result<T>
        where
            T: serde::de::DeserializeOwned {

        let path = path.into();
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
            "items/35892" => {
                serde_json::json!({
                    "base_price": null,
                    "category_id": 6,
                    "group_id": 30,
                    "meta_group_id": null,
                    "name": "Standup Market Hub I",
                    "repackaged": 10000000,
                    "type_id": 35892,
                    "volume": 100000000
                })
            },
            "structures/rigs/46497" => {
                serde_json::json!({
                    "item": {
                        "base_price": null,
                        "category_id": 6,
                        "group_id": 30,
                        "meta_group_id": null,
                        "name": "Standup L-Set Reactor Efficiency II",
                        "repackaged": 10000000,
                        "type_id": 46497,
                        "volume": 100000000
                    },
                    "excludes": [
                        46496
                    ],
                    "material": 2.4,
                    "time": 24,
                    "category_groups": [
                        // TODO: insert categories
                        0
                    ]
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
                dbg!(path);
                serde_json::json!({})
            }
        };

        Ok(
            serde_json::from_value(response).unwrap(),
        )
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
            "items/bulk" => {
                let data: Vec<TypeId> = serde_json::from_value(serde_json::to_value(&data).unwrap()).unwrap();
                if data.is_empty() {
                    serde_json::json!([])
                } else {
                    dbg!(&data);
                    serde_json::json!([{
                        "base_price": null,
                        "category_id": 6,
                        "group_id": 30,
                        "meta_group_id": null,
                        "name": "Nitrogen Fuel Block",
                        "repackaged": 10000000,
                        "type_id": 4051,
                        "volume": 100000000
                    }, {
                        "base_price": null,
                        "category_id": 6,
                        "group_id": 30,
                        "meta_group_id": null,
                        "name": "Hydrogen Fuel Block",
                        "repackaged": 10000000,
                        "type_id": 4246,
                        "volume": 100000000
                    }, {
                        "base_price": null,
                        "category_id": 6,
                        "group_id": 30,
                        "meta_group_id": null,
                        "name": "Helium Fuel Block",
                        "repackaged": 10000000,
                        "type_id": 4247,
                        "volume": 100000000
                    }, {
                        "base_price": null,
                        "category_id": 6,
                        "group_id": 30,
                        "meta_group_id": null,
                        "name": "Oxygen Fuel Block",
                        "repackaged": 10000000,
                        "type_id": 4312,
                        "volume": 100000000
                    }, {
                        "base_price": null,
                        "category_id": 6,
                        "group_id": 1321,
                        "meta_group_id": null,
                        "name": "Standup Market Hub I",
                        "repackaged": 10000000,
                        "type_id": 35892,
                        "volume": 100000000
                    }])
                }
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
