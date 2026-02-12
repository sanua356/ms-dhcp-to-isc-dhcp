#![allow(dead_code)]

use crate::configs::{
    isc::{ISCDHCP, ISCOptionDefinition, ISCOptionDefinitionType},
    microsoft::{MicrosoftOptionDefinition, MicrosoftOptionDefinitionType},
};
use crate::constants::{
    NO_CONFIGURABLE_V4_ISC_OPTION_DEFINITIONS, STANDARD_V4_ISC_OPTION_DEFINITIONS,
};
use crate::helpers::format_string_isc;

fn get_isc_option_compat(option_type: &MicrosoftOptionDefinitionType) -> ISCOptionDefinitionType {
    match option_type {
        MicrosoftOptionDefinitionType::IPv4Address => ISCOptionDefinitionType::IPv4Address,
        MicrosoftOptionDefinitionType::IPv6Address => ISCOptionDefinitionType::IPv6Address,
        MicrosoftOptionDefinitionType::EncapsulatedData => ISCOptionDefinitionType::Encapsulate,
        MicrosoftOptionDefinitionType::BinaryData => ISCOptionDefinitionType::Text,
        MicrosoftOptionDefinitionType::String => ISCOptionDefinitionType::DataString,
        MicrosoftOptionDefinitionType::Byte => ISCOptionDefinitionType::UInt8,
        MicrosoftOptionDefinitionType::Word => ISCOptionDefinitionType::UInt16,
        MicrosoftOptionDefinitionType::DWord => ISCOptionDefinitionType::UInt32,
        MicrosoftOptionDefinitionType::DWordDWord => ISCOptionDefinitionType::Text,
    }
}

impl ISCDHCP {
    pub fn transform_option_definitions(
        &mut self,
        microsoft_option_definitions: &[MicrosoftOptionDefinition],
    ) {
        let mut option_defs: Vec<ISCOptionDefinition> = Vec::new();

        for ms_option_def in microsoft_option_definitions {
            if NO_CONFIGURABLE_V4_ISC_OPTION_DEFINITIONS
                .iter()
                .any(|item| item.code == ms_option_def.option_id)
            {
                continue;
            }

            if STANDARD_V4_ISC_OPTION_DEFINITIONS.iter().any(|item| {
                item.code == ms_option_def.option_id
                    && ms_option_def
                        .vendor_class
                        .as_ref()
                        .unwrap_or(&String::new())
                        .is_empty()
            }) {
                continue;
            }

            let option_name = format_string_isc(&ms_option_def.name);
            let mut option_type = get_isc_option_compat(&ms_option_def.r#type);
            let option_vendor_class: Option<String> = ms_option_def
                .vendor_class
                .as_ref()
                .map(|vendor_class| format_string_isc(vendor_class));

            if ms_option_def.multi_valued.unwrap_or_default() {
                option_type = ISCOptionDefinitionType::Arrays(Box::new(option_type));
            }

            let option = ISCOptionDefinition {
                code: ms_option_def.option_id,
                name: option_name,
                r#type: option_type,
                vendor_class: option_vendor_class,
            };

            option_defs.push(option);
        }
        self.option_definitions.extend(option_defs);
    }
}

#[cfg(test)]
mod _tests;

#[cfg(test)]
mod test {
    use quick_xml::de::from_str;

    use super::_tests::{
        OPTION_DEFINITIONS_ISC_TEST_TEMPLATE, OPTION_DEFINITIONS_XML_TEST_TEMPLATE,
    };

    use crate::configs::{ISCDHCP, microsoft::MicrosoftOptionDefinition};

    #[test]
    fn transform_option_definitions_test() {
        let data: Vec<MicrosoftOptionDefinition> =
            from_str(OPTION_DEFINITIONS_XML_TEST_TEMPLATE).unwrap();

        let mut isc_config: ISCDHCP = ISCDHCP::default();
        isc_config.transform_option_definitions(&data);

        for (idx, item) in isc_config.option_definitions.iter().enumerate() {
            if item != &OPTION_DEFINITIONS_ISC_TEST_TEMPLATE[idx] {
                panic!(
                    "{:?}, {:?}",
                    item, OPTION_DEFINITIONS_ISC_TEST_TEMPLATE[idx]
                );
            }
        }

        assert!(true);
    }
}
