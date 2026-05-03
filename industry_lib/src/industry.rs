mod calculation;
mod stock;

use crate::Result;

pub use self::calculation::*;
pub use self::stock::*;

use axum::http::{HeaderMap, HeaderValue};
use reqwest::header::HOST;
use starfoundry_lib_gateway::{ApiClient, HEADER_CHARACTER_ID};
use starfoundry_lib_types::CharacterId;

pub trait IndustryApiClientIndustry: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn calculation(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        request:        &TmpRequest,
    ) -> Result<Vec<TmpResponse>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .post_auth(
                "industry/calculation",
                request,
                headers,
            )
            .await
            .map_err(Into::into)
    }
}
