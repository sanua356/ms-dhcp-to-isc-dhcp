use crate::{
    configs::{
        ISCDHCP,
        isc::{ISCClass, ISCOption},
        microsoft::{
            MicrosoftClass, MicrosoftClassType, MicrosoftOptionDefinition, MicrosoftPolicy,
        },
    },
    constants::{POLICY_CONDITION_OPERATORS, POLICY_PSEUDOCONDITION_FOR_DISABLE},
    helpers::{format_string_isc, hex_to_ascii},
    transformers::options::ms_options_to_isc_options,
};

fn get_policy_condition(
    condition_parts: &[String],
    first_operand: &'static str,
    is_hex_bytes: bool,
) -> Vec<String> {
    let operator = condition_parts
        .iter()
        .enumerate()
        .find(|(_, item)| POLICY_CONDITION_OPERATORS.contains(&item.as_str()));

    let mut result: Vec<String> = Vec::new();

    if let Some(operator_value) = operator {
        for (idx, part) in condition_parts.iter().enumerate() {
            if idx == operator_value.0 {
                continue;
            }

            let value = if is_hex_bytes {
                part.replace("-", ":")
            } else {
                format!("\"{part}\"")
            };

            if operator_value.1 == "EQ" {
                result.push(format!("{first_operand} = {value}"));
            } else {
                result.push(format!("not ({first_operand} = {value})"));
            }
        }
    }

    result
}

