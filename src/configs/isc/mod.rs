#![allow(dead_code)]

pub mod classes;
pub mod hosts;
pub mod option_definitions;
pub mod options;
pub mod subclasses;
pub mod subnets;

pub use classes::*;
pub use hosts::*;
pub use option_definitions::*;
pub use options::*;
pub use subclasses::*;
pub use subnets::*;

use crate::constants::{
    FILTER_ALLOW_CLASS, GLOBAL_ENCAPSULATED_CLASS, GLOBAL_ENCAPSULATED_SPACE,
    RELAY_AGENT_SUBSCRIBER_ID_OPTION_DEFINITION,
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
        }
    }

    pub fn write_internal_configuration_parameters(&self, config: &mut String) {
        config.push_str(GLOBAL_ENCAPSULATED_SPACE);
        config.push_str(GLOBAL_ENCAPSULATED_CLASS);
        config.push_str(FILTER_ALLOW_CLASS);
        config.push_str(RELAY_AGENT_SUBSCRIBER_ID_OPTION_DEFINITION);
    }
}
