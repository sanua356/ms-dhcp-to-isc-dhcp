#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MicrosoftOptionDefinitionType {
    String,
    IPv4Address,
    BinaryData,
    EncapsulatedData,
    Byte,
    Word,
    DWord,
    DWordDWord,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Class", rename_all = "PascalCase")]
pub struct MicrosoftOptionDefinition {
    pub name: String,
    pub option_id: u32,
    pub r#type: MicrosoftOptionDefinitionType,
    pub default_value: Option<String>,
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
