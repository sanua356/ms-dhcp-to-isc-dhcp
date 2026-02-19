use std::{collections::HashMap, fmt::Display};

use crate::helpers::render_template;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ISCClass {
    pub name: String,
    pub condition: String,

    pub vendor_option_space: Option<String>,
}

const SERIALIZER_TEMPLATE: &str = r#"
class "{{name}}" {
	match {{condition}};
	{%- if name %}
	vendor-option-space {{vendor_option_space}};
	{%- endif %}
}
"#;

impl Display for ISCClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut arguments: HashMap<&str, Option<String>> = HashMap::new();
        arguments.insert("name", Some(self.name.clone()));
        arguments.insert("condition", Some(self.condition.clone()));
        arguments.insert("vendor_option_space", self.vendor_option_space.clone());

        f.write_str(render_template(SERIALIZER_TEMPLATE, arguments).as_str())
    }
}
