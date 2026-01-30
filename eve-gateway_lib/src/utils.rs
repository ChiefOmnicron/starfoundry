use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer};

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

