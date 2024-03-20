use crate::model;
use teo_result::Error;
use crate::value::Value;

impl<'a> TryFrom<&'a Value> for &'a model::Object {

    type Error = Error;

    fn try_from(value: &'a Value) -> std::result::Result<Self, Self::Error> {
        match value.as_model_object() {
            Some(o) => Ok(o),
            None => Err(Error::new(format!("object is not model object: {:?}", value)))
        }
    }
}

impl TryFrom<&Value> for model::Object {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        match value.as_model_object() {
            Some(o) => Ok(o.clone()),
            None => Err(Error::new(format!("object is not model object: {:?}", value)))
        }
    }
}

impl TryFrom<Value> for model::Object {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        match value.as_model_object() {
            Some(o) => Ok(o.clone()),
            None => Err(Error::new(format!("object is not model object: {:?}", value)))
        }
    }
}

impl<'a> TryFrom<&'a Value> for Option<&'a model::Object> {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::ModelObject(o) => Ok(Some(o)),
            Value::Null => Ok(None),
            _ => Err(Error::new(format!("object is not model object or null: {:?}", value)))
        }
    }
}

impl TryFrom<&Value> for Option<model::Object> {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        match value {
            Value::ModelObject(o) => Ok(Some(o.clone())),
            Value::Null => Ok(None),
            _ => Err(Error::new(format!("object is not model object or null: {:?}", value)))
        }
    }
}

impl TryFrom<Value> for Option<model::Object> {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::ModelObject(o) => Ok(Some(o.clone())),
            Value::Null => Ok(None),
            _ => Err(Error::new(format!("object is not model object or null: {:?}", value)))
        }
    }
}