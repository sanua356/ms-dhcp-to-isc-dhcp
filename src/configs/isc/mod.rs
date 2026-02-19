#![allow(dead_code)]

pub mod classes;
pub mod hosts;
pub mod option_definitions;

pub use classes::*;
pub use option_definitions::*;

use crate::constants::{FILTER_ALLOW_CLASS, GLOBAL_ENCAPSULATED_CLASS, GLOBAL_ENCAPSULATED_SPACE};

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct ISCDHCP {
    pub option_definitions: Vec<option_definitions::ISCOptionDefinition>,
    pub classes: Vec<classes::ISCClass>,
}

impl ISCDHCP {
    pub fn default() -> Self {
        ISCDHCP {
            option_definitions: vec![],
            classes: vec![],
        }
    }

    pub fn write_internal_configuration_parameters(&self, config: &mut String) {
        config.push_str(GLOBAL_ENCAPSULATED_SPACE);
        config.push_str(GLOBAL_ENCAPSULATED_CLASS);
        config.push_str(FILTER_ALLOW_CLASS);
    }
}
