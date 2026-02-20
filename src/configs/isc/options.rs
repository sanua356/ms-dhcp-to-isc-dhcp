use std::{collections::HashMap, fmt::Display};

use crate::{configs::isc::ISCOptionDefinitionType, helpers::render_template};

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ISCOption {
    pub name: String,
    pub space: Option<String>,
    pub value: Vec<String>,

    pub r#type: ISCOptionDefinitionType,
}

fn escape_option_value(value: String, r#type: ISCOptionDefinitionType) -> String {
    match r#type {
        ISCOptionDefinitionType::DataString
        | ISCOptionDefinitionType::Text
        | ISCOptionDefinitionType::DomainList => format!("\"{value}\""),
        ISCOptionDefinitionType::Arrays(inner) => escape_option_value(value, *inner),
        _ => value,
    }
}

const SERIALIZER_TEMPLATE: &str =
    r#"option {% if space %}{{space}}.{% endif %}{{name}} {{value}};"#;

impl Display for ISCOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut arguments: HashMap<&str, Option<String>> = HashMap::new();
        arguments.insert("name", Some(self.name.clone()));
        arguments.insert("space", self.space.clone());

        let value: String = match &self.r#type {
            ISCOptionDefinitionType::Records(inner) => {
                let mut output: Vec<String> = Vec::with_capacity(self.value.len());

                for (idx, subvalue) in self.value.iter().enumerate() {
                    output.push(escape_option_value(subvalue.clone(), inner[idx].clone()));
                }

                format!("{{ {} }}", output.join(", "))
            }
            ISCOptionDefinitionType::Arrays(inner) => {
                let mut output: Vec<String> = Vec::with_capacity(self.value.len());

                for subvalue in self.value.iter() {
                    output.push(escape_option_value(subvalue.clone(), *inner.clone()));
                }

                output.join(", ")
            }
            any => self
                .value
                .clone()
                .into_iter()
                .map(|item| escape_option_value(item, any.clone()))
                .collect::<Vec<String>>()
                .join(", "),
        };
        arguments.insert("value", Some(value));

        f.write_str(render_template(SERIALIZER_TEMPLATE, arguments).as_str())
    }
}
