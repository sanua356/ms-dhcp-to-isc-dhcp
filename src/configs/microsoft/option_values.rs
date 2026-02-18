#![allow(dead_code)]

use serde::Deserialize;

use crate::validators::validate_string_optional;

#[derive(Debug, Deserialize)]
#[serde(rename = "OptionValue", rename_all = "PascalCase")]
pub struct MicrosoftOptionValue {
    pub option_id: u32,
    pub value: Option<Vec<String>>,
    #[serde(deserialize_with = "validate_string_optional")]
    pub vendor_class: Option<String>,
    #[serde(deserialize_with = "validate_string_optional")]
    pub user_class: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "OptionValues")]
pub struct MicrosoftOptionValues {
    #[serde(rename = "OptionValue")]
    pub items: Vec<MicrosoftOptionValue>,
}
