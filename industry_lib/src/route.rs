mod create;
//mod fetch;
//mod update;
mod misc;

pub use self::create::*;
pub use self::misc::*;
//pub use self::fetch::*;
//pub use self::update::*;

use starfoundry_lib_gateway::ApiClient;

use crate::Result;

pub trait IndustryApiClientRoute: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn route_create(
        &self,
        request: &CreateRoute,
    ) -> Result<CreateRouteResponse> {
        self
            .post(
                "routes",
                request,
            )
            .await
            .map_err(Into::into)
    }

    /*#[allow(async_fn_in_trait)]
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
    }*/
}
