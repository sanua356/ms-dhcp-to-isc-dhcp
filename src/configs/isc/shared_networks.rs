use std::{collections::HashMap, fmt::Display};

use crate::{configs::isc::ISCSubnetV4, helpers::render_template};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ISCSharedNetworkV4 {
    pub name: String,
    pub subnets: Vec<ISCSubnetV4>,
}

const SERIALIZER_TEMPLATE: &str = r#"
shared-network "{{name}}" {
{{subnets}}
}
"#;

impl Display for ISCSharedNetworkV4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut arguments: HashMap<&str, Option<String>> = HashMap::new();

        arguments.insert("name", Some(self.name.clone()));

        let subnets: Option<String> = Some(
            self.subnets
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        );

        arguments.insert("subnets", subnets);

        f.write_str(render_template(SERIALIZER_TEMPLATE, arguments).as_str())
    }
}
