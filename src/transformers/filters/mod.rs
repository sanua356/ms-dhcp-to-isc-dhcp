use std::collections::HashMap;

use crate::{
    configs::{
        ISCDHCP,
        isc::{ISCHost, ISCSubclass},
        microsoft::{MicrosoftFilterListType, MicrosoftFilters},
    },
    constants::FILTER_ALLOW_CLASS_NAME,
    helpers::render_template,
};

static DENY_FILTER_GROUP_TEMPLATE: &str = r#"
group {
	deny booting;
{%- if deny_filter_hosts %}
{{deny_filter_hosts}}
{%- endif %}
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
        let deny_hosts = self
            .deny_filter_hosts
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        let mut arguments: HashMap<&str, Option<String>> = HashMap::new();
        arguments.insert("deny_filter_hosts", Some(deny_hosts));
        config.push_str(render_template(DENY_FILTER_GROUP_TEMPLATE, arguments).as_str());

        let allow_hosts = self
            .allow_filter_subclasses
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();
        config.push_str(allow_hosts.join("\n").as_str());
    }
}

#[cfg(test)]
mod _tests;

#[cfg(test)]
mod test {
    use quick_xml::de::from_str;

    use super::_tests::{
        FILTERS_ALLOW_SUBCLASSES_ISC_TEST_TEMPLATE, FILTERS_DENY_HOSTS_ISC_TEST_TEMPLATE,
        FILTERS_TRANSFORMED_TEST_TEMPLATE, FILTERS_XML_TEST_TEMPLATE,
    };

    use crate::configs::{ISCDHCP, microsoft::MicrosoftFilters};

    #[test]
    fn transform_filters_test() {
        let data: MicrosoftFilters = from_str(FILTERS_XML_TEST_TEMPLATE).unwrap();

        let mut isc_config: ISCDHCP = ISCDHCP::default();
        isc_config.transform_filters(&data);

        for (idx, item) in isc_config.allow_filter_subclasses.iter().enumerate() {
            if item != &FILTERS_ALLOW_SUBCLASSES_ISC_TEST_TEMPLATE[idx] {
                panic!(
                    "{:?}, {:?}",
                    item, FILTERS_ALLOW_SUBCLASSES_ISC_TEST_TEMPLATE[idx]
                );
            }
        }

        for (idx, item) in isc_config.deny_filter_hosts.iter().enumerate() {
            if item != &FILTERS_DENY_HOSTS_ISC_TEST_TEMPLATE[idx] {
                panic!(
                    "{:?}, {:?}",
                    item, FILTERS_DENY_HOSTS_ISC_TEST_TEMPLATE[idx]
                );
            }
        }

        assert!(true);
    }

    #[test]
    fn write_transformed_filters_test() {
        let data: MicrosoftFilters = from_str(FILTERS_XML_TEST_TEMPLATE).unwrap();

        let mut x = String::new();

        let mut isc_config: ISCDHCP = ISCDHCP::default();
        isc_config.transform_filters(&data);
        isc_config.write_transformed_filters(&mut x);

        assert_eq!(x.trim(), FILTERS_TRANSFORMED_TEST_TEMPLATE.trim());
    }
}
