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
    async fn tag_create(
        &self,
        request: &CreateTag,
    ) -> Result<CreateTagResponse> {
        self
            .post(
                "tags",
                request,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn tag_delete(
        &self,
        tag_id: &TagUuid,
    ) -> Result<()> {
        self
            .delete(
                format!("tags/{tag_id}"),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn tag_fetch(
        &self,
        tag_id: &TagUuid,
    ) -> Result<Option<Tag>> {
        self
            .fetch(
                format!("projects/{tag_id}"),
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn tag_list(
        &self,
    ) -> Result<Vec<Tag>> {

        self
            .fetch(
                "tags",
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn tag_update(
        &self,
        tag_id:     &TagUuid,
        request:    &UpdateTag,
    ) -> Result<()> {
        self
            .put(
                format!("tags/{tag_id}"),
                request,
            )
            .await
            .map_err(Into::into)
    }
}
