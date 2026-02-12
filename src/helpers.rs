#![allow(dead_code)]

pub fn hex_to_ascii(hex_str: &str) -> String {
    let bytes = hex_str.trim_start_matches("0x").as_bytes();

    let mut ascii_string = String::new();
    for chunk in bytes.chunks_exact(2) {
        if let Ok(byte_value) = u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16) {
            ascii_string.push(byte_value as char);
        }
    }

    ascii_string
}

pub fn format_string_isc(source: &str) -> String {
    let mut output = source.to_lowercase().replace(" ", "-");

    if output.starts_with("-") {
        output = output[1..output.len()].to_string();
    }

    if output.ends_with("-") {
        output = output[0..output.len() - 1].to_string();
    }

    output
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{format_string_isc, hex_to_ascii};

    #[rstest]
    #[case(
        String::from("0x525241532e4d6963726f736f6674"),
        String::from("RRAS.Microsoft")
    )]
    #[case(
        String::from("0x424f4f54502e4d6963726f736f6674"),
        String::from("BOOTP.Microsoft")
    )]
    #[case(String::from("0x4d53465420352e30"), String::from("MSFT 5.0"))]
    #[case(String::from("0x4d534654203938"), String::from("MSFT 98"))]
    fn hex_to_ascii_test(#[case] source: String, #[case] output: String) {
        assert_eq!(hex_to_ascii(&source), output);
    }

    #[test]
    #[should_panic]
    fn hex_to_ascii_test_panic() {
        let a = hex_to_ascii("$R!@RFFQ@WGWSAEGASEGE");
        assert_eq!("100", a);
    }

    #[rstest]
    #[case(
        String::from("Default Routing and Remote Access Class"),
        String::from("default-routing-and-remote-access-class")
    )]
    #[case(
        String::from(" STRING WITH SPACES IN START AND END "),
        String::from("string-with-spaces-in-start-and-end")
    )]
    fn format_string_isc_test(#[case] source: String, #[case] output: String) {
        assert_eq!(format_string_isc(&source), output);
    }
}
