use std::sync::LazyLock;

use crate::configs::isc::option_definitions::{ISCOptionDefinition, ISCOptionDefinitionType};

// Information about the declared options is taken from the ISC DHCP documentation, KEA Docs and many RFCs
pub static STANDARD_V6_ISC_OPTION_DEFINITIONS: LazyLock<Vec<ISCOptionDefinition>> =
    LazyLock::new(|| {
        vec![
            ISCOptionDefinition {
                code: 1,
                name: String::from("dhcp6.client-id"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 2,
                name: String::from("dhcp6.server-id"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 3,
                name: String::from("dhcp6.ia-na"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 4,
                name: String::from("dhcp6.ia-ta"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 5,
                name: String::from("dhcp6.ia-addr"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 6,
                name: String::from("dhcp6.oro"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(ISCOptionDefinitionType::UInt16)),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 7,
                name: String::from("dhcp6.preference"),
                r#type: ISCOptionDefinitionType::UInt8,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 8,
                name: String::from("dhcp6.elapsed-time"),
                r#type: ISCOptionDefinitionType::UInt16,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 9,
                name: String::from("dhcp6.relay-msg"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 12,
                name: String::from("dhcp6.unicast"),
                r#type: ISCOptionDefinitionType::IPv6Address,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 13,
                name: String::from("dhcp6.status-code"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::DataString,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 14,
                name: String::from("dhcp6.rapid-commit"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 17,
                name: String::from("dhcp6.vendor-opts"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 18,
                name: String::from("dhcp6.interface-id"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 19,
                name: String::from("dhcp6.reconf-msg"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 20,
                name: String::from("dhcp6.reconf-accept"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 21,
                name: String::from("dhcp6.sip-servers-names"),
                r#type: ISCOptionDefinitionType::DomainList,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 22,
                name: String::from("dhcp6.sip-servers-addresses"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv6Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 23,
                name: String::from("dhcp6.name-servers"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv6Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 24,
                name: String::from("dhcp6.domain-search"),
                r#type: ISCOptionDefinitionType::DomainList,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 25,
                name: String::from("dhcp6.ia-pd"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 26,
                name: String::from("dhcp6.ia-prefix"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 27,
                name: String::from("dhcp6.nis-servers"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv6Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 28,
                name: String::from("dhcp6.nisp-servers"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv6Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 29,
                name: String::from("dhcp6.nis-domain-name"),
                r#type: ISCOptionDefinitionType::DomainList,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 30,
                name: String::from("dhcp6.nisp-domain-name"),
                r#type: ISCOptionDefinitionType::DomainList,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 31,
                name: String::from("dhcp6.sntp-servers"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv6Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 32,
                name: String::from("dhcp6.info-refresh-time"),
                r#type: ISCOptionDefinitionType::UInt32,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 33,
                name: String::from("dhcp6.bcms-server-d"),
                r#type: ISCOptionDefinitionType::DomainList,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 34,
                name: String::from("dhcp6.bcms-server-a"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv6Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 36,
                name: String::from("dhcp6.geoconf-civic"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 37,
                name: String::from("dhcp6.remote-id"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 38,
                name: String::from("dhcp6.subscriber-id"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 39,
                name: String::from("dhcp6.fqdn"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 40,
                name: String::from("dhcp6.pana-agent"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv6Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 41,
                name: String::from("dhcp6.new-posix-timezone"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 42,
                name: String::from("dhcp6.new-tzdb-timezone"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 43,
                name: String::from("dhcp6.ero"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(ISCOptionDefinitionType::UInt16)),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 44,
                name: String::from("dhcp6.lq-query"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 45,
                name: String::from("dhcp6.client-data"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 46,
                name: String::from("dhcp6.clt-time"),
                r#type: ISCOptionDefinitionType::UInt32,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 47,
                name: String::from("dhcp6.lq-relay-data"),
                r#type: ISCOptionDefinitionType::Records(vec![
                    ISCOptionDefinitionType::IPv6Address,
                    ISCOptionDefinitionType::DataString,
                ]),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 48,
                name: String::from("dhcp6.lq-client-link"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv6Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 51,
                name: String::from("dhcp6.v6-lost"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 52,
                name: String::from("dhcp6.capwap-ac-v6"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                    ISCOptionDefinitionType::IPv6Address,
                )),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 53,
                name: String::from("dhcp6.relay-id"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 57,
                name: String::from("dhcp6.v6-access-domain"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 58,
                name: String::from("dhcp6.sip-ua-cs-list"),
                r#type: ISCOptionDefinitionType::DomainList,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 59,
                name: String::from("dhcp6.bootfile-url"),
                r#type: ISCOptionDefinitionType::Text,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 60,
                name: String::from("dhcp6.bootfile-param"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 61,
                name: String::from("dhcp6.client-arch-type"),
                r#type: ISCOptionDefinitionType::Arrays(Box::new(ISCOptionDefinitionType::UInt16)),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 62,
                name: String::from("dhcp6.nii"),
                r#type: ISCOptionDefinitionType::Records(vec![
                    ISCOptionDefinitionType::UInt8,
                    ISCOptionDefinitionType::UInt8,
                    ISCOptionDefinitionType::UInt8,
                ]),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 64,
                name: String::from("dhcp6.aftr-name"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 65,
                name: String::from("dhcp6.erp-local-domain-name"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 74,
                name: String::from("dhcp6.rdnss-selection"),
                r#type: ISCOptionDefinitionType::Records(vec![
                    ISCOptionDefinitionType::IPv6Address,
                    ISCOptionDefinitionType::UInt8,
                    ISCOptionDefinitionType::DataString,
                ]),
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 79,
                name: String::from("dhcp6.client-linklayer-addr"),
                r#type: ISCOptionDefinitionType::DataString,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 80,
                name: String::from("dhcp6.link-address"),
                r#type: ISCOptionDefinitionType::IPv6Address,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 82,
                name: String::from("dhcp6.solmax-rt"),
                r#type: ISCOptionDefinitionType::UInt32,
                vendor_class: None,
            },
            ISCOptionDefinition {
                code: 83,
                name: String::from("dhcp6.inf-max-rt"),
                r#type: ISCOptionDefinitionType::UInt32,
                vendor_class: None,
            },
        ]
    });
