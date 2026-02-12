#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ISCClass {
    pub name: String,
    pub condition: String,
}
