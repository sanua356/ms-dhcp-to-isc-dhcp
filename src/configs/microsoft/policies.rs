#![allow(clippy::upper_case_acronyms)]

use std::fmt::Display;

use serde::Deserialize;

use super::{option_values::MicrosoftOptionValues, scopes::MicrosoftIPRange};

use crate::validators::validate_string_optional;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum MicrosoftPolicyConditionType {
    AND,
    OR,
}

impl Display for MicrosoftPolicyConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MicrosoftPolicyConditionType::AND => f.write_str("and"),
            MicrosoftPolicyConditionType::OR => f.write_str("or"),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "IPRanges")]
pub struct MicrosoftIPRanges {
    #[serde(rename = "IPRange")]
    pub items: Vec<MicrosoftIPRange>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "Policy", rename_all = "PascalCase")]
pub struct MicrosoftPolicy {
    pub name: String,
    pub processing_order: i32,
    pub enabled: bool,
    pub condition: MicrosoftPolicyConditionType,
    #[serde(default, deserialize_with = "validate_string_optional")]
    pub description: Option<String>,
    #[serde(default, deserialize_with = "validate_string_optional")]
    pub dns_suffix: Option<String>,
    pub option_values: Option<MicrosoftOptionValues>,
    #[serde(rename = "IPRanges")]
    pub ip_ranges: Option<MicrosoftIPRanges>,

    pub vendor_class: Option<Vec<String>>,
    pub user_class: Option<Vec<String>>,
    pub mac_address: Option<Vec<String>>,
    pub client_id: Option<Vec<String>>,
    pub relay_agent: Option<Vec<String>>,
    pub circuit_id: Option<Vec<String>>,
    pub remote_id: Option<Vec<String>>,
    pub subscriber_id: Option<Vec<String>>,
    pub fqdn: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Policies")]
pub struct MicrosoftPolicies {
    #[serde(rename = "Policy")]
    pub items: Vec<MicrosoftPolicy>,
}
