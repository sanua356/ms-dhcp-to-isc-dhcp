use std::{collections::HashMap, fmt::Display, net::Ipv4Addr};

use crate::{configs::isc::ISCOption, helpers::render_template};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ISCHost {
    pub name: String,
    pub mac_address: Option<String>,
    pub fixed_address: Option<Vec<Ipv4Addr>>,

    pub options: Option<Vec<ISCOption>>,
}

const SERIALIZER_TEMPLATE: &str = r#"
host {{name}} {
	{%- if mac_address %}
	hardware ethernet {{mac_address}};
	{%- endif %}
	{%- if fixed_address %}
	fixed-address {{fixed_address}};
	{%- endif %}
	{%- if options %}
	{{options}}
	{%- endif %}
}
"#;

impl Display for ISCHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut arguments: HashMap<&str, Option<String>> = HashMap::new();
        arguments.insert("name", Some(self.name.clone()));
        arguments.insert("mac_address", self.mac_address.clone());

        let fixed_address: Option<String> = self.fixed_address.as_ref().map(|vec| {
            vec.iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        });

        arguments.insert("fixed_address", fixed_address);

        let options: Option<String> = self.options.as_ref().map(|vec| {
            vec.iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        });

        arguments.insert("options", options);

        f.write_str(render_template(SERIALIZER_TEMPLATE, arguments).as_str())
    }
}
