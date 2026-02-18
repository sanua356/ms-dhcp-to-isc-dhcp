use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ISCOptionDefinitionType {
    Boolean,
    Int8,
    Int16,
    Int32,
    UInt8,
    UInt16,
    UInt32,
    IPv4Address,
    IPv6Address,
    Text,
    DataString,
    DomainList,
    Encapsulate(String),
    Arrays(Box<ISCOptionDefinitionType>),
    Records(Vec<ISCOptionDefinitionType>),
}

static TYPE_COMPAT: &[(ISCOptionDefinitionType, &str)] = &[
    (ISCOptionDefinitionType::Boolean, "boolean"),
    (ISCOptionDefinitionType::Int8, "integer 8"),
    (ISCOptionDefinitionType::Int16, "integer 16"),
    (ISCOptionDefinitionType::Int32, "integer 32"),
    (ISCOptionDefinitionType::UInt8, "unsigned integer 8"),
    (ISCOptionDefinitionType::UInt16, "unsigned integer 16"),
    (ISCOptionDefinitionType::UInt32, "unsigned integer 32"),
    (ISCOptionDefinitionType::IPv4Address, "ip-address"),
    (ISCOptionDefinitionType::IPv6Address, "ip6-address"),
    (ISCOptionDefinitionType::Text, "text"),
    (ISCOptionDefinitionType::DataString, "string"),
    (ISCOptionDefinitionType::DomainList, "domain-list"),
];

impl Display for ISCOptionDefinitionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let compat = TYPE_COMPAT.iter().find(|item| self == &item.0);

        match compat {
            Some(r#type) => f.write_str(r#type.1),
            None => match self {
                ISCOptionDefinitionType::Encapsulate(_) => {
                    write!(f, "text")
                }
                ISCOptionDefinitionType::Arrays(inner) => {
                    write!(f, "array of {}", inner)
                }
                ISCOptionDefinitionType::Records(inner) => {
                    let subtypes: Vec<String> = inner.iter().map(|item| item.to_string()).collect();
                    write!(f, "array of {{ {} }}", subtypes.join(", "))
                }
                _ => f.write_str("string"),
            },
        }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ISCOptionDefinition {
    pub code: u8,
    pub name: String,
    pub r#type: ISCOptionDefinitionType,

    pub vendor_class: Option<String>,
}

const SERIALIZER_TEMPLATE: &str = r#"option {name} code {code} = {type};"#;

impl Display for ISCOptionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut name = self.name.clone();

        if let Some(vendor_class) = &self.vendor_class {
            name = format!("{}.{}", vendor_class, name);
        }

        let formatted = SERIALIZER_TEMPLATE
            .replace("{name}", &name)
            .replace("{code}", &self.code.to_string())
            .replace("{type}", self.r#type.to_string().as_str());

        f.write_str(formatted.as_str())
    }
}
