#![allow(dead_code)]

use std::sync::LazyLock;

use crate::configs::isc::{ISCOptionDefinition, ISCOptionDefinitionType};

#[cfg(test)]
pub static OPTION_DEFINITIONS_XML_TEST_TEMPLATE: &str = r#"
<OptionDefinition>
  <Name>byteoption</Name>
  <OptionId>181</OptionId>
  <Type>Byte</Type>
  <MultiValued>false</MultiValued>
  <DefaultValue>0x00</DefaultValue>
  <Description />
  <VendorClass>TEST_VENDOR_CLASS</VendorClass>
</OptionDefinition>
<OptionDefinition>
  <Name>wordoption</Name>
  <OptionId>182</OptionId>
  <Type>Word</Type>
  <MultiValued>true</MultiValued>
  <DefaultValue>0</DefaultValue>
  <Description />
  <VendorClass />
</OptionDefinition>
<OptionDefinition>
  <Name>longoption</Name>
  <OptionId>183</OptionId>
  <Type>DWord</Type>
  <MultiValued>false</MultiValued>
  <DefaultValue>0</DefaultValue>
  <Description />
  <VendorClass />
</OptionDefinition>
<OptionDefinition>
  <Name>longintoption</Name>
  <OptionId>184</OptionId>
  <Type>DWordDWord</Type>
  <MultiValued>false</MultiValued>
  <DefaultValue>0</DefaultValue>
  <Description />
  <VendorClass />
</OptionDefinition>
<OptionDefinition>
  <Name>ipv4option</Name>
  <OptionId>185</OptionId>
  <Type>IPv4Address</Type>
  <MultiValued>false</MultiValued>
  <DefaultValue>0.0.0.0</DefaultValue>
  <Description />
  <VendorClass />
</OptionDefinition>
<OptionDefinition>
  <Name>stringoption</Name>
  <OptionId>186</OptionId>
  <Type>String</Type>
  <MultiValued>false</MultiValued>
  <DefaultValue />
  <Description />
  <VendorClass />
</OptionDefinition>
<OptionDefinition>
  <Name>binaryoption</Name>
  <OptionId>187</OptionId>
  <Type>BinaryData</Type>
  <MultiValued>false</MultiValued>
  <DefaultValue>0x00</DefaultValue>
  <Description />
  <VendorClass />
</OptionDefinition>
<OptionDefinition>
  <Name>encapsulated</Name>
  <OptionId>188</OptionId>
  <Type>EncapsulatedData</Type>
  <MultiValued>false</MultiValued>
  <DefaultValue>0x00</DefaultValue>
  <Description />
  <VendorClass />
</OptionDefinition>
"#;

#[cfg(test)]
pub static OPTION_DEFINITIONS_ISC_TEST_TEMPLATE: LazyLock<Vec<ISCOptionDefinition>> =
    LazyLock::new(|| {
        vec![
            ISCOptionDefinition {
                code: 181,
                name: String::from("byteoption"),
                r#type: ISCOptionDefinitionType::UInt8,
                vendor_class: Some(String::from("test_vendor_class-SPACE")),
            },
            ISCOptionDefinition {
                code: 182,
                name: String::from("wordoption"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(ISCOptionDefinitionType::UInt16)),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 183,
                name: String::from("longoption"),
                r#type: ISCOptionDefinitionType::UInt32,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 184,
                name: String::from("longintoption"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 185,
                name: String::from("ipv4option"),
                r#type: ISCOptionDefinitionType::IPv4Address,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 186,
                name: String::from("stringoption"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 187,
                name: String::from("binaryoption"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 188,
                name: String::from("encapsulated"),
                r#type: ISCOptionDefinitionType::Encapsulate(String::new()),
                vendor_class: None,
            },
        ]
    });
