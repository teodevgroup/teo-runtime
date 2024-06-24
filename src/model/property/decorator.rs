use std::sync::Arc;
use educe::Educe;
use serde::Serialize;
use crate::arguments::Arguments;
use crate::model::property::{Builder, Property};
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

#[derive(Educe, Serialize, Clone)]
#[educe(Debug)]
pub struct Decorator {
    inner: Arc<DecoratorInner>
}

struct DecoratorInner {
    path: Vec<String>,
    #[educe(Debug(ignore))] #[serde(skip)]
    call: Arc<dyn Call>,
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

    pub fn call(&self) -> &dyn crate::model::decorator::Call {
        self.inner.call.as_ref()
    }
}