#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Class", rename_all = "PascalCase")]
#[cfg_attr(test, derive(PartialEq))]
pub struct MicrosoftOptionDefinition {
    pub name: String,
    pub option_id: u8,
    pub r#type: MicrosoftOptionDefinitionType,
    pub default_value: Option<Vec<String>>,
    pub description: Option<String>,
    pub vendor_class: Option<String>,
    pub multi_valued: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "OptionDefinitions")]
pub struct MicrosoftOptionDefinitions {
    #[serde(rename = "OptionDefinition")]
    pub items: Vec<MicrosoftOptionDefinition>,
}
