use std::sync::Arc;
use teo_teon::Value;
use crate::error::Error;
use crate::model;
use crate::pipeline::pipeline::Pipeline;
use crate::r#struct;
use crate::result::Result;
use bigdecimal::BigDecimal;

#[derive(Debug, Clone)]
pub struct Object {
    inner: Arc<ObjectInner>,
}

#[derive(Debug)]
pub enum ObjectInner {
    Teon(Value),
    ModelObject(model::Object),
    StructObject(r#struct::Object),
    Pipeline(Pipeline),
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

    pub fn is_pipeline(&self) -> bool {
        self.as_pipeline().is_some()
    }

    pub fn as_pipeline(&self) -> Option<&Pipeline> {
        match self.inner.as_ref() {
            ObjectInner::Pipeline(p) => Some(p),
            _ => None,
        }
    }

    pub fn try_into_err_prefix<'a, T: 'a, E>(&'a self, prefix: impl AsRef<str>) -> Result<T> where E: std::error::Error, T: TryFrom<&'a Object, Error = E> {
        let result: std::result::Result<T, E> = self.try_into();
        match result {
            Ok(t) => Ok(t),
            Err(e) => Err(Error::new(format!("{}: {e}", prefix.as_ref()))),
        }
    }

    pub fn try_into_err_message<'a, T: 'a, E>(&'a self, message: impl AsRef<str>) -> Result<T> where E: std::error::Error, T: TryFrom<&'a Object, Error = E> {
        let result: std::result::Result<T, E> = self.try_into();
        match result {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::new(message.as_ref())),
        }
    }
}

impl From<Value> for Object {

    fn from(value: Value) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(value)),
        }
    }
}

impl From<&Value> for Object {

    fn from(value: &Value) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(value.clone())),
        }
    }
}


impl From<i32> for Object {

    fn from(value: i32) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::Int(value)))
        }
    }
}

impl From<i64> for Object {

    fn from(value: i64) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::Int64(value)))
        }
    }
}

impl From<f32> for Object {

    fn from(value: f32) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::Float32(value)))
        }
    }
}

impl From<f64> for Object {

    fn from(value: f64) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::Float(value)))
        }
    }
}

impl From<BigDecimal> for Object {

    fn from(value: BigDecimal) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::Decimal(value)))
        }
    }
}

impl From<&BigDecimal> for Object {

    fn from(value: &BigDecimal) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::Decimal(value.clone())))
        }
    }
}

impl From<model::Object> for Object {

    fn from(value: model::Object) -> Self {
        Object {
            inner: Arc::new(ObjectInner::ModelObject(value)),
        }
    }
}

impl From<r#struct::Object> for Object {

    fn from(value: r#struct::Object) -> Self {
        Object {
            inner: Arc::new(ObjectInner::StructObject(value)),
        }
    }
}

impl From<Pipeline> for Object {

    fn from(value: Pipeline) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Pipeline(value)),
        }
    }
}

impl TryFrom<&Object> for Object {

    type Error = Error;

    fn try_from(value: &Object) -> std::result::Result<Self, Self::Error> {
        Ok(value.clone())
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

impl<'a> TryFrom<&'a Object> for i32 {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        let teon: &'a Value = value.try_into()?;
        match teon.try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(format!("object is not i32: {:?}", value)))
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

impl<'a> TryFrom<&'a Object> for &'a Pipeline {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        match value.as_pipeline() {
            Some(p) => Ok(p),
            None => Err(Error::new(format!("object is not pipeline: {:?}", value)))
        }
    }
}


