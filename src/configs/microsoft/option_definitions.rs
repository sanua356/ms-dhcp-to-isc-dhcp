#![allow(dead_code)]

use serde::Deserialize;

use crate::validators::validate_string_optional;

#[derive(Debug, Deserialize, PartialEq)]
pub enum MicrosoftOptionDefinitionType {
    String,
    IPv4Address,
    IPv6Address,
    BinaryData,
    EncapsulatedData,
    Byte,
    Word,
    DWord,
    DWordDWord,
}

// Numeric types (All unsigned)
// Byte = 1 byte
// Word = 2 bytes
// DWord = 4 bytes
// DWordDWord = 8 bytes

#[derive(Debug, Deserialize)]
#[serde(rename = "Class", rename_all = "PascalCase")]
#[cfg_attr(test, derive(PartialEq))]
pub struct MicrosoftOptionDefinition {
    pub name: String,
    pub option_id: u8,
    pub r#type: MicrosoftOptionDefinitionType,
    pub default_value: Option<Vec<String>>,
    #[serde(deserialize_with = "validate_string_optional")]
    pub description: Option<String>,
    #[serde(deserialize_with = "validate_string_optional")]
    pub vendor_class: Option<String>,
    pub multi_valued: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "OptionDefinitions")]
pub struct MicrosoftOptionDefinitions {
    #[serde(rename = "OptionDefinition")]
    pub items: Vec<MicrosoftOptionDefinition>,
}
