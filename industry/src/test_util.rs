use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_gateway::ApiClient;
use starfoundry_lib_types::TypeId;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

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

        let response = match dbg!(path.into()).as_ref() {
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
            }
            _ => serde_json::json!({})
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
        let response = match dbg!(path.into()).as_ref() {
            "items/bulk" => {
                let data: Vec<TypeId> = serde_json::from_value(serde_json::to_value(&data).unwrap()).unwrap();
                if data.is_empty() {
                    serde_json::json!([])
                } else {
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
                    }])
                }
            }
            _ => serde_json::json!({})
        };

        Ok(
            serde_json::from_value(response).unwrap(),
        )
    }
}
