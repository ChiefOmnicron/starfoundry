use sqlx::PgPool;
use std::sync::Arc;

use crate::config::ShopConfig;

#[derive(Clone)]
pub struct AppState {
    pub postgres:       PgPool,

    pub shop_config:    Arc<ShopConfig>,
    pub discord_url:    Arc<String>,
}
