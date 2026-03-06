#![allow(dead_code)]

use std::sync::LazyLock;

use crate::configs::isc::{ISCOptionDefinition, ISCOptionDefinitionType};

#[cfg(test)]
pub static OPTION_DEFINITIONS_XML_TEST_TEMPLATE_V4: &str = r#"
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
pub static OPTION_DEFINITIONS_ISC_TEST_TEMPLATE_V4: LazyLock<Vec<ISCOptionDefinition>> =
    LazyLock::new(|| {
        vec![
            ISCOptionDefinition {
                code: 181,
                name: String::from("byteoption"),
                r#type: ISCOptionDefinitionType::UInt8,
                vendor_class: Some(String::from("test-vendor-class")),
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
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: Some(String::from("INTERNAL--global-encapsulated-compat")),
            },
        ]
    });

#[cfg(test)]
pub static OPTION_DEFINITIONS_TRANSFORMED_TEST_TEMPLATE_V4: &str = r#"
option test-vendor-class.byteoption code 181 = unsigned integer 8;
option wordoption code 182 = array of unsigned integer 16;
option longoption code 183 = unsigned integer 32;
option longintoption code 184 = text;
option ipv4option code 185 = ip-address;
option stringoption code 186 = string;
option binaryoption code 187 = text;
option INTERNAL--global-encapsulated-compat.encapsulated code 188 = string;
"#;

#[cfg(test)]
pub static OPTION_DEFINITIONS_XML_TEST_TEMPLATE_V6: &str = r#"
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
<OptionDefinition>
  <Name>ipv6option</Name>
  <OptionId>189</OptionId>
  <Type>IPv6Address</Type>
  <MultiValued>true</MultiValued>
  <DefaultValue />
  <Description />
  <VendorClass />
</OptionDefinition>
"#;

#[cfg(test)]
pub static OPTION_DEFINITIONS_ISC_TEST_TEMPLATE_V6: LazyLock<Vec<ISCOptionDefinition>> =
    LazyLock::new(|| {
        vec![
            ISCOptionDefinition {
                code: 182,
                name: String::from("dhcp6.wordoption"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(ISCOptionDefinitionType::UInt16)),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 183,
                name: String::from("dhcp6.longoption"),
                r#type: ISCOptionDefinitionType::UInt32,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 184,
                name: String::from("dhcp6.longintoption"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 185,
                name: String::from("dhcp6.ipv4option"),
                r#type: ISCOptionDefinitionType::IPv4Address,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 186,
                name: String::from("dhcp6.stringoption"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 187,
                name: String::from("dhcp6.binaryoption"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 189,
                name: String::from("dhcp6.ipv6option"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv6Address,
                )),
                vendor_class: None,
            },
        ]
    });

#[cfg(test)]
pub static OPTION_DEFINITIONS_TRANSFORMED_TEST_TEMPLATE_V6: &str = r#"
option dhcp6.wordoption code 182 = array of unsigned integer 16;
option dhcp6.longoption code 183 = unsigned integer 32;
option dhcp6.longintoption code 184 = text;
option dhcp6.ipv4option code 185 = ip-address;
option dhcp6.stringoption code 186 = string;
option dhcp6.binaryoption code 187 = text;
option dhcp6.ipv6option code 189 = array of ip6-address;
"#;
