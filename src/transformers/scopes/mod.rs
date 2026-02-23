use std::net::Ipv4Addr;

use regex::Regex;

use crate::{
    configs::{
        ISCDHCP,
        isc::{
            ISCClass, ISCHost, ISCOption, ISCOptionDefinitionType, ISCPoolV4, ISCSubnetV4,
            ISCSubnetV4Type,
        },
        microsoft::{
            MicrosoftClass, MicrosoftFilters, MicrosoftIPRange, MicrosoftOptionDefinition,
            MicrosoftPolicy, MicrosoftScopeType, MicrosoftScopeV4,
        },
    },
    constants::{FILTER_ALLOW_CLASS_NAME, MICROSOFT_STANDARD_CLASSES, POLICY_SUBNET_PREFIX},
    helpers::{integer_to_ipv4_address, ipv4_address_to_integer},
    transformers::{options::ms_options_to_isc_options, policies::ms_policies_to_isc_classes},
};

fn get_allowed_ranges(
    start_range: Ipv4Addr,
    end_range: Ipv4Addr,
    excluded_ranges: &[MicrosoftIPRange],
) -> Vec<MicrosoftIPRange> {
    let mut allowed_ranges: Vec<MicrosoftIPRange> = vec![MicrosoftIPRange {
        start_range,
        end_range,
    }];

    for exclude_range in excluded_ranges {
        let mut temp: Vec<MicrosoftIPRange> = Vec::new();

        let exclude_start = ipv4_address_to_integer(exclude_range.start_range);
        let exclude_end = ipv4_address_to_integer(exclude_range.end_range);

        for allowed_range in allowed_ranges {
            let allowed_start = ipv4_address_to_integer(allowed_range.start_range);
            let allowed_end = ipv4_address_to_integer(allowed_range.end_range);

            // The exclusion range is not within the allowed range
            if exclude_end < allowed_start || exclude_start > allowed_end {
                temp.push(allowed_range);
                continue;
            }
            // The exclusion range only crosses the beginning of the allowed range
            if exclude_start > allowed_start {
                temp.push(MicrosoftIPRange {
                    start_range: integer_to_ipv4_address(allowed_start),
                    end_range: integer_to_ipv4_address(exclude_start.saturating_sub(1)),
                });
            }

            // The exclusion range only crosses the end of the allowed range
            if exclude_end < allowed_end {
                temp.push(MicrosoftIPRange {
                    start_range: integer_to_ipv4_address(exclude_end.saturating_add(1)),
                    end_range: integer_to_ipv4_address(allowed_end),
                });
            }
        }

        allowed_ranges = temp;
    }

    allowed_ranges
}

fn get_lease_time(lease_time: String) -> i32 {
    let re = Regex::new(r"^(\d+)\.(\d{2}):(\d{2}):(\d{2})(?:\.\d+)?$").unwrap();

    if let Some(caps) = re.captures(lease_time.as_str()) {
        let days: i32 = caps[1].parse().unwrap();
        let hours: i32 = caps[2].parse().unwrap();
        let minutes: i32 = caps[3].parse().unwrap();
        let seconds: i32 = caps[4].parse().unwrap();

        let mut sum: i32 = 0;

        // If Microsoft has set the value to "infinity", a large number of days are defined there
        if days > 10000000 {
            return i32::MAX;
        }

        sum += days * 86400;
        sum += hours * 3600;
        sum += minutes * 60;
        sum += seconds;

        return sum;
    }

    0
}

