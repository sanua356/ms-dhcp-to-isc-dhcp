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
