use teo_result::Error;
use crate::pipeline::pipeline::Pipeline;
use crate::value::Value;

impl<'a> TryFrom<&'a Value> for &'a Pipeline {

    type Error = Error;

    fn try_from(value: &'a Value) -> std::result::Result<Self, Self::Error> {
        match value.as_pipeline() {
            Some(p) => Ok(p),
            None => Err(Error::new(format!("object is not pipeline: {:?}", value)))
        }
    }
}

impl TryFrom<Value> for Pipeline {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        match value.as_pipeline() {
            Some(p) => Ok(p.clone()),
            None => Err(Error::new(format!("object is not pipeline: {:?}", value)))
        }
    }
}

impl TryFrom<&Value> for Pipeline {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        match value.as_pipeline() {
            Some(p) => Ok(p.clone()),
            None => Err(Error::new(format!("object is not pipeline: {:?}", value)))
        }
    }
}