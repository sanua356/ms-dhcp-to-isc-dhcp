#![allow(dead_code)]

use std::sync::LazyLock;

use crate::{
    configs::isc::{ISCHost, ISCSubclass},
    constants::FILTER_ALLOW_CLASS_NAME,
};

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
pub static FILTERS_DENY_HOSTS_ISC_TEST_TEMPLATE: LazyLock<Vec<ISCHost>> = LazyLock::new(|| {
    vec![
        ISCHost {
            name: String::from("deny-host-0"),
            mac_address: Some(String::from("CC:DD:EE:FF:11:11")),
            fixed_address: None,
            options: None,
        },
        ISCHost {
            name: String::from("deny-host-1"),
            mac_address: Some(String::from("11:11:11:11:11:11")),
            fixed_address: None,
            options: None,
        },
    ]
});

#[cfg(test)]
pub static FILTERS_ALLOW_SUBCLASSES_ISC_TEST_TEMPLATE: LazyLock<Vec<ISCSubclass>> =
    LazyLock::new(|| {
        vec![
            ISCSubclass {
                name: String::from("AA:BB:CC:DD:EE:FF"),
                parent_name: String::from(FILTER_ALLOW_CLASS_NAME),
                condition: None,
                vendor_option_space: None,
            },
            ISCSubclass {
                name: String::from("33:33:33:33:33:33"),
                parent_name: String::from(FILTER_ALLOW_CLASS_NAME),
                condition: None,
                vendor_option_space: None,
            },
        ]
    });

#[cfg(test)]
pub static FILTERS_TRANSFORMED_TEST_TEMPLATE: &str = r#"
group {
	deny booting;

host deny-host-0 {
	hardware ethernet CC:DD:EE:FF:11:11;
}

host deny-host-1 {
	hardware ethernet 11:11:11:11:11:11;
}
}
subclass "INTERNAL--allow-filter" "AA:BB:CC:DD:EE:FF" {
}

subclass "INTERNAL--allow-filter" "33:33:33:33:33:33" {
}
"#;
