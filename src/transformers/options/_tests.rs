#![allow(dead_code)]

use std::sync::LazyLock;

#[cfg(test)]
use crate::{
    configs::isc::{ISCOption, ISCOptionDefinitionType},
    constants::GLOBAL_ENCAPSULATED_CLASS_NAME,
};

#[cfg(test)]
pub static OPTION_DEFINITIONS_XML_TEST_TEMPLATE: &str = r#"
<OptionDefinition>
  <Name>Subnet Mask</Name>
  <OptionId>1</OptionId>
  <Type>IPv4Address</Type>
  <MultiValued>false</MultiValued>
  <DefaultValue>0.0.0.0</DefaultValue>
  <Description>Subnet mask in network byte order</Description>
  <VendorClass />
</OptionDefinition>
<OptionDefinition>
  <Name>testoption</Name>
  <OptionId>1</OptionId>
  <Type>IPv4Address</Type>
  <MultiValued>true</MultiValued>
  <DefaultValue>0.0.0.0</DefaultValue>
  <Description>Subnet mask in network byte order</Description>
  <VendorClass>TEST_VENDOR_CLASS</VendorClass>
</OptionDefinition>
<OptionDefinition>
  <Name>encapsulated-global</Name>
  <OptionId>188</OptionId>
  <Type>EncapsulatedData</Type>
  <MultiValued>false</MultiValued>
  <DefaultValue>0x00</DefaultValue>
  <Description />
  <VendorClass />
</OptionDefinition>
<OptionDefinition>
  <Name>encapsulated</Name>
  <OptionId>189</OptionId>
  <Type>EncapsulatedData</Type>
  <MultiValued>false</MultiValued>
  <DefaultValue>0x00</DefaultValue>
  <Description />
  <VendorClass>TEST_VENDOR_CLASS</VendorClass>
</OptionDefinition>
"#;

#[cfg(test)]
pub static OPTIONS_XML_TEST_TEMPLATE: &str = r#"
<OptionValue>
  <OptionId>1</OptionId>
  <Value>1.2.3.4</Value>
  <VendorClass />
  <UserClass />
</OptionValue>
<OptionValue>
  <OptionId>1</OptionId>
  <Value>1.2.3.4</Value>
  <Value>5.6.7.8</Value>
  <VendorClass>TEST_VENDOR_CLASS</VendorClass>
  <UserClass />
</OptionValue>
<OptionValue>
  <OptionId>188</OptionId>
  <Value>0x45</Value>
  <Value>0x46</Value>
  <Value>0x47</Value>
  <VendorClass />
  <UserClass />
</OptionValue>
<OptionValue>
  <OptionId>189</OptionId>
  <Value>0x55</Value>
  <Value>0x56</Value>
  <Value>0x57</Value>
  <VendorClass>TEST_VENDOR_CLASS</VendorClass>
  <UserClass />
</OptionValue>
"#;

#[cfg(test)]
pub static OPTIONS_ISC_TEST_TEMPLATE: LazyLock<Vec<ISCOption>> = LazyLock::new(|| {
    vec![
        ISCOption {
            name: String::from("subnet-mask"),
            space: None,
            r#type: ISCOptionDefinitionType::IPv4Address,
            value: vec![String::from("1.2.3.4")],
        },
        ISCOption {
            name: String::from("testoption"),
            space: Some(String::from("test-vendor-class")),
            r#type: ISCOptionDefinitionType::IPv4Address,
            value: vec![String::from("1.2.3.4"), String::from("5.6.7.8")],
        },
        ISCOption {
            name: String::from("encapsulated-global"),
            space: Some(String::from(GLOBAL_ENCAPSULATED_CLASS_NAME)),
            r#type: ISCOptionDefinitionType::DataString,
            value: vec![String::from("EFG")],
        },
        ISCOption {
            name: String::from("encapsulated"),
            space: Some(String::from("test-vendor-class")),
            r#type: ISCOptionDefinitionType::DataString,
            value: vec![String::from("UVW")],
        },
    ]
});

#[cfg(test)]
pub static OPTIONS_TRANSFORMED_TEST_TEMPLATE: &str = r#"
option subnet-mask 1.2.3.4;
option test-vendor-class.testoption 1.2.3.4, 5.6.7.8;
option INTERNAL--global-encapsulated-compat.encapsulated-global "EFG";
option test-vendor-class.encapsulated "UVW";
"#;
