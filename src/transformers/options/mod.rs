use std::{
    fmt::Debug,
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use crate::{
    configs::{
        ISCDHCP,
        isc::ISCOption,
        microsoft::{
            MicrosoftOptionDefinition, MicrosoftOptionDefinitionType, MicrosoftOptionValue,
        },
    },
    constants::{
        GLOBAL_ENCAPSULATED_CLASS_NAME, NO_CONFIGURABLE_V4_ISC_OPTION_DEFINITIONS,
        STANDARD_V4_ISC_OPTION_DEFINITIONS,
    },
    helpers::{format_string_isc, vec_bytes_string_to_string},
    transformers::option_definitions::get_isc_option_compat,
};

fn transform_numeric_values<T>(values: &[String], error_type: &str, is_hex: bool) -> Vec<String>
where
    T: TryFrom<u128> + ToString,
    <T as TryFrom<u128>>::Error: Debug,
{
    let mut output: Vec<String> = Vec::with_capacity(values.len());

    for value in values.iter() {
        let cleaned = value.trim_start_matches("0x");

        let parsed = if is_hex {
            u128::from_str_radix(cleaned, 16)
                .unwrap_or_else(|_| panic!("Value '{value}' is not valid hex."))
        } else {
            cleaned
                .parse::<u128>()
                .unwrap_or_else(|_| panic!("Value '{value}' is not valid decimal."))
        };

        let transformed = T::try_from(parsed)
            .unwrap_or_else(|_| panic!("Value '{value}' does not fit into {error_type}."));

        output.push(transformed.to_string());
    }

    output
}

fn transform_ipaddr_values<T>(values: &[String], error_type: &str) -> Vec<String>
where
    T: FromStr + ToString,
    <T as FromStr>::Err: Debug,
{
    let mut output: Vec<String> = Vec::with_capacity(values.len());

    for value in values.iter() {
        let transformed: T = value
            .parse()
            .unwrap_or_else(|_| panic!("Value '{value}' is not {error_type}."));

        output.push(transformed.to_string());
    }

    output
}

fn get_transformed_option_value(
    ms_option: &MicrosoftOptionValue,
    ms_option_def: &MicrosoftOptionDefinition,
) -> Vec<String> {
    let values: Vec<String> = match &ms_option.value {
        Some(ms_value) => ms_value.clone(),
        None => vec![],
    };

    match ms_option_def.r#type {
        MicrosoftOptionDefinitionType::BinaryData
        | MicrosoftOptionDefinitionType::EncapsulatedData => {
            vec![vec_bytes_string_to_string(values)]
        }
        MicrosoftOptionDefinitionType::String => values,
        MicrosoftOptionDefinitionType::IPv4Address => {
            transform_ipaddr_values::<Ipv4Addr>(&values, "IPv4 address")
        }
        MicrosoftOptionDefinitionType::IPv6Address => {
            transform_ipaddr_values::<Ipv6Addr>(&values, "IPv6 address")
        }
        MicrosoftOptionDefinitionType::Byte => {
            transform_numeric_values::<u8>(&values, "1 byte", true)
        }
        MicrosoftOptionDefinitionType::Word => {
            transform_numeric_values::<u16>(&values, "2 bytes", false)
        }
        MicrosoftOptionDefinitionType::DWord => {
            transform_numeric_values::<u32>(&values, "4 bytes", false)
        }
        MicrosoftOptionDefinitionType::DWordDWord => {
            transform_numeric_values::<u64>(&values, "8 bytes", false)
        }
    }
}

