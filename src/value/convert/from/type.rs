use teo_parser::r#type::Type;
use crate::value::Value;


impl From<Type> for Value {

    fn from(v: Type) -> Self {
        Value::Type(v)
    }
}

impl From<Option<Type>> for Value {

    fn from(value: Option<Type>) -> Self {
        match value {
            Some(s) => Value::Type(s),
            None => Value::Null,
        }
    }
}
