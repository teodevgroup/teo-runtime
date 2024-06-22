use std::sync::Arc;
use educe::Educe;
use serde::Serialize;
use crate::arguments::Arguments;
use crate::interface::Interface;
use teo_result::Result;

pub trait Call {
    fn call(&self, args: Arguments, interface: &mut Interface) -> Result<()>;
}

impl<F> Call for F where
    F: Fn(Arguments, &mut Interface) -> Result<()> {
    fn call(&self, args: Arguments, interface: &mut Interface) -> Result<()> {
        self(args, interface)
    }
}

#[derive(Educe)]
#[educe(Debug)]
#[derive(Serialize, Clone)]
pub struct Decorator {
    pub path: Vec<String>,
    #[serde(skip)] #[educe(Debug(ignore))]
    pub(crate) call: Arc<dyn Call>,
}

impl Decorator {

    pub fn new<T>(path: Vec<String>, call: T) -> Self where T: Call + 'static {
        Self {
            path,
            call: Arc::new(call),
        }
    }

    pub fn path(&self) -> &Vec<String> {
        &self.path
    }

    pub fn call(&self) -> &dyn Call {
        self.call.as_ref()
    }
}