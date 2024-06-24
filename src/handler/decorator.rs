use std::sync::Arc;
use educe::Educe;
use serde::Serialize;
use crate::arguments::Arguments;
use teo_result::Result;
use super::Builder;

pub trait Call {
    fn call(&self, args: Arguments, handler_builder: &Builder) -> Result<()>;
}

impl<F> Call for F where
    F: Fn(Arguments, &Builder) -> Result<()> {
    fn call(&self, args: Arguments, handler_builder: &Builder) -> Result<()> {
        self(args, handler_builder)
    }
}

#[derive(Educe, Serialize, Clone)]
#[educe(Debug)]
pub struct Decorator {
    inner: Arc<Inner>
}

#[derive(Educe, Serialize)]
#[educe(Debug)]
struct Inner {
    pub path: Vec<String>,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub(crate) call: Arc<dyn Call>,
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