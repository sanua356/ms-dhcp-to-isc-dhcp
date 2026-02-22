#![allow(dead_code)]

use std::sync::LazyLock;

#[cfg(test)]
use crate::configs::isc::{ISCClass, ISCOption, ISCOptionDefinitionType};

#[cfg(test)]
pub static POLICIES_XML_TEST_TEMPLATE: &str = r#"
<Policy>
  <Name>testpolicy</Name>
  <ProcessingOrder>1</ProcessingOrder>
  <Enabled>true</Enabled>
  <Condition>OR</Condition>
  <Description />
  <VendorClass>EQ</VendorClass>
  <VendorClass>Microsoft Windows 2000 Options</VendorClass>
  <DnsSuffix>fawfawfawfawfaw</DnsSuffix>
  <OptionValues>
    <OptionValue>
      <OptionId>2</OptionId>
      <Value>2</Value>
      <VendorClass />
      <UserClass />
    </OptionValue>
    <OptionValue>
      <OptionId>81</OptionId>
      <Value>215</Value>
      <VendorClass />
      <UserClass />
    </OptionValue>
  </OptionValues>
</Policy>
<Policy>
  <Name>testpolicy2</Name>
  <ProcessingOrder>2</ProcessingOrder>
  <Enabled>false</Enabled>
  <Condition>AND</Condition>
  <Description />
  <VendorClass>EQ</VendorClass>
  <VendorClass>Microsoft Options</VendorClass>
  <VendorClass>Microsoft Windows 98 Options</VendorClass>
  <VendorClass>Microsoft Windows 2000 Options</VendorClass>
  <UserClass>NE</UserClass>
  <UserClass>Default BOOTP Class</UserClass>
  <UserClass>Default Routing and Remote Access Class</UserClass>
  <MacAddress>EQ</MacAddress>
  <MacAddress>aa-bb-cc-dd-ee-ff</MacAddress>
  <ClientId>EQ</ClientId>
  <ClientId>00-aa-cc-fe-16</ClientId>
  <RelayAgent>EQ</RelayAgent>
  <RelayAgent>0a-aa-aa</RelayAgent>
  <CircuitId>EQ</CircuitId>
  <CircuitId>0e-ee</CircuitId>
  <RemoteId>EQ</RemoteId>
  <RemoteId>de-d4-ea-ae-ae</RemoteId>
  <SubscriberId>EQ</SubscriberId>
  <SubscriberId>ee-fe-af-ee</SubscriberId>
  <Fqdn>EQ</Fqdn>
  <Fqdn>dev.dev.</Fqdn>
  <Fqdn>google.com</Fqdn>
</Policy>
"#;

#[cfg(test)]
pub static CLASSES_XML_TEST_TEMPLATE: &str = r#"
<Class>
  <Name>Default Routing and Remote Access Class</Name>
  <Type>User</Type>
  <Data>0x525241532e4d6963726f736f6674</Data>
  <Description>User class for remote access clients</Description>
</Class>
<Class>
  <Name>Default BOOTP Class</Name>
  <Type>User</Type>
  <Data>0x424f4f54502e4d6963726f736f6674</Data>
  <Description>User class for BOOTP Clients</Description>
</Class>
<Class>
  <Name>Microsoft Windows 2000 Options</Name>
  <Type>Vendor</Type>
  <Data>0x4d53465420352e30</Data>
  <Description>Microsoft vendor-specific options for Windows 2000 and above Clients</Description>
</Class>
<Class>
  <Name>Microsoft Windows 98 Options</Name>
  <Type>Vendor</Type>
  <Data>0x4d534654203938</Data>
  <Description>Microsoft vendor-specific options for Windows 98 Clients</Description>
</Class>
<Class>
  <Name>Microsoft Options</Name>
  <Type>Vendor</Type>
  <Data>0x4d534654</Data>
  <Description>Microsoft vendor-specific options applicable to all Windows Clients</Description>
</Class>
"#;

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
  <Name>Time Offset</Name>
  <OptionId>2</OptionId>
  <Type>DWord</Type>
  <MultiValued>false</MultiValued>
  <DefaultValue>0</DefaultValue>
  <Description>UTC offset in seconds</Description>
  <VendorClass />
</OptionDefinition>
"#;

#[cfg(test)]
pub static POLICIES_ISC_TEST_TEMPLATE: LazyLock<Vec<ISCClass>> = LazyLock::new(|| {
    vec![
        ISCClass {
            name: String::from("testpolicy"),
            condition: String::from("if option vendor-class-identifier = \"MSFT 5.0\""),
            vendor_option_space: None,
            options: Some(vec![ISCOption {
                name: String::from("time-offset"),
                space: None,
                r#type: ISCOptionDefinitionType::UInt32,
                value: vec![String::from("2")],
            }]),
        },
        ISCClass {
            name: String::from("testpolicy2"),
            condition: String::from(
                r#"if "enabled" = "true" and (option vendor-class-identifier = "MSFT" and option vendor-class-identifier = "MSFT 98" and option vendor-class-identifier = "MSFT 5.0" and not (option user-class = "BOOTP.Microsoft") and not (option user-class = "RRAS.Microsoft") and hardware = aa:bb:cc:dd:ee:ff and option dhcp-client-identifier = 00:aa:cc:fe:16 and option fqdn.fqdn = "dev.dev." and option fqdn.fqdn = "google.com" and option agent.circuit-id = 0e:ee and option agent.remote-id = de:d4:ea:ae:ae and option agent.subscriber-id = ee:fe:af:ee)"#,
            ),
            vendor_option_space: None,
            options: Some(vec![]),
        },
    ]
});

#[cfg(test)]
pub static POLICIES_TRANSFORMED_TEST_TEMPLATE: &str = r#"
class "testpolicy" {
	match if option vendor-class-identifier = "MSFT 5.0";
	option time-offset 2;
}
class "testpolicy2" {
	match if "enabled" = "true" and (option vendor-class-identifier = "MSFT" and option vendor-class-identifier = "MSFT 98" and option vendor-class-identifier = "MSFT 5.0" and not (option user-class = "BOOTP.Microsoft") and not (option user-class = "RRAS.Microsoft") and hardware = aa:bb:cc:dd:ee:ff and option dhcp-client-identifier = 00:aa:cc:fe:16 and option fqdn.fqdn = "dev.dev." and option fqdn.fqdn = "google.com" and option agent.circuit-id = 0e:ee and option agent.remote-id = de:d4:ea:ae:ae and option agent.subscriber-id = ee:fe:af:ee);
}
"#;
