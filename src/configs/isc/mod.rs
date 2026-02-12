#![allow(dead_code)]

pub mod classes;
pub mod option_definitions;

pub use classes::*;
pub use option_definitions::*;

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct ISCDHCP {
    pub option_definitions: Vec<option_definitions::ISCOptionDefinition>,
    pub classes: Vec<classes::ISCClass>,
}
