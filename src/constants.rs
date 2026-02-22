#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;
use std::sync::LazyLock;

use super::configs::isc::option_definitions::{ISCOptionDefinition, ISCOptionDefinitionType};

lazy_static! {
    pub static ref SINGLE_BYTE_WITH_PREFIX_REGEXP: Regex =
        Regex::new(r"^0x[A-Fa-f0-9]{2}$").unwrap();
    pub static ref BYTES_WITH_PREFIX_REGEXP: Regex = Regex::new(r"^0x[A-Fa-f0-9]+$").unwrap();
    pub static ref MAC_ADDRESS_REGEXP: Regex =
        Regex::new(r"^[0-9A-Fa-f]{2}([-][0-9A-Fa-f]{2}){5}$").unwrap();
}

pub static DEFAULT_PADDING: &str = "\n\n";

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
}
"#;

pub static RELAY_AGENT_SUBSCRIBER_ID_OPTION_DEFINITION: &str =
    "option agent.subscriber-id code 6 = string;";

pub static POLICY_PSEUDOCONDITION_FOR_DISABLE: &str = "\"enabled\" = \"true\"";
pub static POLICY_CONDITION_OPERATORS: [&str; 2] = ["EQ", "NE"];
/////////////////// MIGRATION SPECIFIC CONFIGURATION PARAMETERS ///////////////////////////

