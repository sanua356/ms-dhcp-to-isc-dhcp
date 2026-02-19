use crate::{
    configs::{
        ISCDHCP,
        isc::{ISCHost, ISCSubclass},
        microsoft::{MicrosoftFilterListType, MicrosoftFilters},
    },
    constants::FILTER_ALLOW_CLASS_NAME,
};

static DENY_FILTER_GROUP_TEMPLATE: &str = r#"group {
	deny booting;

{deny-filter-hosts}
}
"#;

impl ISCDHCP {
    pub fn transform_filters(&mut self, filters: &MicrosoftFilters) {
        if filters.deny {
            let mut deny_hosts: Vec<ISCHost> = Vec::new();

            let mut counter: i32 = 0;
            for host in &filters.items {
                if host.list != MicrosoftFilterListType::Deny {
                    continue;
                }

                deny_hosts.push(ISCHost {
                    name: format!("deny-host-{counter}"),
                    mac_address: Some(host.mac_address.replace("-", ":")),
                    fixed_address: None,
                });
                counter += 1;
            }

            self.deny_filter_hosts.extend(deny_hosts);
        }

        if filters.allow {
            let mut allow_subclasses: Vec<ISCSubclass> = Vec::new();

            for host in &filters.items {
                if host.list != MicrosoftFilterListType::Allow {
                    continue;
                }

                allow_subclasses.push(ISCSubclass {
                    parent_name: FILTER_ALLOW_CLASS_NAME.to_string(),
                    name: host.mac_address.replace("-", ":"),
                    condition: None,
                    vendor_option_space: None,
                });
            }

            self.allow_filter_subclasses.extend(allow_subclasses);
        }
    }

    pub fn write_transformed_filters(&self, config: &mut String) {
        let deny_hosts = DENY_FILTER_GROUP_TEMPLATE.replace(
            "{deny-filter-hosts}",
            self.deny_filter_hosts
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
                .join("\n")
                .as_str(),
        );
        config.push_str(deny_hosts.as_str());

        let allow_hosts = self
            .allow_filter_subclasses
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();
        config.push_str(allow_hosts.join("\n").as_str());
    }
}
