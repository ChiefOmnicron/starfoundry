use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer};
use reqwest::Response;

pub fn from_datetime<'de, D>(
    deserializer: D
) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de> {

    let datetime: String = Deserialize::deserialize(deserializer)?;
    let datetime = NaiveDateTime::parse_from_str(
        &datetime, "%Y-%m-%dT%H:%M:%S%Z"
    )
    .unwrap_or_default();

    Ok(datetime)
}

/// Extract the page header from the give [reqwest::Response].
///
/// # Params
///
/// * `response` -> Response to get the header from
///
/// # Returns
///
/// - If the header is not present a 0 is returned
/// - If the header exists, it will try to parse it, if that fails a 0 is
///   is returned
///
pub fn page_count(response: &Response) -> u16 {
    let headers = response.headers();
    if let Some(x) = headers.get("x-pages") {
        x.to_str()
            .unwrap_or_default()
            .parse::<u16>()
            .unwrap_or_default()
    } else {
        0u16
    }
}

/// Checks if the return has content in it.
/// 
/// # Params
///
/// * `response` -> Response to get the header from
///
/// # Returns
///
/// - False if the header does not exists
/// - False if the content does not contain data
///   True if the content contains data
///
pub fn has_content(
    response: &Response
) -> bool {
    let headers = response.headers();
    if let Some(x) = headers.get("content-length") {
        x.to_str()
            .unwrap_or_default()
            .parse::<u64>()
            .unwrap_or_default() > 0
    } else {
        false
    }
}
