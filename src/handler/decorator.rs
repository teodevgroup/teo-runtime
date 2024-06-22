use std::sync::Arc;
use educe::Educe;
use serde::Serialize;
use crate::arguments::Arguments;
use teo_result::Result;

use super::Handler;

pub trait Call {
    fn call(&self, args: Arguments, handler: &mut Handler) -> Result<()>;
}

impl<F> Call for F where
    F: Fn(Arguments, &mut Handler) -> Result<()> {
    fn call(&self, args: Arguments, handler: &mut Handler) -> Result<()> {
        self(args, handler)
    }
}

#[derive(Educe, Serialize, Clone)]
#[educe(Debug)]
pub struct Decorator {
    inner: Arc<DecoratorInner>
}

#[derive(Educe, Serialize)]
#[educe(Debug)]
struct DecoratorInner {
    pub path: Vec<String>,
    #[educe(Debug(ignore))] #[serde(skip)]
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