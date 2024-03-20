use crate::value::interface_enum_variant::InterfaceEnumVariant;
use crate::value::Value;

impl From<InterfaceEnumVariant> for Value {

    fn from(value: InterfaceEnumVariant) -> Self {
        Value::InterfaceEnumVariant(value)
    }
}

impl From<&InterfaceEnumVariant> for Value {

    fn from(value: &InterfaceEnumVariant) -> Self {
        Value::InterfaceEnumVariant(value.clone())
    }
}

impl From<Option<InterfaceEnumVariant>> for Value {

    fn from(value: Option<InterfaceEnumVariant>) -> Self {
        match value {
            Some(value) => Value::InterfaceEnumVariant(value),
            None => Value::Null,
        }
    }
}

impl From<Option<&InterfaceEnumVariant>> for Value {

    fn from(value: Option<&InterfaceEnumVariant>) -> Self {
        match value {
            Some(value) => Value::InterfaceEnumVariant(value.clone()),
            None => Value::Null,
        }
    }
}