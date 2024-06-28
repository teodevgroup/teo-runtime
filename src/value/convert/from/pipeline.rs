use crate::pipeline::Pipeline;
use crate::value::Value;

impl From<Pipeline> for Value {

    fn from(value: Pipeline) -> Self {
        Value::Pipeline(value)
    }
}

impl From<&Pipeline> for Value {

    fn from(value: &Pipeline) -> Self {
        Value::Pipeline(value.clone())
    }
}

impl From<Option<Pipeline>> for Value {
    fn from(value: Option<Pipeline>) -> Self {
        match value {
            Some(value) => Value::Pipeline(value),
            None => Value::Null,
        }
    }
}

impl From<Option<&Pipeline>> for Value {
    fn from(value: Option<&Pipeline>) -> Self {
        match value {
            Some(value) => Value::Pipeline(value.clone()),
            None => Value::Null,
        }
    }
}
