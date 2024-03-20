use crate::model;
use crate::value::Value;

impl From<model::Object> for Value {

    fn from(value: model::Object) -> Self {
        Value::ModelObject(value)
    }
}

impl From<&model::Object> for Value {

    fn from(value: &model::Object) -> Self {
        Value::ModelObject(value.clone())
    }
}

impl From<Option<model::Object>> for Value {
    fn from(value: Option<model::Object>) -> Self {
        match value {
            Some(value) => Value::ModelObject(value),
            None => Value::Null,
        }
    }
}

impl From<Option<&model::Object>> for Value {
    fn from(value: Option<&model::Object>) -> Self {
        match value {
            Some(value) => Value::ModelObject(value.clone()),
            None => Value::Null,
        }
    }
}