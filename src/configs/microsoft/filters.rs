#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::validators::validate_mac_address_string;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MicrosoftFilterListType {
    Allow,
    Deny,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Filter", rename_all = "PascalCase")]
pub struct MicrosoftFilter {
    pub list: MicrosoftFilterListType,
    #[serde(deserialize_with = "validate_mac_address_string")]
    pub mac_address: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Filters", rename_all = "PascalCase")]
pub struct MicrosoftFilters {
    pub allow: bool,
    pub deny: bool,

    #[serde(rename = "Filter")]
    pub items: Vec<MicrosoftFilter>,
}
