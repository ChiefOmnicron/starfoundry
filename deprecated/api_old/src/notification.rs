use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_api::Credentials;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use utoipa::ToSchema;
use uuid::Uuid;
use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::{with_identity, with_pool};

pub mod create;
pub mod delete;
pub mod error;
pub mod fetch;
pub mod list;
pub mod test_message;
pub mod update;

pub use self::error::*;

pub mod service {
    pub use super::create::*;
    pub use super::delete::*;
    pub use super::fetch::*;
    pub use super::list::*;
    pub use super::test_message::*;
    pub use super::update::*;
}

pub fn api(
    pool:        PgPool,
    base_path:   BoxedFilter<()>,
    credentials: Credentials,
) -> BoxedFilter<(impl Reply,)> {
    let base_path = base_path
        .clone()
        .and(with_pool(pool.clone()))
        .and(with_identity(pool.clone(), credentials.clone()))
        .and(warp::path!("notifications" / ..))
        .boxed();

    let list = base_path
        .clone()
        .and(warp::get())
        .and(warp::path::end())
        .and(warp::query())
        .and_then(service::list_api)
        .boxed();

    let fetch = base_path
        .clone()
        .and(warp::get())
        .and(warp::path!(NotificationUuid))
        .and_then(service::fetch_api)
        .boxed();

    let create = base_path
        .clone()
        .and(warp::post())
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(service::create_api)
        .boxed();

    let update = base_path
        .clone()
        .and(warp::put())
        .and(warp::path!(NotificationUuid))
        .and(warp::body::json())
        .and_then(service::update_api)
        .boxed();

    let delete = base_path
        .clone()
        .and(warp::delete())
        .and(warp::path!(NotificationUuid))
        .and_then(service::delete_api)
        .boxed();

    let test_message = base_path
        .clone()
        .and(warp::post())
        .and(warp::path!("test-message"))
        .and(warp::body::json())
        .and_then(service::test_message_api)
        .boxed();

    list
        .or(fetch)
        .or(create)
        .or(update)
        .or(delete)
        .or(test_message)
        .boxed()
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(transparent)]
#[schema(
    example = json!(Uuid::new_v4()),
    value_type = Uuid,
)]
pub struct NotificationUuid(Uuid);

impl NotificationUuid {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl Deref for NotificationUuid {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for NotificationUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromStr for NotificationUuid {
    type Err = uuid::Error;

    fn from_str(uuid_str: &str) -> Result<Self, Self::Err> {
        Ok(NotificationUuid(Uuid::parse_str(uuid_str)?))
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Notification {
    pub id:     NotificationUuid,
    pub target: NotificationTarget,
    pub url:    String,
    pub name:   String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "NOTIFICATION_TARGET")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationTarget {
    Discord,
    Json,
}
