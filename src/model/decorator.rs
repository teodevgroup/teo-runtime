use std::sync::Arc;
use educe::Educe;
use serde::Serialize;
use crate::arguments::Arguments;
use teo_result::Result;
use crate::model;

pub trait Call {
    fn call(&self, args: Arguments, model_builder: &model::Builder) -> Result<()>;
}

impl<F> Call for F where
    F: Fn(Arguments, &model::Builder) -> Result<()> {
    fn call(&self, args: Arguments, model_builder: &model::Builder) -> Result<()> {
        self(args, model_builder)
    }
}

#[derive(Educe, Serialize, Clone)]
#[educe(Debug)]
pub struct Decorator {
    inner: Arc<Inner>,
}

#[derive(Educe, Serialize)]
#[educe(Debug)]
struct Inner {
    path: Vec<String>,
    #[educe(Debug(ignore))] #[serde(skip)]
    call: Arc<dyn Call>,
}

impl Decorator {

    pub fn new<T>(path: Vec<String>, call: T) -> Self where T: Call + 'static {
        Self {
            inner: Arc::new(Inner {
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