#![allow(dead_code)]

use clap::ValueEnum;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref SINGLE_BYTE_WITH_PREFIX_REGEXP: Regex =
        Regex::new(r"^0x[A-Fa-f0-9]{2}$").unwrap();
    pub static ref BYTES_WITH_PREFIX_REGEXP: Regex = Regex::new(r"^0x[A-Fa-f0-9]+$").unwrap();
    pub static ref MAC_ADDRESS_REGEXP: Regex =
        Regex::new(r"^[0-9A-Fa-f]{2}([-][0-9A-Fa-f]{2}){5}$").unwrap();
}

pub static DEFAULT_PADDING: &str = "\n\n";
#[derive(Debug, PartialEq, Clone, Copy, ValueEnum)]
pub enum MigratorTypes {
    V4,
    V6,
}

/////////////////// MIGRATION SPECIFIC CONFIGURATION PARAMETERS ///////////////////////////
pub static GLOBAL_ENCAPSULATED_SPACE: &str = "option space INTERNAL--global-encapsulated-compat;";
pub static GLOBAL_ENCAPSULATED_CLASS_NAME: &str = "INTERNAL--global-encapsulated-compat";
pub static GLOBAL_ENCAPSULATED_CLASS: &str = r#"
class "INTERNAL--global-encapsulated-compat" {
	match if not exists vendor-class-identifier;
	vendor-option-space INTERNAL--global-encapsulated-compat;
}
"#;

pub static FILTER_ALLOW_CLASS_NAME: &str = "INTERNAL--allow-filter";
pub static FILTER_ALLOW_CLASS: &str = r#"
class "INTERNAL--allow-filter" {
	match hardware;
}"#;

pub static RELAY_AGENT_SUBSCRIBER_ID_OPTION_DEFINITION: &str = r#"option agent.subscriber-id code 6 = string;
"#;

pub static POLICY_PSEUDOCONDITION_FOR_DISABLE: &str = "\"enabled\" = \"true\"";
pub static POLICY_CONDITION_OPERATORS: [&str; 2] = ["EQ", "NE"];
pub static POLICY_SUBNET_PREFIX: &str = "INTERNAL--SUBNET-";
/////////////////// MIGRATION SPECIFIC CONFIGURATION PARAMETERS ///////////////////////////

pub static MICROSOFT_STANDARD_CLASSES: [&str; 5] = [
    "Default Routing and Remote Access Class",
    "Default BOOTP Class",
    "Microsoft Windows 2000 Options",
    "Microsoft Windows 98 Options",
    "Microsoft Options",
];

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::{BYTES_WITH_PREFIX_REGEXP, MAC_ADDRESS_REGEXP};

    #[rstest]
    #[case("0x112233")]
    #[case("0xAABBCC11")]
    #[case("0xaaBBcc22")]
    fn bytes_with_prefix_string_regexp_match_test(#[case] source: &str) {
        assert!(BYTES_WITH_PREFIX_REGEXP.is_match(source));
    }

    #[rstest]
    #[case("112233")]
    #[case("0xTTTQQQ")]
    fn bytes_with_prefix_string_regexp_not_match_test(#[case] source: &str) {
        assert!(!BYTES_WITH_PREFIX_REGEXP.is_match(source));
    }

    #[rstest]
    #[case("11-22-33-44-55-66")]
    #[case("AA-BB-CC-DD-EE-FF")]
    #[case("AA-bb-cc-DD-11-22")]
    fn mac_address_string_regexp_match_test(#[case] source: &str) {
        assert!(MAC_ADDRESS_REGEXP.is_match(source));
    }

    #[rstest]
    #[case("11-66")]
    #[case("AA:BB:CC:DD-EE-FF")]
    #[case("QQ-WW-EE-RR-TT-YY")]
    #[case("Q-W-E-R-T-Y")]
    fn mac_address_string_regexp_not_match_test(#[case] source: &str) {
        assert!(!MAC_ADDRESS_REGEXP.is_match(source));
    }
}