// Information about the declared options is taken from the ISC DHCP documentation, KEA Docs and many RFCs
pub static STANDARD_V4_ISC_OPTION_DEFINITIONS: LazyLock<Vec<ISCOptionDefinition>> =
    LazyLock::new(|| {
        vec![
            ISCOptionDefinition {
                code: 27,
                name: String::from("all-subnets-local"),
                r#type: ISCOptionDefinitionType::Boolean,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 35,
                name: String::from("arp-cache-timeout"),
                r#type: ISCOptionDefinitionType::UInt32,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 89,
                name: String::from("bcms-controller-address"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 88,
                name: String::from("bcms-controller-names"),
                r#type: ISCOptionDefinitionType::DomainList,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 67,
                name: String::from("bootfile-name"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 13,
                name: String::from("boot-size"),
                r#type: ISCOptionDefinitionType::UInt16,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 28,
                name: String::from("broadcast-address"),
                r#type: ISCOptionDefinitionType::IPv4Address,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 138,
                name: String::from("capwap-ac-v4"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 8,
                name: String::from("cookie-servers"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 23,
                name: String::from("default-ip-ttl"),
                r#type: ISCOptionDefinitionType::UInt8,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 37,
                name: String::from("default-tcp-ttl"),
                r#type: ISCOptionDefinitionType::UInt8,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 114,
                name: String::from("default-url"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 61,
                name: String::from("dhcp-client-identifier"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 51,
                name: String::from("dhcp-lease-time"),
                r#type: ISCOptionDefinitionType::UInt32,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 57,
                name: String::from("dhcp-max-message-size"),
                r#type: ISCOptionDefinitionType::UInt16,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 55,
                name: String::from("dhcp-parameter-request-list"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(ISCOptionDefinitionType::UInt8)),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 59,
                name: String::from("dhcp-rebinding-time"),
                r#type: ISCOptionDefinitionType::UInt32,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 58,
                name: String::from("dhcp-renewal-time"),
                r#type: ISCOptionDefinitionType::UInt32,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 54,
                name: String::from("dhcp-server-identifier"),
                r#type: ISCOptionDefinitionType::IPv4Address,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 15,
                name: String::from("domain-name"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 6,
                name: String::from("domain-name-servers"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 119,
                name: String::from("domain-search"),
                r#type: ISCOptionDefinitionType::DomainList,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 18,
                name: String::from("extensions-path"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 73,
                name: String::from("finger-server"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 48,
                name: String::from("font-servers"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 99,
                name: String::from("geoconf-civic"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 12,
                name: String::from("host-name"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 36,
                name: String::from("ieee802-3-encapsulation"),
                r#type: ISCOptionDefinitionType::Boolean,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 5,
                name: String::from("ien116-name-servers"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 10,
                name: String::from("impress-servers"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 26,
                name: String::from("interface-mtu"),
                r#type: ISCOptionDefinitionType::UInt16,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 19,
                name: String::from("ip-forwarding"),
                r#type: ISCOptionDefinitionType::Boolean,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 74,
                name: String::from("irc-server"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 209,
                name: String::from("loader-configfile"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 210,
                name: String::from("loader-pathprefix"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 211,
                name: String::from("loader-reboottime"),
                r#type: ISCOptionDefinitionType::UInt32,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 7,
                name: String::from("log-servers"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 9,
                name: String::from("lpr-servers"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 30,
                name: String::from("mask-supplier"),
                r#type: ISCOptionDefinitionType::Boolean,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 22,
                name: String::from("max-dgram-reassembly"),
                r#type: ISCOptionDefinitionType::UInt16,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 14,
                name: String::from("merit-dump"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 68,
                name: String::from("mobile-ip-home-agent"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 117,
                name: String::from("name-service-search"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(ISCOptionDefinitionType::UInt16)),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 117,
                name: String::from("nds-context"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 85,
                name: String::from("nds-servers"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 86,
                name: String::from("nds-tree-name"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 45,
                name: String::from("netbios-dd-server"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 44,
                name: String::from("netbios-name-servers"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 46,
                name: String::from("netbios-node-type"),
                r#type: ISCOptionDefinitionType::UInt8,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 47,
                name: String::from("netbios-scope"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 112,
                name: String::from("netinfo-server-address"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 113,
                name: String::from("netinfo-server-tag"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 40,
                name: String::from("nis-domain"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 41,
                name: String::from("nis-servers"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 64,
                name: String::from("nisplus-domain"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 65,
                name: String::from("nisplus-servers"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 71,
                name: String::from("nntp-server"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 20,
                name: String::from("non-local-source-routing"),
                r#type: ISCOptionDefinitionType::Boolean,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 42,
                name: String::from("ntp-servers"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 62,
                name: String::from("nwip-domain"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 63,
                name: String::from("nwip-suboptions"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 93,
                name: String::from("pxe-system-type"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(ISCOptionDefinitionType::UInt16)),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 94,
                name: String::from("pxe-interface-id"),
                r#type: ISCOptionDefinitionType::Records(vec![
                    ISCOptionDefinitionType::UInt8,
                    ISCOptionDefinitionType::UInt8,
                    ISCOptionDefinitionType::UInt8,
                ]),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 97,
                name: String::from("pxe-client-id"),
                r#type: ISCOptionDefinitionType::Records(vec![
                    ISCOptionDefinitionType::UInt8,
                    ISCOptionDefinitionType::DataString,
                ]),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 212,
                name: String::from("option-6rd"),
                r#type: ISCOptionDefinitionType::Records(vec![
                    ISCOptionDefinitionType::UInt8,
                    ISCOptionDefinitionType::UInt8,
                    ISCOptionDefinitionType::IPv6Address,
                    ISCOptionDefinitionType::Arrays(Box::new(ISCOptionDefinitionType::IPv4Address)),
                ]),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 136,
                name: String::from("pana-agent"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 24,
                name: String::from("path-mtu-aging-timeout"),
                r#type: ISCOptionDefinitionType::UInt32,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 25,
                name: String::from("path-mtu-plateau-table"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(ISCOptionDefinitionType::UInt16)),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 100,
                name: String::from("pcode"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 29,
                name: String::from("perform-mask-discovery"),
                r#type: ISCOptionDefinitionType::Boolean,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 21,
                name: String::from("policy-filter"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 70,
                name: String::from("pop-server"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 146,
                name: String::from("rdnss-selection"),
                r#type: ISCOptionDefinitionType::Records(vec![
                    ISCOptionDefinitionType::UInt8,
                    ISCOptionDefinitionType::IPv4Address,
                    ISCOptionDefinitionType::IPv4Address,
                    ISCOptionDefinitionType::Text,
                ]),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 11,
                name: String::from("resource-location-servers"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 17,
                name: String::from("root-path"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 31,
                name: String::from("router-discovery"),
                r#type: ISCOptionDefinitionType::Boolean,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 32,
                name: String::from("router-solicitation-address"),
                r#type: ISCOptionDefinitionType::IPv4Address,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 3,
                name: String::from("routers"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 78,
                name: String::from("slp-directory-agent"),
                r#type: ISCOptionDefinitionType::Records(vec![
                    ISCOptionDefinitionType::Boolean,
                    ISCOptionDefinitionType::Arrays(Box::new(ISCOptionDefinitionType::IPv4Address)),
                ]),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 79,
                name: String::from("slp-service-scope"),
                r#type: ISCOptionDefinitionType::Records(vec![
                    ISCOptionDefinitionType::Boolean,
                    ISCOptionDefinitionType::Text,
                ]),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 69,
                name: String::from("smtp-server"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 33,
                name: String::from("static-routes"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 76,
                name: String::from("streettalk-directory-assistance-server"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 75,
                name: String::from("streettalk-server"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 1,
                name: String::from("subnet-mask"),
                r#type: ISCOptionDefinitionType::IPv4Address,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 16,
                name: String::from("swap-server"),
                r#type: ISCOptionDefinitionType::IPv4Address,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 150,
                name: String::from("tftp-server-address"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 39,
                name: String::from("tcp-keepalive-garbage"),
                r#type: ISCOptionDefinitionType::Boolean,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 38,
                name: String::from("tcp-keepalive-interval"),
                r#type: ISCOptionDefinitionType::UInt32,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 101,
                name: String::from("tcode"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 66,
                name: String::from("tftp-server-name"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 2,
                name: String::from("time-offset"),
                r#type: ISCOptionDefinitionType::Int32,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 4,
                name: String::from("time-servers"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 34,
                name: String::from("trailer-encapsulation"),
                r#type: ISCOptionDefinitionType::Boolean,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 98,
                name: String::from("uap-servers"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 77,
                name: String::from("user-class"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 213,
                name: String::from("v4-access-domain"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 137,
                name: String::from("v4-lost"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 60,
                name: String::from("vendor-class-identifier"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 43,
                name: String::from("vendor-encapsulated-options"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 72,
                name: String::from("www-server"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 49,
                name: String::from("x-display-manager"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
        ]
    });

pub static NO_CONFIGURABLE_V4_ISC_OPTION_DEFINITIONS: LazyLock<Vec<ISCOptionDefinition>> =
    LazyLock::new(|| {
        vec![
            ISCOptionDefinition {
                code: 92,
                name: String::from("associated-ip"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv4Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 91,
                name: String::from("client-last-transaction-time"),
                r#type: ISCOptionDefinitionType::UInt32,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 56,
                name: String::from("dhcp-message"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 53,
                name: String::from("dhcp-message-type"),
                r#type: ISCOptionDefinitionType::UInt8,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 52,
                name: String::from("dhcp-option-overload"),
                r#type: ISCOptionDefinitionType::UInt8,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 50,
                name: String::from("dhcp-requested-address"),
                r#type: ISCOptionDefinitionType::IPv4Address,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 118,
                name: String::from("subnet-selection"),
                r#type: ISCOptionDefinitionType::IPv4Address,
                vendor_class: None,
            },
        ]
    });

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
