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
#[derive(Serialize)]
pub struct Decorator {
    pub path: Vec<String>,
    #[serde(skip)] #[educe(Debug(ignore))]
    pub(crate) call: Arc<dyn Call>,
}
