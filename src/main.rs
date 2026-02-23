use quick_xml::de::from_str;
use std::fs;

use crate::{
    configs::{ISCDHCP, MicrosoftDHCP},
    constants::DEFAULT_PADDING,
};

mod configs;
mod constants;
mod helpers;
mod transformers;
mod validators;

fn main() {
    let config_data: String = fs::read_to_string("dhcp2016.xml").unwrap();
    let microsoft_config: MicrosoftDHCP = from_str(&config_data).unwrap();

    let mut isc_config: ISCDHCP = ISCDHCP::default();

    let defs = microsoft_config.ipv4.option_definitions.unwrap().items;
    let classes = microsoft_config.ipv4.classes.unwrap().items;
    let filters = microsoft_config.ipv4.filters.unwrap();

    isc_config.transform_option_definitions(&defs);
    isc_config.transform_options(&microsoft_config.ipv4.option_values.unwrap().items, &defs);
    isc_config.transform_classes(&classes);
    isc_config.transform_policies(
        &microsoft_config.ipv4.policies.unwrap().items,
        &defs,
        &classes,
    );
    isc_config.transform_filters(&filters);
    isc_config.transform_scopes_v4(
        &microsoft_config.ipv4.scopes.unwrap().items,
        &defs,
        &classes,
        &filters,
    );

    let mut x = String::new();
    isc_config.write_internal_configuration_parameters(&mut x);
    x.push_str(DEFAULT_PADDING);
    isc_config.write_transformed_classes_to_spaces(&mut x);
    x.push_str(DEFAULT_PADDING);
    isc_config.write_transformed_option_definitions(&mut x);
    x.push_str(DEFAULT_PADDING);
    isc_config.write_transformed_options(&mut x);
    x.push_str(DEFAULT_PADDING);
    isc_config.write_transformed_classes(&mut x);
    x.push_str(DEFAULT_PADDING);
    isc_config.write_transformed_filters(&mut x);
    x.push_str(DEFAULT_PADDING);
    isc_config.write_transformed_policies(&mut x);
    x.push_str(DEFAULT_PADDING);
    isc_config.write_transformed_scopes(&mut x);

    fs::write("output.conf", x).unwrap();

    println!("Config transformed successfully!");
}
