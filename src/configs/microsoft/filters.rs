#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MicrosoftFilterListType {
    Allow,
    Deny,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Filter", rename_all = "PascalCase")]
pub struct MicrosoftFilter {
    pub list: MicrosoftFilterListType,
    pub mac_addresses: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Filters", rename_all = "PascalCase")]
pub struct MicrosoftFilters {
    #[serde(rename = "Filter")]
    pub allow: bool,
    pub deny: bool,

    pub items: Vec<MicrosoftFilter>,
}
