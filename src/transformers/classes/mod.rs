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
                vendor_option_space: Some(class_name),
            });
        }

        self.classes.extend(classes);
    }

    pub fn write_transformed_classes(&self, config: &mut String) {
        for class in self.classes.iter() {
            config.push_str(class.to_string().as_str());
        }
    }

    pub fn write_transformed_classes_to_spaces(&self, config: &mut String) {
        config.push_str(
            self.classes
                .iter()
                .map(|item| format!("option space {};", item.name))
                .collect::<Vec<String>>()
                .join("\n")
                .as_str(),
        );
    }
}

#[cfg(test)]
mod _tests;

#[cfg(test)]
mod test {
    use quick_xml::de::from_str;

    use super::_tests::{CLASSES_ISC_TEST_TEMPLATE, CLASSES_XML_TEST_TEMPLATE};

    use crate::{
        configs::{ISCDHCP, microsoft::MicrosoftClass},
        transformers::classes::_tests::{
            CLASSES_TRANSFORMED_TEST_TEMPLATE, SPACES_TRANSFORMED_TEST_TEMPLATE,
        },
    };

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

    #[test]
    fn write_transformed_classes_test() {
        let data: Vec<MicrosoftClass> = from_str(CLASSES_XML_TEST_TEMPLATE).unwrap();

        let mut x = String::new();

        let mut isc_config: ISCDHCP = ISCDHCP::default();
        isc_config.transform_classes(&data);
        isc_config.write_transformed_classes(&mut x);

        assert_eq!(x.trim(), CLASSES_TRANSFORMED_TEST_TEMPLATE.trim());
    }

    #[test]
    fn write_transformed_classes_to_spaces_test() {
        let data: Vec<MicrosoftClass> = from_str(CLASSES_XML_TEST_TEMPLATE).unwrap();

        let mut x = String::new();

        let mut isc_config: ISCDHCP = ISCDHCP::default();
        isc_config.transform_classes(&data);
        isc_config.write_transformed_classes_to_spaces(&mut x);

        assert_eq!(x.trim(), SPACES_TRANSFORMED_TEST_TEMPLATE.trim());
    }
}
