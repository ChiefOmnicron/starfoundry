use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use starfoundry_lib_types::TypeId;
use std::collections::HashMap;

use crate::{ExternalAppraisal, AppraisalEntry, Error, Persistance, Result};

mod item;
mod response;
mod value;

pub use self::item::*;
pub use self::response::*;
pub use self::value::*;

/// Implementation for [janice](https://janice.e-351.com/)
///
/// Additional documentation [swagger](https://janice.e-351.com/api/rest/docs/index.html)
/// 
pub struct JaniceAppraisal(reqwest::Client);

impl JaniceAppraisal {
    /// Name of the UserAgent ENV
    const USER_AGENT: &'static str    = "JANICE_USER_AGENT";
    /// Name of the ApiKey ENV
    const API_KEY: &'static str       = "JANICE_API_KEY";
    /// Url for creating appraisals
    const APPRAISAL_URL: &'static str = "https://janice.e-351.com/api/rest/v2/appraisal";

    /// Creates a new appraisal client, with required headers.
    /// 
    /// The headers `JANICE_USER_AGENT` and `JANICE_API_KEY` are required,
    /// otherwise this function will error.
    ///
    /// # Error
    ///
    /// Errors if the ENV `JANICE_USER_AGENT` and `JANICE_API_KEY` are not set.
    ///
    /// # Returns
    ///
    /// Rest client to the appraisal page
    ///
    pub fn new() -> Result<Self> {
        let user_agent = std::env::var(Self::USER_AGENT)
            .map_err(|_| Error::MissingEnv(format!("JANICE_{}", Self::USER_AGENT)))?;
        let api_key = std::env::var(Self::API_KEY)
            .map_err(|_| Error::MissingEnv(format!("JANICE_{}", Self::API_KEY)))?;

        let mut headers = HeaderMap::new();
        headers.insert("X-ApiKey", HeaderValue::from_str(&api_key).unwrap());
        headers.insert("Content-Type", HeaderValue::from_static("text/plain"));

        let client = Client::builder()
            .user_agent(user_agent)
            .default_headers(headers)
            .build()
            .map_err(Error::CouldNotConstructClient)?;

        Ok(Self(client))
    }
}

#[async_trait::async_trait]
impl ExternalAppraisal<JaniceResponse> for JaniceAppraisal
where
    Self: Sized,
{
    /// Validates that all required Environment variables are set
    ///
    /// # Error
    ///
    /// Fails when a Environment-Variable is missing
    ///
    /// # Returns
    ///
    /// `Ok`  -> If all Environment-Variables are set
    /// `Err` -> Not all Environment-Variables are set, contains the missing ENV-Name
    ///
    fn validate() -> Result<()> {
        std::env::var(Self::USER_AGENT)
            .map_err(|_| Error::MissingEnv(format!("JANICE_{}", Self::USER_AGENT)))
            .map(drop)?;
        std::env::var(Self::API_KEY)
            .map_err(|_| Error::MissingEnv(format!("JANICE_{}", Self::API_KEY)))
            .map(drop)?;
        Ok(())
    }

    /// Creates a new appraisal
    ///
    /// # Params
    ///
    /// * `persist` -> Determines if the appraisal should be stored
    /// * `entries` -> List of entries to create a appraisal for
    ///                Format: `item_name quantity`
    ///
    /// # Errors
    ///
    /// - When the server is not reachable
    /// - Invalid Format
    ///
    /// # Returns
    ///
    /// Appraisal information
    ///
    async fn create(
        &self,
        persist: Persistance,
        entries: Vec<AppraisalEntry>,
    ) -> Result<JaniceResponse> {
        let entries = entries
            .into_iter()
            .map(|x| {
                if x.type_id == TypeId(16679) {
                    return format!("Fullerides {}", x.quantity)
                }

                format!("{} {}", x.name, x.quantity)
            })
            .collect::<Vec<_>>();

        let mut params = HashMap::new();
        params.insert("persist", persist.to_string());
        params.insert("designation", "appraisal".into());
        params.insert("pricing", "split".into());
        params.insert("pricingVariant", "immediate".into());

        self.0
            .post(Self::APPRAISAL_URL)
            .query(&params)
            .body(entries.join("\n"))
            .send()
            .await
            .map_err(Error::RequestError)?
            .json::<JaniceResponse>()
            .await
            .map_err(Error::RequestError)
    }
}
