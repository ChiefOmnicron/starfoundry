mod create;
mod fetch;
mod update;

pub use self::create::*;
pub use self::fetch::*;
pub use self::update::*;

use starfoundry_lib_gateway::ApiClient;

use crate::{TagUuid, Result};

pub trait IndustryApiClientTag: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn create(
        &self,
        request: &CreateTag,
    ) -> Result<CreateTagResponse> {
        self
            .post_auth(
                "tags",
                request,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn delete(
        &self,
        tag_id: &TagUuid,
    ) -> Result<()> {
        self
            .delete_auth(
                format!("tags/{tag_id}"),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch(
        &self,
        tag_id: &TagUuid,
    ) -> Result<Option<Tag>> {
        self
            .fetch_auth(
                format!("projects/{tag_id}"),
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list(
        &self,
    ) -> Result<Vec<Tag>> {

        self
            .fetch_auth(
                "tags",
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn update(
        &self,
        tag_id:     &TagUuid,
        request:    &UpdateTag,
    ) -> Result<()> {
        self
            .put_auth(
                format!("tags/{tag_id}"),
                request,
            )
            .await
            .map_err(Into::into)
    }
}
