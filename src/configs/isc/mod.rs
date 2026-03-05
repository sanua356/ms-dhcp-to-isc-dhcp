pub mod classes;
pub mod hosts;
pub mod option_definitions;
pub mod options;
pub mod shared_networks;
pub mod subclasses;
pub mod subnets;

pub use classes::*;
pub use hosts::*;
pub use option_definitions::*;
pub use options::*;
pub use shared_networks::*;
pub use subclasses::*;
pub use subnets::*;

use crate::{
    configs::MicrosoftDHCP,
    constants::{
        DEFAULT_PADDING, FILTER_ALLOW_CLASS, GLOBAL_ENCAPSULATED_CLASS, GLOBAL_ENCAPSULATED_SPACE,
        RELAY_AGENT_SUBSCRIBER_ID_OPTION_DEFINITION,
    },
};

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct ISCDHCP {
    pub option_definitions: Vec<option_definitions::ISCOptionDefinition>,
    pub options: Vec<options::ISCOption>,
    pub classes: Vec<classes::ISCClass>,
    pub policices_classes: Vec<classes::ISCClass>,
    pub subnet_v4_classes: Vec<classes::ISCClass>,
    pub deny_filter_hosts: Vec<hosts::ISCHost>,
    pub allow_filter_subclasses: Vec<subclasses::ISCSubclass>,
    pub subnets_v4: Vec<subnets::ISCSubnetV4>,
    pub shared_networks_v4: Vec<shared_networks::ISCSharedNetworkV4>,
}

impl ISCDHCP {
    pub fn default() -> Self {
        ISCDHCP {
            option_definitions: vec![],
            options: vec![],
            classes: vec![],
            policices_classes: vec![],
            subnet_v4_classes: vec![],
            deny_filter_hosts: vec![],
            allow_filter_subclasses: vec![],
            subnets_v4: vec![],
            shared_networks_v4: vec![],
        }
    }

    fn write_internal_configuration_parameters(&self, config: &mut String) {
        config.push_str(RELAY_AGENT_SUBSCRIBER_ID_OPTION_DEFINITION);
        config.push_str(GLOBAL_ENCAPSULATED_SPACE);
        config.push_str(GLOBAL_ENCAPSULATED_CLASS);
        config.push_str(FILTER_ALLOW_CLASS);
    }

    pub fn transform_v4(&mut self, microsoft_config: MicrosoftDHCP) {
        let defs = microsoft_config
            .ipv4
            .option_definitions
            .map(|data| data.items)
            .unwrap_or_default();
        let classes = microsoft_config
            .ipv4
            .classes
            .map(|data| data.items)
            .unwrap_or_default();
        let filters = microsoft_config.ipv4.filters;
        let option_values = microsoft_config
            .ipv4
            .option_values
            .map(|data| data.items)
            .unwrap_or_default();
        let policies = microsoft_config
            .ipv4
            .policies
            .map(|data| data.items)
            .unwrap_or_default();
        let scopes_v4 = microsoft_config
            .ipv4
            .scopes
            .map(|data| data.items)
            .unwrap_or_default();

        self.transform_option_definitions(&defs);
        self.transform_options(&option_values, &defs);
        self.transform_classes(&classes);
        self.transform_policies(&policies, &defs, &classes);
        if let Some(data) = &filters {
            self.transform_filters(data);
        }
        self.transform_scopes_v4(&scopes_v4, &defs, &classes, &filters);
    }

    pub fn write_v4(&self) -> String {
        let mut output: String = String::new();

        self.write_internal_configuration_parameters(&mut output);
        output.push_str(DEFAULT_PADDING);
        self.write_transformed_classes_to_spaces(&mut output);
        output.push_str(DEFAULT_PADDING);
        self.write_transformed_classes(&mut output);
        output.push_str(DEFAULT_PADDING);
        self.write_transformed_option_definitions(&mut output);
        output.push_str(DEFAULT_PADDING);
        self.write_transformed_options(&mut output);
        output.push_str(DEFAULT_PADDING);
        self.write_transformed_policies(&mut output);
        output.push_str(DEFAULT_PADDING);
        self.write_transformed_scopes(&mut output);
        output.push_str(DEFAULT_PADDING);
        self.write_transformed_filters(&mut output);

        output
    }
}
