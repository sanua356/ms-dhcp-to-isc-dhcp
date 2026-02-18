#![allow(dead_code)]

use crate::constants::{
    NO_CONFIGURABLE_V4_ISC_OPTION_DEFINITIONS, STANDARD_V4_ISC_OPTION_DEFINITIONS,
};
use crate::{
    configs::{
        isc::{ISCDHCP, ISCOptionDefinition, ISCOptionDefinitionType},
        microsoft::{MicrosoftOptionDefinition, MicrosoftOptionDefinitionType},
    },
    helpers::format_string_isc,
};

fn get_isc_option_compat(option_type: &MicrosoftOptionDefinitionType) -> ISCOptionDefinitionType {
    match option_type {
        MicrosoftOptionDefinitionType::IPv4Address => ISCOptionDefinitionType::IPv4Address,
        MicrosoftOptionDefinitionType::IPv6Address => ISCOptionDefinitionType::IPv6Address,
        MicrosoftOptionDefinitionType::BinaryData => ISCOptionDefinitionType::Text,
        MicrosoftOptionDefinitionType::String => ISCOptionDefinitionType::DataString,
        MicrosoftOptionDefinitionType::Byte => ISCOptionDefinitionType::UInt8,
        MicrosoftOptionDefinitionType::Word => ISCOptionDefinitionType::UInt16,
        MicrosoftOptionDefinitionType::DWord => ISCOptionDefinitionType::UInt32,
        MicrosoftOptionDefinitionType::DWordDWord => ISCOptionDefinitionType::Text,

        // In Microsoft, the "EncapsulatedData" type describes
        // a string of bytes that will be passed as part of option 43
        MicrosoftOptionDefinitionType::EncapsulatedData => ISCOptionDefinitionType::DataString,
    }
}

impl ISCDHCP {
    pub fn transform_option_definitions(
        &mut self,
        microsoft_option_definitions: &[MicrosoftOptionDefinition],
    ) {
        let mut option_defs: Vec<ISCOptionDefinition> = Vec::new();

        for ms_option_def in microsoft_option_definitions {
            // Skipping options that cannot be directly controlled in ISC
            if NO_CONFIGURABLE_V4_ISC_OPTION_DEFINITIONS
                .iter()
                .any(|item| item.code == ms_option_def.option_id)
            {
                continue;
            }

            // Skipping options that are already declared by default in the ISC
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

            // If the option supports writing multiple values,
            // it needs to replace the original type.
            if ms_option_def.multi_valued.unwrap_or_default() {
                option_type = ISCOptionDefinitionType::Arrays(Box::new(option_type));
            }

            let option_def = ISCOptionDefinition {
                code: ms_option_def.option_id,
                name: option_name,
                r#type: option_type,
                vendor_class: option_vendor_class,
            };

            option_defs.push(option_def);
        }
        self.option_definitions.extend(option_defs);
    }

    pub fn write_transformed_option_definitions(&self, config: &mut String) {
        let option_defs: Vec<String> = self
            .option_definitions
            .iter()
            .map(|item| item.to_string())
            .collect();
        config.push_str(option_defs.join("\n").as_str());
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

    use crate::{
        configs::{ISCDHCP, microsoft::MicrosoftOptionDefinition},
        transformers::option_definitions::_tests::OPTION_DEFINITIONS_TRANSFORMED_TEST_TEMPLATE,
    };

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

    #[test]
    fn write_transformed_option_definitions() {
        let data: Vec<MicrosoftOptionDefinition> =
            from_str(OPTION_DEFINITIONS_XML_TEST_TEMPLATE).unwrap();

        let mut x = String::new();

        let mut isc_config: ISCDHCP = ISCDHCP::default();
        isc_config.transform_option_definitions(&data);
        isc_config.write_transformed_option_definitions(&mut x);

        assert_eq!(
            x.trim(),
            OPTION_DEFINITIONS_TRANSFORMED_TEST_TEMPLATE.trim()
        );
    }
}
