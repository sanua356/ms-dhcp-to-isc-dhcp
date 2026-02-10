#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "OptionValue", rename_all = "PascalCase")]
pub struct MicrosoftOptionValue {
    pub option_id: u32,
    pub value: Option<Vec<String>>,
    pub vendor_class: Option<String>,
    pub user_class: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "OptionValues")]
pub struct MicrosoftOptionValues {
    #[serde(rename = "OptionValue")]
    pub items: Vec<MicrosoftOptionValue>,
}
