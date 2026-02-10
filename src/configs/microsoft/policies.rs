#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use super::option_values::MicrosoftOptionValues;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MicrosoftPolicyConditionType {
    AND,
    OR,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Policy", rename_all = "PascalCase")]
pub struct MicrosoftPolicy {
    pub name: String,
    pub processing_order: i32,
    pub enabled: bool,
    pub condition: MicrosoftPolicyConditionType,
    pub description: Option<bool>,
    pub dns_suffix: Option<String>,
    pub option_values: Option<MicrosoftOptionValues>,

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Policies")]
pub struct MicrosoftPolicies {
    #[serde(rename = "Policy")]
    pub items: Vec<MicrosoftPolicy>,
}
