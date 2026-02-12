use serde::{Deserialize, Deserializer};

use crate::constants::{BYTES_WITH_PREFIX_REGEXP, MAC_ADDRESS_REGEXP};

pub fn validate_hex_string_optional<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(ref value) if !BYTES_WITH_PREFIX_REGEXP.is_match(value) => Err(
            serde::de::Error::custom(format!("Invalid format for hex string: {}", value)),
        ),
        _ => Ok(opt),
    }
}

pub fn validate_mac_address_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if !MAC_ADDRESS_REGEXP.is_match(&s) {
        Err(serde::de::Error::custom(format!(
            "Invalid format for MAC address string: {}",
            s
        )))
    } else {
        Ok(s)
    }
}
