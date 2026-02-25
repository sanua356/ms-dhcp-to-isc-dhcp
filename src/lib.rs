use wasm_bindgen::prelude::*;

mod configs;
mod constants;
mod helpers;
mod transformers;
mod transliterator;
mod validators;

use quick_xml::de::from_str;

#[wasm_bindgen]
pub fn transform_wasm(input: String, with_transliterate: bool) -> String {
    let mut config_data: String = input;

    if with_transliterate {
        config_data = transliterator::transliterate(config_data);
    }

    let microsoft_config: configs::MicrosoftDHCP = from_str(&config_data).unwrap();

    let microsoft_config_version = format!(
        "{}.{}",
        &microsoft_config.major_version, &microsoft_config.minor_version
    );

    if microsoft_config_version != "10.0" {
        panic!("The Microsoft Server DHCP configuration version must be 10.0.");
    }

    let mut isc_config: configs::ISCDHCP = configs::ISCDHCP::default();
    isc_config.transform_v4(microsoft_config);

    isc_config.write_v4()
}