fn find_class_values_by_names(
    source: &[String],
    microsoft_classes: &[MicrosoftClass],
    microsoft_class_type: MicrosoftClassType,
) -> Vec<String> {
    let mut classes_values: Vec<String> = Vec::new();

    for element in source {
        if POLICY_CONDITION_OPERATORS.contains(&element.as_str()) {
            classes_values.push(element.clone());
            continue;
        }

        let ms_class = microsoft_classes
            .iter()
            .find(|item| item.name == *element && item.r#type == microsoft_class_type);

        if let Some(class) = ms_class {
            classes_values.push(hex_to_ascii(
                class.data.clone().unwrap_or_default().as_str(),
            ));
        }
    }

    classes_values
}

pub fn ms_policies_to_isc_classes(
    microsoft_policies: &[MicrosoftPolicy],
    microsoft_option_definitions: &[MicrosoftOptionDefinition],
    microsoft_classes: &[MicrosoftClass],
) -> Vec<ISCClass> {
    let mut classes: Vec<ISCClass> = Vec::new();

    let mut sorted_policies: Vec<&MicrosoftPolicy> = microsoft_policies.iter().collect();
    sorted_policies.sort_by_key(|item| item.processing_order);

    for ms_policy in sorted_policies {
        let name = format_string_isc(&ms_policy.name);
        let mut options: Vec<ISCOption> = Vec::new();
        let merge_operator = ms_policy.condition.to_string();
        let mut conditions: Vec<String> = Vec::new();

        if let Some(policy_options) = &ms_policy.option_values {
            options.extend(ms_options_to_isc_options(
                &policy_options.items,
                microsoft_option_definitions,
            ));
        }

        if let Some(vendor_class) = &ms_policy.vendor_class {
            conditions.extend(get_policy_condition(
                &find_class_values_by_names(
                    vendor_class,
                    microsoft_classes,
                    MicrosoftClassType::Vendor,
                ),
                "option vendor-class-identifier",
                false,
            ));
        }

        if let Some(user_class) = &ms_policy.user_class {
            conditions.extend(get_policy_condition(
                &find_class_values_by_names(
                    user_class,
                    microsoft_classes,
                    MicrosoftClassType::User,
                ),
                "option user-class",
                false,
            ));
        }

        if let Some(mac_address) = &ms_policy.mac_address {
            conditions.extend(get_policy_condition(mac_address, "hardware", true));
        }

        if let Some(client_id) = &ms_policy.client_id {
            conditions.extend(get_policy_condition(
                client_id,
                "option dhcp-client-identifier",
                true,
            ));
        }

        if let Some(fqdn) = &ms_policy.fqdn {
            conditions.extend(get_policy_condition(fqdn, "option fqdn.fqdn", false));
        }

        if let Some(circuit_id) = &ms_policy.circuit_id {
            conditions.extend(get_policy_condition(
                circuit_id,
                "option agent.circuit-id",
                true,
            ));
        }

        if let Some(remote_id) = &ms_policy.remote_id {
            conditions.extend(get_policy_condition(
                remote_id,
                "option agent.remote-id",
                true,
            ));
        }

        if let Some(subscriber_id) = &ms_policy.subscriber_id {
            conditions.extend(get_policy_condition(
                subscriber_id,
                "option agent.subscriber-id",
                true,
            ));
        }

        if conditions.is_empty() {
            continue;
        }

        let mut condition_string = conditions.join(format!(" {merge_operator} ").as_str());

        if !ms_policy.enabled {
            condition_string =
                format!("{POLICY_PSEUDOCONDITION_FOR_DISABLE} and ({condition_string})");
        }

        classes.push(ISCClass {
            name,
            condition: format!("if {condition_string}"),
            vendor_option_space: None,
            options: Some(options),
        });
    }

    classes
}

impl ISCDHCP {
    pub fn transform_policies(
        &mut self,
        microsoft_policies: &[MicrosoftPolicy],
        microsoft_option_definitions: &[MicrosoftOptionDefinition],
        microsoft_classes: &[MicrosoftClass],
    ) {
        self.policices_classes.extend(ms_policies_to_isc_classes(
            microsoft_policies,
            microsoft_option_definitions,
            microsoft_classes,
        ));
    }

    pub fn write_transformed_policies(&self, config: &mut String) {
        for policy in self.policices_classes.iter() {
            config.push_str(policy.to_string().as_str());
        }
    }
}

#[cfg(test)]
mod _tests;

#[cfg(test)]
mod test {
    use quick_xml::de::from_str;

    use super::_tests::{
        OPTION_DEFINITIONS_XML_TEST_TEMPLATE, POLICIES_ISC_TEST_TEMPLATE,
        POLICIES_TRANSFORMED_TEST_TEMPLATE, POLICIES_XML_TEST_TEMPLATE,
    };

    use crate::{
        configs::{
            ISCDHCP,
            microsoft::{MicrosoftClass, MicrosoftOptionDefinition, MicrosoftPolicy},
        },
        transformers::policies::_tests::CLASSES_XML_TEST_TEMPLATE,
    };

    #[test]
    fn transform_policies_test() {
        let ms_option_defs: Vec<MicrosoftOptionDefinition> =
            from_str(OPTION_DEFINITIONS_XML_TEST_TEMPLATE).unwrap();
        let ms_policies: Vec<MicrosoftPolicy> = from_str(POLICIES_XML_TEST_TEMPLATE).unwrap();
        let ms_classes: Vec<MicrosoftClass> = from_str(CLASSES_XML_TEST_TEMPLATE).unwrap();

        let mut isc_config: ISCDHCP = ISCDHCP::default();
        isc_config.transform_policies(&ms_policies, &ms_option_defs, &ms_classes);

        for (idx, item) in isc_config.policices_classes.iter().enumerate() {
            if item != &POLICIES_ISC_TEST_TEMPLATE[idx] {
                panic!("{:?}, {:?}", item, POLICIES_ISC_TEST_TEMPLATE[idx]);
            }
        }

        assert!(true);
    }

    #[test]
    fn write_transformed_policies_test() {
        let ms_option_defs: Vec<MicrosoftOptionDefinition> =
            from_str(OPTION_DEFINITIONS_XML_TEST_TEMPLATE).unwrap();
        let ms_policies: Vec<MicrosoftPolicy> = from_str(POLICIES_XML_TEST_TEMPLATE).unwrap();
        let ms_classes: Vec<MicrosoftClass> = from_str(CLASSES_XML_TEST_TEMPLATE).unwrap();

        let mut x = String::new();

        let mut isc_config: ISCDHCP = ISCDHCP::default();
        isc_config.transform_policies(&ms_policies, &ms_option_defs, &ms_classes);
        isc_config.write_transformed_policies(&mut x);

        assert_eq!(x.trim(), POLICIES_TRANSFORMED_TEST_TEMPLATE.trim());
    }
}
