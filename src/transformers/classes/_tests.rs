#![allow(dead_code)]

use std::sync::LazyLock;

use crate::configs::isc::ISCClass;

#[cfg(test)]
pub static CLASSES_XML_TEST_TEMPLATE: &str = r#"
<Class>
  <Name>Default Routing and Remote Access Class</Name>
  <Type>User</Type>
  <Data>0x525241532e4d6963726f736f6674</Data>
  <Description>User class for remote access clients</Description>
</Class>
<Class>
  <Name>Microsoft Windows 2000 Options</Name>
  <Type>Vendor</Type>
  <Data>0x4d53465420352e30</Data>
  <Description>Microsoft vendor-specific options for Windows 2000 and above Clients</Description>
</Class>
"#;

#[cfg(test)]
pub static CLASSES_ISC_TEST_TEMPLATE: LazyLock<Vec<ISCClass>> = LazyLock::new(|| {
    vec![
        ISCClass {
            name: String::from("default-routing-and-remote-access-class"),
            condition: String::from(
                "match if option vendor-class-identifier = \"RRAS.Microsoft\";",
            ),
        },
        ISCClass {
            name: String::from("microsoft-windows-2000-options"),
            condition: String::from("match if option user-class = \"MSFT 5.0\";"),
        },
    ]
});
