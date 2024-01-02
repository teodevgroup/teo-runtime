use std::sync::Arc;
use educe::Educe;
use serde::Serialize;
use crate::arguments::Arguments;
use teo_result::Result;

use super::Enum;

#[derive(Educe, Serialize)]
#[educe(Debug)]
pub struct Decorator {
    pub path: Vec<String>,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub(crate) call: Arc<dyn Call>,
}

pub trait Call {
    fn call(&self, args: Arguments, field: &mut Enum) -> Result<()>;
}

impl<F> Call for F where
        F: Fn(Arguments, &mut Enum) -> Result<()> {
    fn call(&self, args: Arguments, field: &mut Enum) -> Result<()> {
        self(args, field)
    }
}