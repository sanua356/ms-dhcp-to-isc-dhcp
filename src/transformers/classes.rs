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
                MicrosoftClassType::User => {
                    format!(
                        "match if option vendor-class-identifier = \"{}\";",
                        class_data
                    )
                }
                MicrosoftClassType::Vendor => {
                    format!("match if option user-class = \"{}\";", class_data)
                }
            };

            classes.push(ISCClass {
                name: class_name,
                condition: class_condition,
            });
        }

        self.classes.extend(classes);
    }
}
