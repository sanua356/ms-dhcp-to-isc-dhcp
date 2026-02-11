#![allow(dead_code)]

pub mod option_definitions;

pub struct ISCDHCP {
    option_definitions: Vec<option_definitions::ISCOptionDefinition>,
}
