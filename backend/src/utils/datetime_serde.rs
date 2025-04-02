use std::str::FromStr;
use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Deserializer};

pub fn option_datetime_deserializer<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    // This handles the case when the field is null/missing
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => {
            // Convert the string to DateTime and wrap in Some
            match DateTime::from_str(&s) {
                Ok(dt) => Ok(Some(dt)),
                Err(e) => Err(serde::de::Error::custom(e))
            }
        }
        None => Ok(None),
    }
}