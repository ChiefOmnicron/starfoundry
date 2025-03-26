use starfoundry_libs_eve_api::{Credentials, EveApiClient};
use starfoundry_libs_types::CharacterId;

pub async fn eve_api_client(
    credentials:  Credentials,
    character_id: CharacterId,
) -> Option<EveApiClient> {
    let cache = {
        credentials
            .lock()
            .unwrap()
            .clone()
    };

    if let Ok(client) = cache
        .get((*character_id).into())
        .await {
        Some(client)
    } else {
        tracing::warn!(
            "failed to get valid credentials for {}, skipping",
            character_id
        );
        None
    }
}
