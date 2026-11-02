use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Deserializer};

/// يحوّل حقل رقمي فارغ "" من HTML Form إلى None، ويحوّل النص الرقمي إلى T.
///
/// Usage:
/// #[serde(default, deserialize_with = "empty_number_as_none")]
/// pub price: Option<i64>,
pub fn empty_number_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OptionNum<T> {
        Num(T),
        Str(String),
        None,
    }

    match OptionNum::<T>::deserialize(deserializer)? {
        OptionNum::Num(n) => Ok(Some(n)),
        OptionNum::Str(s) if s.trim().is_empty() => Ok(None),
        OptionNum::Str(s) => s
            .trim()
            .parse::<T>()
            .map(Some)
            .map_err(serde::de::Error::custom),
        OptionNum::None => Ok(None),
    }
}

/// يحوّل حقل ملاحظات نصي فارغ "" من HTML Form إلى None، ويُرجع النص بعد التنظيف من المسافات.
///
/// Usage:
/// #[serde(default, deserialize_with = "empty_string_as_none")]
/// pub notes: Option<String>,
pub fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt
        .map(|s| s.trim().to_owned())
        .filter(|s| !s.is_empty()))
}