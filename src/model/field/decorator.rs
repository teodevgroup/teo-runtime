use std::sync::Arc;
use educe::Educe;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::arguments::Arguments;
use crate::model::field::Builder;
use teo_result::Result;

pub trait Call {
    fn call(&self, args: Arguments, field: &Builder) -> Result<()>;
}

impl<F> Call for F where
    F: Fn(Arguments, &Builder) -> Result<()> {
    fn call(&self, args: Arguments, field: &Builder) -> Result<()> {
        self(args, field)
    }
}

#[derive(Educe, Clone)]
#[educe(Debug)]
pub struct Decorator {
    inner: Arc<DecoratorInner>
}

#[derive(Educe)]
#[educe(Debug)]
struct DecoratorInner {
    path: Vec<String>,
    #[educe(Debug(ignore))]
    pub(crate) call: Arc<dyn Call>,
}

impl Decorator {

    pub fn new<T>(path: Vec<String>, call: T) -> Self where T: Call + 'static {
        Self {
            inner: Arc::new(DecoratorInner {
                path,
                call: Arc::new(call),
            }),
        }
    }

    pub fn path(&self) -> &Vec<String> {
        &self.inner.path
    }

    pub fn call(&self) -> &dyn Call {
        self.inner.call.as_ref()
    }
}

impl Serialize for Decorator {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut r#struct = serializer.serialize_struct("Decorator", 1)?;
        r#struct.serialize_field("path", &self.inner.path)?;
        r#struct.end()
    }
}