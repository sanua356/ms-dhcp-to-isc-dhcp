#![allow(dead_code)]

use serde::Deserialize;

mod classes;
mod filters;
mod option_definitions;
mod option_values;
mod policies;
mod scopes;

pub use classes::*;
pub use filters::*;
pub use option_definitions::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MicrosoftIPv4 {
    pub conflict_detection_attempts: i32,
    pub nap_enabled: bool,
    pub nps_unreachable_action: String,
    pub activate_policies: bool,

    pub classes: Option<classes::MicrosoftClasses>,
    pub option_definitions: Option<option_definitions::MicrosoftOptionDefinitions>,
    pub option_values: Option<option_values::MicrosoftOptionValues>,
    pub filters: Option<filters::MicrosoftFilters>,
    pub scopes: Option<scopes::MicrosoftScopesV4>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MicrosoftDHCP {
    pub major_version: u32,
    pub minor_version: u32,

    #[serde(rename = "IPv4")]
    pub ipv4: MicrosoftIPv4,
}
