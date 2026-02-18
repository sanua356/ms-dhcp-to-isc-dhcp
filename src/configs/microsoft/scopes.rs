#![allow(dead_code)]

use std::net::Ipv4Addr;

use serde::Deserialize;

use crate::validators::validate_string_optional;

#[derive(Debug, Deserialize, PartialEq)]
pub enum MicrosoftScopeStateType {
    Active,
    Inactive,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum MicrosoftScopeType {
    Both,
    Dhcp,
    Bootp,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "IPRange", rename_all = "PascalCase")]
pub struct MicrosoftIPRange {
    pub start_range: Ipv4Addr,
    pub end_range: Ipv4Addr,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "ExclusionRanges")]
pub struct MicrosoftExclusionRanges {
    #[serde(rename = "IPRange")]
    pub items: Vec<MicrosoftIPRange>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Reservation", rename_all = "PascalCase")]
pub struct MicrosoftResevation {
    pub name: Option<String>,
    #[serde(rename = "IPAddress")]
    pub ip_address: Ipv4Addr,
    pub client_id: String,
    pub r#type: MicrosoftScopeType,
    #[serde(deserialize_with = "validate_string_optional")]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Reservations")]
pub struct MicrosoftReservations {
    #[serde(rename = "Reservation")]
    pub items: Vec<MicrosoftResevation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Scope", rename_all = "PascalCase")]
pub struct MicrosoftScopeV4 {
    pub scope_id: Ipv4Addr,
    pub name: String,
    pub subnet_mask: Ipv4Addr,
    pub start_range: Ipv4Addr,
    pub end_range: Ipv4Addr,
    pub lease_duration: String,
    pub state: MicrosoftScopeStateType,
    pub r#type: MicrosoftScopeType,
    pub max_bootp_clients: u32,
    pub nap_enable: bool,
    pub delay: u32,
    #[serde(deserialize_with = "validate_string_optional")]
    pub nap_profile: Option<String>,
    #[serde(deserialize_with = "validate_string_optional")]
    pub description: Option<String>,
    pub activate_policies: bool,
    #[serde(deserialize_with = "validate_string_optional")]
    pub super_scope_name: Option<String>,

    #[serde(rename = "ExclusionRanges")]
    pub exclusion_ranges: Option<MicrosoftExclusionRanges>,
    pub reservations: Option<MicrosoftReservations>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Scopes")]
pub struct MicrosoftScopesV4 {
    #[serde(rename = "Scope")]
    pub items: Vec<MicrosoftScopeV4>,
}