impl ISCDHCP {
    pub fn transform_scopes_v4(
        &mut self,
        microsoft_scopes: &[MicrosoftScopeV4],
        microsoft_option_definitions: &[MicrosoftOptionDefinition],
        microsoft_classes: &[MicrosoftClass],
        microsoft_filters: &MicrosoftFilters,
    ) {
        let mut subnets: Vec<ISCSubnetV4> = Vec::new();
        let mut classes: Vec<ISCClass> = Vec::new();

        for scope in microsoft_scopes {
            let exclusion_ranges = match &scope.exclusion_ranges {
                Some(obj) => &obj.items,
                None => &vec![],
            };
            let policies: &Vec<MicrosoftPolicy> = match &scope.policies {
                Some(obj) => &obj
                    .items
                    .iter()
                    .filter(|item| !MICROSOFT_STANDARD_CLASSES.contains(&item.name.as_str()))
                    .cloned()
                    .collect(),
                None => &vec![],
            };
            let options = match &scope.option_values {
                Some(obj) => &obj.items,
                None => &vec![],
            };
            let ms_pools = get_allowed_ranges(scope.start_range, scope.end_range, exclusion_ranges);

            let policies: Vec<ISCClass> = ms_policies_to_isc_classes(
                policies,
                microsoft_option_definitions,
                microsoft_classes,
            )
            .into_iter()
            .map(|mut item| {
                item.name = format!("{POLICY_SUBNET_PREFIX}-{}", item.name);
                item
            })
            .collect();
            let scope_type = match scope.r#type {
                MicrosoftScopeType::Both => ISCSubnetV4Type::Both,
                MicrosoftScopeType::Dhcp => ISCSubnetV4Type::DHCP,
                MicrosoftScopeType::Bootp => ISCSubnetV4Type::BOOTP,
            };
            let mut classes_names: Vec<String> =
                policies.iter().map(|item| item.name.clone()).collect();

            if microsoft_filters.allow {
                classes_names.push(FILTER_ALLOW_CLASS_NAME.to_string());
            }

            let pools: Vec<ISCPoolV4> = ms_pools
                .into_iter()
                .map(|item| ISCPoolV4 {
                    start_range: item.start_range,
                    end_range: item.end_range,
                    classes_names: Some(classes_names.clone()),
                })
                .collect();

            let lease_time = get_lease_time(scope.lease_duration.to_owned());

            let mut reservations: Vec<ISCHost> = Vec::new();

            if let Some(ms_reservations) = &scope.reservations {
                reservations.extend(
                    ms_reservations
                        .items
                        .iter()
                        .enumerate()
                        .map(|(idx, item)| ISCHost {
                            name: item.name.clone().unwrap_or(format!("reservation-{idx}")),
                            fixed_address: Some(vec![item.ip_address]),
                            mac_address: None,
                            options: Some(vec![ISCOption {
                                name: String::from("dhcp-client-identifier"),
                                space: None,
                                r#type: ISCOptionDefinitionType::Arrays(Box::new(
                                    ISCOptionDefinitionType::UInt8,
                                )),
                                value: vec![item.client_id.replace("-", ":")],
                            }]),
                        })
                        .collect::<Vec<ISCHost>>(),
                );
            }

            classes.extend(policies);
            subnets.push(ISCSubnetV4 {
                id: scope.scope_id,
                netmask: scope.subnet_mask,
                r#type: scope_type,
                pools,
                reservations: Some(reservations),
                options: Some(ms_options_to_isc_options(
                    options,
                    microsoft_option_definitions,
                )),
                default_lease_time: lease_time,
                max_lease_time: lease_time,
                min_lease_time: lease_time,
            });
        }

        self.subnet_v4_classes.extend(classes);
        self.subnets_v4.extend(subnets);
    }

    pub fn write_transformed_scopes(&self, config: &mut String) {
        for class in self.subnet_v4_classes.iter() {
            config.push_str(class.to_string().as_str());
        }
        for subnet in self.subnets_v4.iter() {
            config.push_str(subnet.to_string().as_str());
        }
    }
}

#[cfg(test)]
mod _tests;

#[cfg(test)]
mod test {
    use std::net::Ipv4Addr;

    use quick_xml::de::from_str;
    use rstest::rstest;

    use crate::{
        configs::{
            ISCDHCP,
            microsoft::{
                MicrosoftClass, MicrosoftFilters, MicrosoftIPRange, MicrosoftOptionDefinition,
                MicrosoftScopeV4,
            },
        },
        transformers::scopes::_tests::{
            CLASSES_XML_TEST_TEMPLATE, FILTERS_XML_TEST_TEMPLATE,
            OPTION_DEFINITIONS_XML_TEST_TEMPLATE, SCOPES_TRANSFORMED_TEST_TEMPLATE,
            SCOPES_XML_TEST_TEMPLATE, SUBNETS_ISC_TEST_TEMPLATE,
        },
    };

    use super::get_allowed_ranges;

