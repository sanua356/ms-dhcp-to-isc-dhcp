use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "lowercase")]
pub enum ISCOptionDefinitionType {
    Boolean,
    #[serde(rename = "integer 8")]
    Int8,
    #[serde(rename = "integer 16")]
    Int16,
    #[serde(rename = "integer 32")]
    Int32,
    #[serde(rename = "unsigned integer 8")]
    UInt8,
    #[serde(rename = "unsigned integer 16")]
    UInt16,
    #[serde(rename = "unsigned integer 32")]
    UInt32,
    #[serde(rename = "ip-address")]
    IPv4Address,
    #[serde(rename = "ip6-address")]
    IPv6Address,
    Text,
    #[serde(rename = "string")]
    DataString,
    #[serde(rename = "domain-list")]
    DomainList,
    Encapsulate,
    #[serde(rename = "array of")]
    Arrays(Box<ISCOptionDefinitionType>),
    Records(Vec<ISCOptionDefinitionType>),
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ISCOptionDefinition {
    pub code: u8,
    pub name: String,
    pub r#type: ISCOptionDefinitionType,

    pub vendor_class: Option<String>,
}
