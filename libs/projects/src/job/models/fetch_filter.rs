use serde::Deserialize;
use starfoundry_lib_types::TypeId;
use utoipa::{IntoParams, ToSchema};

#[derive(Clone, Debug, Default, Deserialize, ToSchema, IntoParams)]
pub struct FetchJobFilter {
    #[serde(default)]
    pub type_id: Option<TypeId>,

    #[serde(default)]
    pub grouped: bool,
}
