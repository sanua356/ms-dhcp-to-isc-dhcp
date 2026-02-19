#![allow(dead_code)]

pub mod classes;
pub mod hosts;
pub mod option_definitions;
pub mod subclasses;

pub use classes::*;
pub use hosts::*;
pub use option_definitions::*;
pub use subclasses::*;

use crate::constants::{FILTER_ALLOW_CLASS, GLOBAL_ENCAPSULATED_CLASS, GLOBAL_ENCAPSULATED_SPACE};

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct ISCDHCP {
    pub option_definitions: Vec<option_definitions::ISCOptionDefinition>,
    pub classes: Vec<classes::ISCClass>,
    pub deny_filter_hosts: Vec<hosts::ISCHost>,
    pub allow_filter_subclasses: Vec<subclasses::ISCSubclass>,
}

impl ISCDHCP {
    pub fn default() -> Self {
        ISCDHCP {
            option_definitions: vec![],
            classes: vec![],
            deny_filter_hosts: vec![],
            allow_filter_subclasses: vec![],
        }
    }

    pub fn write_internal_configuration_parameters(&self, config: &mut String) {
        config.push_str(GLOBAL_ENCAPSULATED_SPACE);
        config.push_str(GLOBAL_ENCAPSULATED_CLASS);
        config.push_str(FILTER_ALLOW_CLASS);
    }
}