    #[rstest]
    #[case(
	    Ipv4Addr::new(10, 10, 10, 100),
	    Ipv4Addr::new(10, 10, 10, 200),
	    vec![MicrosoftIPRange {
	        start_range: Ipv4Addr::new(10, 10, 10, 150),
	        end_range: Ipv4Addr::new(10, 10, 10, 170),
	    }],
	    vec![MicrosoftIPRange {
	        start_range: Ipv4Addr::new(10, 10, 10, 100),
	        end_range: Ipv4Addr::new(10, 10, 10, 149),
	    },
		MicrosoftIPRange {
	        start_range: Ipv4Addr::new(10, 10, 10, 171),
	        end_range: Ipv4Addr::new(10, 10, 10, 200),
	    }],
    )]
    #[case(
	    Ipv4Addr::new(10, 10, 10, 0),
	    Ipv4Addr::new(10, 10, 10, 100),
	    vec![MicrosoftIPRange {
	        start_range: Ipv4Addr::new(10, 10, 10, 0),
	        end_range: Ipv4Addr::new(10, 10, 10, 20),
	    },
		MicrosoftIPRange {
	        start_range: Ipv4Addr::new(10, 10, 10, 20),
	        end_range: Ipv4Addr::new(10, 10, 10, 30),
	    }],
	    vec![MicrosoftIPRange {
	        start_range: Ipv4Addr::new(10, 10, 10, 31),
	        end_range: Ipv4Addr::new(10, 10, 10, 100),
	    }],
    )]
    #[case(
	    Ipv4Addr::new(10, 10, 10, 50),
	    Ipv4Addr::new(10, 10, 10, 100),
	    vec![MicrosoftIPRange {
	        start_range: Ipv4Addr::new(10, 10, 10, 20),
	        end_range: Ipv4Addr::new(10, 10, 10, 60),
	    },
		MicrosoftIPRange {
	        start_range: Ipv4Addr::new(10, 10, 10, 80),
	        end_range: Ipv4Addr::new(10, 10, 10, 120),
	    }],
	    vec![MicrosoftIPRange {
	        start_range: Ipv4Addr::new(10, 10, 10, 61),
	        end_range: Ipv4Addr::new(10, 10, 10, 79),
	    }],
    )]
    fn get_allowed_ranges_test(
        #[case] start_range: Ipv4Addr,
        #[case] end_range: Ipv4Addr,
        #[case] excluded_ranges: Vec<MicrosoftIPRange>,
        #[case] output: Vec<MicrosoftIPRange>,
    ) {
        for (idx, item) in get_allowed_ranges(start_range, end_range, &excluded_ranges)
            .iter()
            .enumerate()
        {
            if item != &output[idx] {
                panic!("{:?}, {:?}", item, output[idx]);
            }
        }

        assert!(true);
    }

    #[test]
    fn transform_scopes_test() {
        let ms_option_defs: Vec<MicrosoftOptionDefinition> =
            from_str(OPTION_DEFINITIONS_XML_TEST_TEMPLATE).unwrap();
        let ms_classes: Vec<MicrosoftClass> = from_str(CLASSES_XML_TEST_TEMPLATE).unwrap();
        let ms_filters: MicrosoftFilters = from_str(FILTERS_XML_TEST_TEMPLATE).unwrap();
        let ms_scopes: Vec<MicrosoftScopeV4> = from_str(SCOPES_XML_TEST_TEMPLATE).unwrap();

        let mut isc_config: ISCDHCP = ISCDHCP::default();
        isc_config.transform_scopes_v4(&ms_scopes, &ms_option_defs, &ms_classes, &ms_filters);

        for (idx, item) in isc_config.subnets_v4.iter().enumerate() {
            if item != &SUBNETS_ISC_TEST_TEMPLATE[idx] {
                panic!("{:?}, {:?}", item, SUBNETS_ISC_TEST_TEMPLATE[idx]);
            }
        }

        assert!(true);
    }

    #[test]
    fn write_transformed_scopes_test() {
        let ms_option_defs: Vec<MicrosoftOptionDefinition> =
            from_str(OPTION_DEFINITIONS_XML_TEST_TEMPLATE).unwrap();
        let ms_classes: Vec<MicrosoftClass> = from_str(CLASSES_XML_TEST_TEMPLATE).unwrap();
        let ms_filters: MicrosoftFilters = from_str(FILTERS_XML_TEST_TEMPLATE).unwrap();
        let ms_scopes: Vec<MicrosoftScopeV4> = from_str(SCOPES_XML_TEST_TEMPLATE).unwrap();

        let mut x = String::new();

        let mut isc_config: ISCDHCP = ISCDHCP::default();
        isc_config.transform_scopes_v4(&ms_scopes, &ms_option_defs, &ms_classes, &ms_filters);
        isc_config.write_transformed_scopes(&mut x);

        assert_eq!(x.trim(), SCOPES_TRANSFORMED_TEST_TEMPLATE.trim());
    }
}
