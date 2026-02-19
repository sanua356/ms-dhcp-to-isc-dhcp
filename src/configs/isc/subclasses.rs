use std::fmt::Display;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ISCSubclass {
    pub parent_name: String,
    pub name: String,
    pub condition: Option<String>,

    pub vendor_option_space: Option<String>,
}

const SERIALIZER_TEMPLATE: &str = r#"subclass "{parent_name}" "{name}" {
	{condition}
	{vendor_option_space}
}
"#;

impl Display for ISCSubclass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatted = SERIALIZER_TEMPLATE
            .replace("{parent_name}", &self.parent_name)
            .replace("{name}", &self.name);

        if let Some(condition) = &self.condition {
            formatted = formatted.replace("{condition}", format!("match {condition};").as_str());
        } else {
            formatted = formatted.replace("{condition}", "");
        }

        if let Some(vendor_option_space) = &self.vendor_option_space {
            formatted = formatted.replace(
                "{vendor_option_space}",
                format!("vendor-option-space {vendor_option_space};").as_str(),
            );
        } else {
            formatted = formatted.replace("{vendor_option_space}", "");
        }

        f.write_str(formatted.as_str())
    }
}
