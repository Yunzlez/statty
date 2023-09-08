use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serializer};

const DATE_FORMAT: &'static str = "%Y-%m-%d";

pub fn serialize<S> (
    date: &Option<NaiveDate>,
    serializer: S
) -> Result<S::Ok, S::Error>
where
S: Serializer
{
    if date.is_none() {
        return serializer.serialize_none();
    }
    let s = format!("{}", date.unwrap().format(DATE_FORMAT));
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D> (
    deserializer: D
) -> Result<Option<NaiveDate>, D::Error>
where
D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, DATE_FORMAT)
        .map(|d| Some(d))
        .map_err(serde::de::Error::custom)
}