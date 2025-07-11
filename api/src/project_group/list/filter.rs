use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Default, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ProjectGroupFilter {
    #[serde(default)]
    #[param(
        example = json!("ProjectGroup1337"),
        required = false,
    )]
    pub name:  Option<String>,

    #[serde(default)]
    pub owner: Option<bool>,
}
