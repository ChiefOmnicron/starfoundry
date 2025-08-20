use std::fmt::Debug;

use serde::de::DeserializeOwned;
use starfoundry_lib_eve_api::{Credentials, EveApiClient};
use starfoundry_lib_types::CharacterId;

use crate::error::{Error, Result};
use crate::task::Task;

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

pub fn additional_data<T>(
    task: &mut Task,
) -> Result<T>
    where
        T: Debug + DeserializeOwned {

    if let Some(x) = task.additional_data::<T>() {
        Ok(x)
    } else {
        tracing::error!(
            "additional data was empty, but was expected to be filled, task: {:?}",
            task.task
        );
        task.add_error("additional data was empty");
        Err(Error::NoOp)
    }
}
