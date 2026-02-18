use std::fmt::Display;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ISCClass {
    pub name: String,
    pub condition: String,

    pub vendor_option_space: Option<String>,
}

const SERIALIZER_TEMPLATE: &str = r#"class "{name}" {
	match {condition};
	vendor-option-space {vendor_option_space};
}
"#;

impl Display for ISCClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = SERIALIZER_TEMPLATE
            .replace("{name}", &self.name)
            .replace("{condition}", &self.condition)
            .replace(
                "{vendor_option_space}",
                self.vendor_option_space.as_deref().unwrap_or(""),
            );

        f.write_str(formatted.as_str())
    }
}
