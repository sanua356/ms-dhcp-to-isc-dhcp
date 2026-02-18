use crate::{
    configs::{
        isc::{ISCClass, ISCDHCP},
        microsoft::{MicrosoftClass, MicrosoftClassType},
    },
    helpers::{format_string_isc, hex_to_ascii},
};

impl ISCDHCP {
    pub fn transform_classes(&mut self, microsoft_classes: &[MicrosoftClass]) {
        let mut classes: Vec<ISCClass> = Vec::new();

        for ms_class in microsoft_classes {
            let class_name = format_string_isc(&ms_class.name);
            let class_data =
                hex_to_ascii(ms_class.data.as_ref().unwrap_or(&String::new()).as_str());
            let class_condition = match ms_class.r#type {
                MicrosoftClassType::Vendor => {
                    format!("if option vendor-class-identifier = \"{}\"", class_data)
                }
                MicrosoftClassType::User => {
                    format!("if option user-class = \"{}\"", class_data)
                }
            };

            classes.push(ISCClass {
                name: class_name.clone(),
                condition: class_condition,
                vendor_option_space: Some(format!("{}-SPACE", class_name)),
            });
        }

        self.classes.extend(classes);
    }

    pub fn write_transformed_classes(&self, config: &mut String) {
        for class in self.classes.iter() {
            config.push_str(class.to_string().as_str());
        }
    }
}

#[cfg(test)]
mod _tests;

#[cfg(test)]
mod test {
    use quick_xml::de::from_str;

    use super::_tests::{CLASSES_ISC_TEST_TEMPLATE, CLASSES_XML_TEST_TEMPLATE};

    use crate::configs::{ISCDHCP, microsoft::MicrosoftClass};

    #[test]
    fn transform_classes_test() {
        let data: Vec<MicrosoftClass> = from_str(CLASSES_XML_TEST_TEMPLATE).unwrap();

        let mut isc_config: ISCDHCP = ISCDHCP::default();
        isc_config.transform_classes(&data);

        for (idx, item) in isc_config.classes.iter().enumerate() {
            if item != &CLASSES_ISC_TEST_TEMPLATE[idx] {
                panic!("{:?}, {:?}", item, CLASSES_ISC_TEST_TEMPLATE[idx]);
            }
        }

        assert!(true);
    }
}
