#![allow(dead_code)]

#[cfg(test)]
use std::net::Ipv4Addr;
use std::sync::LazyLock;

#[cfg(test)]
use crate::{
    configs::isc::{ISCOption, ISCOptionDefinitionType, ISCPoolV4, ISCSubnetV4, ISCSubnetV4Type},
    constants::FILTER_ALLOW_CLASS_NAME,
};

#[cfg(test)]
pub static SCOPES_XML_TEST_TEMPLATE: &str = r#"
<Scope>
  <ScopeId>1.2.3.0</ScopeId>
  <Name>testscope</Name>
  <SubnetMask>255.255.255.0</SubnetMask>
  <StartRange>1.2.3.4</StartRange>
  <EndRange>1.2.3.10</EndRange>
  <LeaseDuration>8.00:00:00</LeaseDuration>
  <State>Active</State>
  <Type>Dhcp</Type>
  <MaxBootpClients>4294967295</MaxBootpClients>
  <NapEnable>false</NapEnable>
  <Delay>0</Delay>
  <NapProfile />
  <Description />
  <ActivatePolicies>true</ActivatePolicies>
  <SuperScopeName />
  <ExclusionRanges>
    <IPRange>
      <StartRange>1.2.3.5</StartRange>
      <EndRange>1.2.3.7</EndRange>
    </IPRange>
  </ExclusionRanges>
  <Policies>
    <Policy>
      <Name>testpolicy3</Name>
      <ProcessingOrder>1</ProcessingOrder>
      <Enabled>true</Enabled>
      <Condition>OR</Condition>
      <Description />
      <VendorClass>EQ</VendorClass>
      <VendorClass>Microsoft Options</VendorClass>
      <VendorClass>Microsoft Windows 2000 Options</VendorClass>
      <OptionValues>
        <OptionValue>
          <OptionId>1</OptionId>
          <Value>2457</Value>
          <VendorClass>Microsoft Options</VendorClass>
          <UserClass />
        </OptionValue>
        <OptionValue>
          <OptionId>3</OptionId>
          <Value>1.2.3.5</Value>
          <VendorClass />
          <UserClass />
        </OptionValue>
      </OptionValues>
    </Policy>
    <Policy>
      <Name>Default BOOTP Class</Name>
      <ProcessingOrder>10</ProcessingOrder>
      <Enabled>true</Enabled>
      <Condition>OR</Condition>
      <Description>User class for BOOTP Clients</Description>
      <UserClass>EQ</UserClass>
      <UserClass>Default BOOTP Class</UserClass>
      <OptionValues>
        <OptionValue>
          <OptionId>51</OptionId>
          <Value>2592000</Value>
          <VendorClass />
          <UserClass />
        </OptionValue>
      </OptionValues>
      <IPRanges>
        <IPRange>
            <StartRange>10.10.20.110</StartRange>
            <EndRange>10.10.20.120</EndRange>
        </IPRange>
      </IPRanges>
    </Policy>
  </Policies>
  <OptionValues>
    <OptionValue>
      <OptionId>6</OptionId>
      <Value>10.81.0.251</Value>
      <Value>1.2.3.100</Value>
      <VendorClass />
      <UserClass />
    </OptionValue>
    <OptionValue>
      <OptionId>51</OptionId>
      <Value>691200</Value>
      <VendorClass />
      <UserClass />
    </OptionValue>
  </OptionValues>
</Scope>
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
  <Name>Lease</Name>
  <OptionId>51</OptionId>
  <Type>DWord</Type>
  <MultiValued>false</MultiValued>
  <DefaultValue>0</DefaultValue>
  <Description>Client IP address lease time in seconds</Description>
  <VendorClass />
</OptionDefinition>
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
  <Name>Router</Name>
  <OptionId>3</OptionId>
  <Type>IPv4Address</Type>
  <MultiValued>true</MultiValued>
  <DefaultValue>0.0.0.0</DefaultValue>
  <Description>Array of router addresses ordered by preference</Description>
  <VendorClass />
