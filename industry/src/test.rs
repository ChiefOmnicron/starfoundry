use starfoundry_lib_eve_gateway::ApiClient;

#[derive(Clone)]
pub struct TestApiClient;

impl TestApiClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl ApiClient for TestApiClient {
    async fn fetch<T>(
            &self,
            path: impl Into<String>,
        ) -> starfoundry_lib_eve_gateway::Result<T>
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
            _ => serde_json::json!({})
        };

        Ok(
            serde_json::from_value(response).unwrap(),
        )
    }

    async fn post<D, T>(
            &self,
            _path: impl Into<String>,
            _data: D,
        ) -> starfoundry_lib_eve_gateway::Result<T>
        where
            D: std::fmt::Debug + serde::Serialize + Send + Sync,
            T: serde::de::DeserializeOwned {
        unimplemented!()
    }
}
