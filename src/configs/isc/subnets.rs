use std::{collections::HashMap, fmt::Display, net::Ipv4Addr};

use crate::{configs::isc::ISCOption, helpers::render_template};

#[derive(Debug, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum ISCSubnetV4Type {
    Both,
    BOOTP,
    DHCP,
}

impl Display for ISCSubnetV4Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ISCSubnetV4Type::Both => f.write_str("Both"),
            ISCSubnetV4Type::DHCP => f.write_str("DHCP"),
            ISCSubnetV4Type::BOOTP => f.write_str("BOOTP"),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ISCPoolV4 {
    pub start_range: Ipv4Addr,
    pub end_range: Ipv4Addr,

    pub classes_names: Option<Vec<String>>,
}

const POOL_SERIALIZER_TEMPLATE: &str = r#"
pool {
	range {{start_range}} {{end_range}};

	{%- if classes_names %}
	{{classes_names}}
	{%- endif %}
}
"#;

impl Display for ISCPoolV4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut arguments: HashMap<&str, Option<String>> = HashMap::new();

        arguments.insert("start_range", Some(self.start_range.to_string()));
        arguments.insert("end_range", Some(self.end_range.to_string()));

        let classes_names: Option<String> = self.classes_names.as_ref().map(|vec| {
            vec.iter()
                .map(|item| format!("allow members of \"{}\";", item))
                .collect::<Vec<String>>()
                .join("\n\t")
        });

        arguments.insert("classes_names", classes_names);

        f.write_str(render_template(POOL_SERIALIZER_TEMPLATE, arguments).as_str())
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ISCSubnetV4 {
    pub id: Ipv4Addr,
    pub netmask: Ipv4Addr,
    pub pools: Vec<ISCPoolV4>,

    pub r#type: ISCSubnetV4Type,

    pub min_lease_time: i32,
    pub default_lease_time: i32,
    pub max_lease_time: i32,

    pub options: Option<Vec<ISCOption>>,
}

const SUBNET_SERIALIZER_TEMPLATE: &str = r#"
subnet {{id}} netmask {{netmask}} {
{{pools}}

min-lease-time {{min_lease_time}};
default-lease-time {{default_lease_time}};
max-lease-time {{max_lease_time}};

{%- if clients_type == "DHCP" %}
deny bootp;
{%- endif %}

{%- if options %}
{{options}}
{%- endif %}
}
"#;

impl Display for ISCSubnetV4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut arguments: HashMap<&str, Option<String>> = HashMap::new();

        arguments.insert("id", Some(self.id.to_string()));
        arguments.insert("netmask", Some(self.netmask.to_string()));
        arguments.insert("min_lease_time", Some(self.min_lease_time.to_string()));
        arguments.insert(
            "default_lease_time",
            Some(self.default_lease_time.to_string()),
        );
        arguments.insert("max_lease_time", Some(self.max_lease_time.to_string()));
        arguments.insert("clients_type", Some(self.r#type.to_string()));

        let options: Option<String> = self.options.as_ref().map(|vec| {
            vec.iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        });

        arguments.insert("options", options);

        let pools: Option<String> = Some(
            self.pools
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        );

        arguments.insert("pools", pools);

        f.write_str(render_template(SUBNET_SERIALIZER_TEMPLATE, arguments).as_str())
    }
}
