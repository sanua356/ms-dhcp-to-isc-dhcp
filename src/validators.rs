use serde::{Deserialize, Deserializer};

use crate::constants::{BYTES_WITH_PREFIX_REGEXP, MAC_ADDRESS_REGEXP};

pub fn validate_hex_string_optional<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(ref value) if !BYTES_WITH_PREFIX_REGEXP.is_match(value) => Err(
            serde::de::Error::custom(format!("Invalid format for hex string: {}", value)),
        ),
        _ => Ok(opt),
    }
}

pub fn validate_mac_address_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if !MAC_ADDRESS_REGEXP.is_match(&s) {
        Err(serde::de::Error::custom(format!(
            "Invalid format for MAC address string: {}",
            s
        )))
    } else {
        Ok(s)
    }
}

#[cfg(test)]
mod test {
    use quick_xml::de::from_str;
    use serde::Deserialize;

    use super::{validate_hex_string_optional, validate_mac_address_string};

    #[derive(Deserialize)]
    struct TestValidateHEXOptional {
        #[serde(deserialize_with = "validate_hex_string_optional")]
        #[serde(rename = "Field")]
        field: Option<String>,
    }

    #[derive(Deserialize)]
    struct TestValidateMAC {
        #[serde(deserialize_with = "validate_mac_address_string")]
        #[serde(rename = "Field")]
        field: String,
    }

    #[test]
    fn validate_hex_string_optional_correct_test() {
        const TEST_TEMPLATE: &'static str = r#"<TestValidateHEXOptional>
                <Field>0x0102AA</Field>
            </TestValidateHEXOptional> "#;
        let x: TestValidateHEXOptional = from_str(TEST_TEMPLATE).unwrap();
        assert_eq!(x.field, Some(String::from("0x0102AA")));
    }

    #[test]
    fn validate_hex_string_optional_incorrect_test() {
        const TEST_TEMPLATE: &'static str = r#"<TestValidateHEXOptional>
                <Field>INVALID VALUE</Field>
            </TestValidateHEXOptional> "#;

        if let Err(e) = from_str::<TestValidateHEXOptional>(TEST_TEMPLATE) {
            assert_eq!(
                e.to_string(),
                "Invalid format for hex string: INVALID VALUE"
            );
        }
    }

    #[test]
    fn validate_mac_string_correct_test() {
        const TEST_TEMPLATE: &'static str = r#"<TestValidateHEXOptional>
                <Field>11-22-33-44-55-66</Field>
            </TestValidateHEXOptional> "#;
        let x: TestValidateMAC = from_str(TEST_TEMPLATE).unwrap();
        assert_eq!(x.field, String::from("11-22-33-44-55-66"));
    }

    #[test]
    fn validate_mac_string_incorrect_test() {
        const TEST_TEMPLATE: &'static str = r#"<TestValidateHEXOptional>
                <Field>INCORRECT MAC</Field>
            </TestValidateHEXOptional> "#;

        if let Err(e) = from_str::<TestValidateMAC>(TEST_TEMPLATE) {
            assert_eq!(
                e.to_string(),
                "Invalid format for MAC address string: INCORRECT MAC"
            );
        }
    }
}
