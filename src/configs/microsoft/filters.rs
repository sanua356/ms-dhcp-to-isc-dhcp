#![allow(dead_code)]

use serde::Deserialize;

use crate::validators::{validate_mac_address_string, validate_string_optional};

#[derive(Debug, Deserialize, PartialEq)]
pub enum MicrosoftFilterListType {
    Allow,
    Deny,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Filter", rename_all = "PascalCase")]
pub struct MicrosoftFilter {
    pub list: MicrosoftFilterListType,
    #[serde(deserialize_with = "validate_mac_address_string")]
    pub mac_address: String,
    #[serde(deserialize_with = "validate_string_optional")]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Filters", rename_all = "PascalCase")]
pub struct MicrosoftFilters {
    pub allow: bool,
    pub deny: bool,

    #[serde(rename = "Filter")]
    pub items: Vec<MicrosoftFilter>,
}
