use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serializer};

const DATE_FORMAT: &'static str = "%Y-%m-%d";

pub fn serialize<S> (
    date: &NaiveDate,
    serializer: S
) -> Result<S::Ok, S::Error>
where
S: Serializer
{
    let s = format!("{}", date.format(DATE_FORMAT));
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D> (
    deserializer: D
) -> Result<NaiveDate, D::Error>
where
D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, DATE_FORMAT)
        .map_err(serde::de::Error::custom)
}