pub mod serde;
pub mod convert;
pub mod traits;
pub mod error_ext;

use std::fmt::{Display, Formatter};
use std::sync::Arc;
use chrono::{Utc, DateTime};
use teo_teon::Value;
use teo_result::Error;
use crate::model;
use crate::pipeline::pipeline::Pipeline;
use crate::r#struct;
use teo_result::Result;
use indexmap::IndexMap;
use regex::Regex;
use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::types::range::Range;
use crate::interface_enum_variant::InterfaceEnumVariant;

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
    InterfaceEnumVariant(InterfaceEnumVariant),
}

impl AsRef<Object> for Object {

    fn as_ref(&self) -> &Object {
        self
    }
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

    pub fn is_interface_enum_variant(&self) -> bool {
        self.as_interface_enum_variant().is_some()
    }

    pub fn as_interface_enum_variant(&self) -> Option<&InterfaceEnumVariant> {
        match self.inner.as_ref() {
            ObjectInner::InterfaceEnumVariant(n) => Some(n),
            _ => None
        }
    }

    pub fn is_null(&self) -> bool {
        self.is_teon() && self.as_teon().unwrap().is_null()
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

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.inner.as_ref() {
            ObjectInner::Teon(teon) => Display::fmt(teon, f),
            ObjectInner::ModelObject(m) => Display::fmt(m, f),
            ObjectInner::StructObject(s) => Display::fmt(s, f),
            ObjectInner::Pipeline(p) => Display::fmt(p, f),
            ObjectInner::InterfaceEnumVariant(i) => Display::fmt(i, f),
        }
    }
}