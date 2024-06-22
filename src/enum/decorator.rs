use std::sync::Arc;
use educe::Educe;
use serde::Serialize;
use crate::arguments::Arguments;
use teo_result::Result;

use super::Enum;

pub trait Call {
    fn call(&self, args: Arguments, field: &mut Enum) -> Result<()>;
}

impl<F> Call for F where
    F: Fn(Arguments, &mut Enum) -> Result<()> {
    fn call(&self, args: Arguments, field: &mut Enum) -> Result<()> {
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