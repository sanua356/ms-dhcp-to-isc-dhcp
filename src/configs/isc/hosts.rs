use std::{fmt::Display, net::Ipv4Addr};

#[derive(Debug)]
pub struct ISCHost {
    pub name: String,
    pub mac_address: Option<String>,
    pub fixed_address: Option<Vec<Ipv4Addr>>,
    // TODO: It doesn't exist yet options
    // pub options: Option<Vec<>>
}

const SERIALIZER_TEMPLATE: &str = r#"host {name} {
	{mac-address}
	{fixed-address}
}
"#;

impl Display for ISCHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatted = SERIALIZER_TEMPLATE.replace("{name}", &self.name);

        if let Some(mac_address) = &self.mac_address {
            formatted = formatted.replace(
                "{mac-address}",
                format!("hardware ethernet {mac_address};").as_str(),
            );
        } else {
            formatted = formatted.replace("{mac-address}", "");
        }

        if let Some(fixed_address) = &self.fixed_address {
            formatted = formatted.replace(
                "{fixed-address}",
                format!(
                    "fixed-address {};",
                    fixed_address
                        .iter()
                        .map(|item| item.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
                .as_str(),
            );
        } else {
            formatted = formatted.replace("{fixed-address}", "");
        }

        f.write_str(formatted.as_str())
    }
}
