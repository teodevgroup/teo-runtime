use std::fmt::{Display, Formatter};
use futures_util::StreamExt;
use serde::Serialize;
use crate::arguments::Arguments;
use teo_parser::value::interface_enum_variant::InterfaceEnumVariant as ParserInterfaceEnumVariant;

#[derive(Debug, Clone, Serialize, PartialEq)]
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

    pub fn normal_not(&self) -> bool {
        false
    }
}

impl Display for InterfaceEnumVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.value())?;
        if let Some(args) = self.args() {
            f.write_str("(")?;
            Display::fmt(args, f)?;
            f.write_str(")")?;
        }
        Ok(())
    }
}

impl From<ParserInterfaceEnumVariant> for InterfaceEnumVariant {
    fn from(value: ParserInterfaceEnumVariant) -> Self {
        Self {
            value: value.value,
            args: value.args.map(|args| Arguments::new(args.into_iter().map(|(k, v)| (k, v.into())).collect()))
        }
    }
}