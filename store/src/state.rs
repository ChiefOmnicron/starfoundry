use axum::extract::FromRef;
use jsonwebtoken::DecodingKey;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayState;
use std::sync::Arc;

use crate::config::ShopConfig;

#[derive(Clone)]
pub struct AppState {
    pub postgres:       PgPool,

    pub shop_config:    Arc<ShopConfig>,
    pub discord_url:    Arc<String>,

    pub decoding_key:   Arc<DecodingKey>,
}

impl FromRef<AppState> for EveGatewayState {
    fn from_ref(input: &AppState) -> Self {
        EveGatewayState {
            decoding_key: input.decoding_key.clone(),
        }
    }
}
