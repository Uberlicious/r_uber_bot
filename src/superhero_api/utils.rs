use std::fmt::{Debug, Display};
use std::str::FromStr;

use serde::{Deserialize, Deserializer};

pub fn deserialize_number_from_string_or_default<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de> + Default + Debug,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    enum StringOrInt<T> {
        String(String),
        Number(T),
        Null(),
    }

    match StringOrInt::<T>::deserialize(deserializer)? {
        StringOrInt::String(s) => {
            if s == "null" {
                return Ok(T::default());
            }
            s.parse::<T>().map_err(serde::de::Error::custom)
        }
        StringOrInt::Number(i) => Ok(i),
        StringOrInt::Null() => Ok(T::default()),
    }
}
