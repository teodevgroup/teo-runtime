use teo_result::Error;
use crate::object::Object;
use crate::pipeline::pipeline::Pipeline;

impl<'a> TryFrom<&'a Object> for &'a Pipeline {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        match value.as_pipeline() {
            Some(p) => Ok(p),
            None => Err(Error::new(format!("object is not pipeline: {:?}", value)))
        }
    }
}

impl TryFrom<Object> for Pipeline {

    type Error = Error;

    fn try_from(value: Object) -> std::result::Result<Self, Self::Error> {
        match value.as_pipeline() {
            Some(p) => Ok(p.clone()),
            None => Err(Error::new(format!("object is not pipeline: {:?}", value)))
        }
    }
}

impl TryFrom<&Object> for Pipeline {

    type Error = Error;

    fn try_from(value: &Object) -> std::result::Result<Self, Self::Error> {
        match value.as_pipeline() {
            Some(p) => Ok(p.clone()),
            None => Err(Error::new(format!("object is not pipeline: {:?}", value)))
        }
    }
}