</OptionDefinition>
<OptionDefinition>
  <Name>DNS Servers</Name>
  <OptionId>6</OptionId>
  <Type>IPv4Address</Type>
  <MultiValued>true</MultiValued>
  <DefaultValue>0.0.0.0</DefaultValue>
  <Description>Array of DNS servers, by preference</Description>
  <VendorClass />
</OptionDefinition>
"#;

#[cfg(test)]
pub static FILTERS_XML_TEST_TEMPLATE: &str = r#"
<Filters>
  <Allow>true</Allow>
  <Deny>true</Deny>
  <Filter>
    <List>Allow</List>
    <MacAddress>AA-BB-CC-DD-EE-FF</MacAddress>
    <Description />
  </Filter>
  <Filter>
    <List>Allow</List>
    <MacAddress>33-33-33-33-33-33</MacAddress>
    <Description />
  </Filter>
  <Filter>
    <List>Deny</List>
    <MacAddress>CC-DD-EE-FF-11-11</MacAddress>
    <Description />
  </Filter>
  <Filter>
    <List>Deny</List>
    <MacAddress>11-11-11-11-11-11</MacAddress>
    <Description />
  </Filter>
</Filters>
"#;

#[cfg(test)]
pub static SUBNETS_ISC_TEST_TEMPLATE: LazyLock<Vec<ISCSubnetV4>> = LazyLock::new(|| {
    vec![ISCSubnetV4 {
        id: Ipv4Addr::new(1, 2, 3, 0),
        netmask: Ipv4Addr::new(255, 255, 255, 0),
        min_lease_time: 691200,
        default_lease_time: 691200,
        max_lease_time: 691200,
        r#type: ISCSubnetV4Type::DHCP,
        reservations: Some(vec![]),
        pools: vec![
            ISCPoolV4 {
                start_range: Ipv4Addr::new(1, 2, 3, 4),
                end_range: Ipv4Addr::new(1, 2, 3, 4),
                classes_names: Some(vec![
                    String::from("INTERNAL--SUBNET--testpolicy3"),
                    String::from(FILTER_ALLOW_CLASS_NAME),
                ]),
            },
            ISCPoolV4 {
                start_range: Ipv4Addr::new(1, 2, 3, 8),
                end_range: Ipv4Addr::new(1, 2, 3, 10),
                classes_names: Some(vec![
                    String::from("INTERNAL--SUBNET--testpolicy3"),
                    String::from(FILTER_ALLOW_CLASS_NAME),
                ]),
            },
        ],
        options: Some(vec![
            ISCOption {
                name: String::from("domain-name-servers"),
                space: None,
                r#type: ISCOptionDefinitionType::IPv4Address,
                value: vec![String::from("10.81.0.251"), String::from("1.2.3.100")],
            },
            ISCOption {
                name: String::from("dhcp-lease-time"),
                space: None,
                r#type: ISCOptionDefinitionType::UInt32,
                value: vec![String::from("691200")],
            },
        ]),
    }]
});

#[cfg(test)]
pub static SCOPES_TRANSFORMED_TEST_TEMPLATE: &str = r#"
class "INTERNAL--SUBNET--testpolicy3" {
	match if option vendor-class-identifier = "MSFT" or option vendor-class-identifier = "MSFT 5.0";
	option routers 1.2.3.5;
}
subnet 1.2.3.0 netmask 255.255.255.0 {

pool {
	range 1.2.3.4 1.2.3.4;
	allow members of "INTERNAL--SUBNET--testpolicy3";
	allow members of "INTERNAL--allow-filter";
}

pool {
	range 1.2.3.8 1.2.3.10;
	allow members of "INTERNAL--SUBNET--testpolicy3";
	allow members of "INTERNAL--allow-filter";
}

min-lease-time 691200;
default-lease-time 691200;
max-lease-time 691200;
deny bootp;
option domain-name-servers 10.81.0.251, 1.2.3.100;
option dhcp-lease-time 691200;
}
"#;
