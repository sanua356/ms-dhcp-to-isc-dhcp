use quick_xml::de::from_str;
use std::fs;

use crate::configs::{ISCDHCP, MicrosoftDHCP};

mod configs;
mod constants;
mod helpers;
mod transformers;
mod validators;

fn main() {
    let config_data: String = fs::read_to_string("dhcp2016.xml").unwrap();
    let microsoft_config: MicrosoftDHCP = from_str(&config_data).unwrap();

    let mut isc_config: ISCDHCP = ISCDHCP::default();

    isc_config
        .transform_option_definitions(&microsoft_config.ipv4.option_definitions.unwrap().items);
    isc_config.transform_classes(&microsoft_config.ipv4.classes.unwrap().items);

    println!("{:?}", isc_config);
}
