use serde::Deserialize;
use utoipa::ToSchema;


#[derive(Debug, Deserialize, ToSchema)]
pub struct ProjectGroupFilter {
    #[serde(default = "default_read")]
    pub structures: String,
    #[serde(default = "default_read")]
    pub projects:   String,
}

fn default_read() -> String {
    return "READ,WRITE".into();
}

impl Default for ProjectGroupFilter {
    fn default() -> Self {
        Self {
            structures: default_read(),
            projects:   default_read(),
        }
    }
}
