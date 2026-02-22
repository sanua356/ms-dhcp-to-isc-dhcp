use std::{collections::HashMap, fmt::Display};

use crate::{configs::isc::ISCOption, helpers::render_template};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ISCClass {
    pub name: String,
    pub condition: String,

    pub vendor_option_space: Option<String>,

    pub options: Option<Vec<ISCOption>>,
}

const SERIALIZER_TEMPLATE: &str = r#"
class "{{name}}" {
	match {{condition}};
	{%- if vendor_option_space %}
	vendor-option-space {{vendor_option_space}};
	{%- endif %}
	{%- if options %}
	{{options}}
	{%- endif %}
}
"#;

impl Display for ISCClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut arguments: HashMap<&str, Option<String>> = HashMap::new();
        arguments.insert("name", Some(self.name.clone()));
        arguments.insert("condition", Some(self.condition.clone()));
        arguments.insert("vendor_option_space", self.vendor_option_space.clone());

        let options: Option<String> = self.options.as_ref().map(|vec| {
            vec.iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
                .join("\n\t")
        });

        arguments.insert("options", options);

        f.write_str(render_template(SERIALIZER_TEMPLATE, arguments).as_str())
    }
}
