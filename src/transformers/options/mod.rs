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

fn transform_values<T>(values: &[String], error_type: &str) -> Vec<String>
where
    T: FromStr + ToString,
    <T as FromStr>::Err: Debug,
{
    let mut output: Vec<String> = Vec::with_capacity(values.len());

    for value in values.iter() {
        let transformed: T = value
            .trim_start_matches("0x")
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
            transform_values::<Ipv4Addr>(&values, "IPv4 address")
        }
        MicrosoftOptionDefinitionType::IPv6Address => {
            transform_values::<Ipv6Addr>(&values, "IPv6 address")
        }
        MicrosoftOptionDefinitionType::Byte => transform_values::<u8>(&values, "1 byte"),
        MicrosoftOptionDefinitionType::Word => transform_values::<u16>(&values, "2 bytes"),
        MicrosoftOptionDefinitionType::DWord => transform_values::<u32>(&values, "4 bytes"),
        MicrosoftOptionDefinitionType::DWordDWord => transform_values::<u64>(&values, "8 bytes"),
    }
}

impl ISCDHCP {
    pub fn transform_options(
        &mut self,
        microsoft_options: &[MicrosoftOptionValue],
        microsoft_option_definitions: &[MicrosoftOptionDefinition],
    ) {
        let mut options: Vec<ISCOption> = Vec::new();

        for ms_option in microsoft_options {
            // Skipping options that cannot be directly controlled in ISC
            if NO_CONFIGURABLE_V4_ISC_OPTION_DEFINITIONS
                .iter()
                .any(|item| item.code == ms_option.option_id)
            {
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

        self.options.extend(options);
    }

    pub fn write_transformed_options(&self, config: &mut String) {
        let options: Vec<String> = self.options.iter().map(|item| item.to_string()).collect();
        config.push_str(options.join("\n").as_str());
    }
}
