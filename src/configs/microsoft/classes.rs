#![allow(dead_code)]

use serde::Deserialize;

use crate::validators::{validate_hex_string_optional, validate_string_optional};

#[derive(Debug, Deserialize, PartialEq)]
pub enum MicrosoftClassType {
    User,
    Vendor,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Class", rename_all = "PascalCase")]
pub struct MicrosoftClass {
    pub name: String,
    pub r#type: MicrosoftClassType,
    #[serde(deserialize_with = "validate_hex_string_optional")]
    pub data: Option<String>,
    #[serde(default, deserialize_with = "validate_string_optional")]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Classes")]
pub struct MicrosoftClasses {
    #[serde(rename = "Class")]
    pub items: Vec<MicrosoftClass>,
}
