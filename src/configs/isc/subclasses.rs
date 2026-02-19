use std::{collections::HashMap, fmt::Display};

use crate::helpers::render_template;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ISCSubclass {
    pub parent_name: String,
    pub name: String,
    pub condition: Option<String>,

    pub vendor_option_space: Option<String>,
}

const SERIALIZER_TEMPLATE: &str = r#"
subclass "{{parent_name}}" "{{name}}" {
	{%- if condition %}
	match {{condition}};
	{%- endif %}
	{%- if vendor_option_space %}
	vendor-option-space {{vendor_option_space}};
	{%- endif %}
}
"#;

impl Display for ISCSubclass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut arguments: HashMap<&str, Option<String>> = HashMap::new();

        arguments.insert("parent_name", Some(self.parent_name.clone()));
        arguments.insert("name", Some(self.name.clone()));
        arguments.insert("condition", self.condition.clone());
        arguments.insert("vendor_option_space", self.vendor_option_space.clone());

        f.write_str(render_template(SERIALIZER_TEMPLATE, arguments).as_str())
    }
}