pub fn ms_options_to_isc_options(
    microsoft_options: &[MicrosoftOptionValue],
    microsoft_option_definitions: &[MicrosoftOptionDefinition],
) -> Vec<ISCOption> {
    let mut options: Vec<ISCOption> = Vec::new();

    for ms_option in microsoft_options {
        // Skipping options that cannot be directly controlled in ISC
        if NO_CONFIGURABLE_V4_ISC_OPTION_DEFINITIONS
            .iter()
            .any(|item| item.code == ms_option.option_id)
        {
            continue;
        }

        // Since option definitions do not export user classes, such options are skipped
        if ms_option.user_class.is_some() && ms_option.vendor_class.is_none() {
            continue;
        }

        let ms_option_def = microsoft_option_definitions.iter().find(|item| {
            item.option_id == ms_option.option_id && item.vendor_class == ms_option.vendor_class
        });

        if let Some(def) = ms_option_def {
            let name: String = if ms_option.vendor_class.is_none() {
                STANDARD_V4_ISC_OPTION_DEFINITIONS
                    .iter()
                    .find(|item| item.code == ms_option.option_id)
                    .map(|item| item.name.clone())
                    .unwrap_or_else(|| format_string_isc(&def.name))
            } else {
                format_string_isc(&def.name)
            };
            let value = get_transformed_option_value(ms_option, def);
            let space: Option<String> = match &ms_option.vendor_class {
                Some(vendor_class) => Some(format_string_isc(vendor_class.as_str())),
                None => {
                    if def.r#type == MicrosoftOptionDefinitionType::EncapsulatedData {
                        Some(GLOBAL_ENCAPSULATED_CLASS_NAME.to_string())
                    } else {
                        None
                    }
                }
            };
            let r#type = get_isc_option_compat(&def.r#type);

            options.push(ISCOption {
                name,
                space,
                value,
                r#type,
            });
        } else {
            continue;
        }
    }

    options
}

impl ISCDHCP {
    pub fn transform_options(
        &mut self,
        microsoft_options: &[MicrosoftOptionValue],
        microsoft_option_definitions: &[MicrosoftOptionDefinition],
    ) {
        self.options.extend(ms_options_to_isc_options(
            microsoft_options,
            microsoft_option_definitions,
        ));
    }

    pub fn write_transformed_options(&self, config: &mut String) {
        let options: Vec<String> = self.options.iter().map(|item| item.to_string()).collect();
        config.push_str(options.join("\n").as_str());
    }
}

#[cfg(test)]
mod _tests;

#[cfg(test)]
mod test {
    use quick_xml::de::from_str;

    use super::_tests::{
        OPTION_DEFINITIONS_XML_TEST_TEMPLATE, OPTIONS_ISC_TEST_TEMPLATE, OPTIONS_XML_TEST_TEMPLATE,
    };

    use crate::{
        configs::{
            ISCDHCP,
            microsoft::{MicrosoftOptionDefinition, MicrosoftOptionValue},
        },
        transformers::options::_tests::OPTIONS_TRANSFORMED_TEST_TEMPLATE,
    };

    #[test]
    fn transform_options_test() {
        let ms_option_defs: Vec<MicrosoftOptionDefinition> =
            from_str(OPTION_DEFINITIONS_XML_TEST_TEMPLATE).unwrap();
        let ms_options: Vec<MicrosoftOptionValue> = from_str(OPTIONS_XML_TEST_TEMPLATE).unwrap();

        let mut isc_config: ISCDHCP = ISCDHCP::default();
        isc_config.transform_options(&ms_options, &ms_option_defs);

        for (idx, item) in isc_config.options.iter().enumerate() {
            if item != &OPTIONS_ISC_TEST_TEMPLATE[idx] {
                panic!("{:?}, {:?}", item, OPTIONS_ISC_TEST_TEMPLATE[idx]);
            }
        }

        assert!(true);
    }

    #[test]
    fn write_transformed_options_test() {
        let ms_option_defs: Vec<MicrosoftOptionDefinition> =
            from_str(OPTION_DEFINITIONS_XML_TEST_TEMPLATE).unwrap();
        let ms_options: Vec<MicrosoftOptionValue> = from_str(OPTIONS_XML_TEST_TEMPLATE).unwrap();

        let mut x = String::new();

        let mut isc_config: ISCDHCP = ISCDHCP::default();
        isc_config.transform_options(&ms_options, &ms_option_defs);
        isc_config.write_transformed_options(&mut x);

        assert_eq!(x.trim(), OPTIONS_TRANSFORMED_TEST_TEMPLATE.trim());
    }
}
