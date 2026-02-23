use clap::Parser;
use quick_xml::de::from_str;
use std::{env, fs, path::PathBuf};

use crate::{
    configs::{ISCDHCP, MicrosoftDHCP},
    transliterator::transliterate,
};

#[derive(Debug, Parser)]
#[command(
    version,
    about = "A command-line utility for converting Microsoft Server 2016-2025 configuration to ISC DHCP."
)]
pub struct CLIArgs {
    #[arg(
        long,
        help = "Required. Specifies the path to the Microsoft Server DHCP configuration file."
    )]
    microsoft_filepath: String,

    #[arg(
        long,
        help = "Optional. Specifies the path to the ISC DHCP configuration file."
    )]
    isc_filepath: Option<String>,

    #[arg(
        long,
        help = "Optional. Performs the transliteration process (GOST 7.79-2000. System 'B'./ISO 9:1995) before starting migration for Microsoft Server DHCP configuration."
    )]
    with_transliterate: bool,
}

pub fn run_cli(arguments: CLIArgs) {
    let config_filepath = PathBuf::from(arguments.microsoft_filepath);
    let mut config_data: String = fs::read_to_string(config_filepath).unwrap();

    if arguments.with_transliterate {
        config_data = transliterate(config_data);
    }

    let microsoft_config: MicrosoftDHCP = from_str(&config_data).unwrap();

    let microsoft_config_version = format!(
        "{}.{}",
        &microsoft_config.major_version, &microsoft_config.minor_version
    );

    if microsoft_config_version != "10.0" {
        panic!("The Microsoft Server DHCP configuration version must be 10.0.");
    }

    let mut output_filepath = env::current_dir().unwrap().join("dhcpd.conf");

    if let Some(isc_filepath) = arguments.isc_filepath {
        output_filepath = PathBuf::from(isc_filepath);
    }

    let mut isc_config: ISCDHCP = ISCDHCP::default();
    isc_config.transform_v4(microsoft_config);
    let output = isc_config.write_v4();

    fs::write(output_filepath, output).unwrap();

    println!("Config transformed successfully!");
}
