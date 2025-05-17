use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize)]
pub struct Config {
    /// Url where the definition is located, this one will be used to generate the clients
    pub definition_url: String,

    /// List of clients that should be generated
    pub clients: HashMap<String, ClientConfiguration>,
}

#[derive(Debug, Deserialize)]
pub struct ClientConfiguration {
    /// Language of the target client
    pub language: Language,
    /// Output path of the client
    pub path:     ClientPath,
    pub modules:  Vec<ClientModule>,
}

#[derive(Debug, Deserialize)]
pub struct ClientModule {
    /// Tag that should be used to identify the module
    pub tag:     String,
    /// Adds methods for request caching, instead of performing a request to the server each time
    pub caching: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ClientPath(pub String);

impl ClientPath {
    pub fn save(
        &self,
        file_name: &str,
        content:   String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(format!("{}/{}", self.0, file_name))?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    TypeScript
}

impl Language {
    pub fn file_ending(&self) -> String {
        match self {
            Self::TypeScript => ".ts",
        }.into()
    }
}

pub fn config() -> Config {
    toml::from_str(r#"
        # also allow file://definition.json"
        definition_url = 'http://localhost:10101/definition'

        [clients.typescript-web]
        path = "../typescript"
        language = "typescript"

        [[clients.typescript-web.modules]]
        tag = "projects"
        caching = true
    "#).unwrap()
}
