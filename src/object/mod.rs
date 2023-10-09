use std::sync::Arc;
use teo_teon::Value;
use crate::error::Error;
use crate::model;
use crate::r#struct;
use crate::result::Result;

#[derive(Debug, Clone)]
pub struct Object {
    inner: Arc<ObjectInner>,
}

#[derive(Debug)]
pub enum ObjectInner {
    Teon(Value),
    ModelObject(model::Object),
    StructObject(r#struct::Object),
}

impl Object {

    pub fn is_teon(&self) -> bool {
        self.as_teon().is_some()
    }

    pub fn as_teon(&self) -> Option<&Value> {
        match self.inner.as_ref() {
            ObjectInner::Teon(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_teon_or_err(&self, msg: impl AsRef<str>) -> Result<&Value> {
        match self.inner.as_ref() {
            ObjectInner::Teon(v) => Ok(v),
            _ => Err(Error::new(msg.as_ref())),
        }
    }

    pub fn is_model_object(&self) -> bool {
        self.as_model_object().is_some()
    }

    pub fn as_model_object(&self) -> Option<&model::Object> {
        match self.inner.as_ref() {
            ObjectInner::ModelObject(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_struct_object(&self) -> bool {
        self.as_struct_object().is_some()
    }

    pub fn as_struct_object(&self) -> Option<&r#struct::Object> {
        match self.inner.as_ref() {
            ObjectInner::StructObject(v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> TryFrom<&'a Object> for &'a model::Object {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        match value.as_model_object() {
            Some(o) => Ok(o),
            None => Err(Error::new(format!("object is not model object: {:?}", value)))
        }
    }
}

impl TryFrom<&Object> for model::Object {

    type Error = Error;

    fn try_from(value: &Object) -> std::result::Result<Self, Self::Error> {
        match value.as_model_object() {
            Some(o) => Ok(o.clone()),
            None => Err(Error::new(format!("object is not model object: {:?}", value)))
        }
    }
}

impl<'a> TryFrom<&'a Object> for &'a r#struct::Object {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        match value.as_struct_object() {
            Some(o) => Ok(o),
            None => Err(Error::new(format!("object is not struct object: {:?}", value)))
        }
    }
}

impl TryFrom<&Object> for r#struct::Object {

    type Error = Error;

    fn try_from(value: &Object) -> std::result::Result<Self, Self::Error> {
        match value.as_struct_object() {
            Some(o) => Ok(o.clone()),
            None => Err(Error::new(format!("object is not struct object: {:?}", value)))
        }
    }
}

impl<'a> TryFrom<&'a Object> for &'a Value {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        match value.as_teon() {
            Some(o) => Ok(o),
            None => Err(Error::new(format!("object is not teon: {:?}", value)))
        }
    }
}

impl<'a> TryFrom<&'a Object> for &'a str {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        let teon: &'a Value = value.try_into()?;
        match teon.try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(format!("object is not &str: {:?}", value)))
        }
    }
}


