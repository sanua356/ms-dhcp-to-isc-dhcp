#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MicrosoftClassType {
    User,
    Vendor,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Class", rename_all = "PascalCase")]
pub struct MicrosoftClass {
    pub name: String,
    pub r#type: MicrosoftClassType,
    pub data: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Classes")]
pub struct MicrosoftClasses {
    #[serde(rename = "Class")]
    pub items: Vec<MicrosoftClass>,
}
