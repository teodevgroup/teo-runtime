use std::fmt::{Display, Formatter};
use serde::Serialize;
use crate::arguments::Arguments;

#[derive(Debug, Clone, Serialize)]
pub struct InterfaceEnumVariant {
    pub value: String,
    pub args: Option<Arguments>,
}

impl InterfaceEnumVariant {

    pub fn value_only(value: String) -> Self {
        Self { value, args: None }
    }

    pub fn new(value: String, args: Arguments) -> Self {
        Self { value, args: Some(args) }
    }

    pub fn args(&self) -> Option<&Arguments> {
        self.args.as_ref()
    }

    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}

impl Display for InterfaceEnumVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.value())
    }
}