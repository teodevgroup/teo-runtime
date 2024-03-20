use crate::r#struct;
use crate::value::Value;

impl From<r#struct::Object> for Value {

    fn from(value: r#struct::Object) -> Self {
        Value::StructObject(value)
    }
}

impl From<Option<r#struct::Object>> for Value {
    fn from(value: Option<r#struct::Object>) -> Self {
        match value {
            Some(v) => Value::StructObject(v),
            None => Value::Null,
        }
    }
}

impl From<&r#struct::Object> for Value {

    fn from(value: &r#struct::Object) -> Self {
        Value::StructObject(value.clone())
    }
}

impl From<Option<&r#struct::Object>> for Value {
    fn from(value: Option<&r#struct::Object>) -> Self {
        match value {
            Some(v) => Value::StructObject(v.clone()),
            None => Value::Null,
        }
    }
}