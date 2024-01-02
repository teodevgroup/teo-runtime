use std::sync::Arc;
use educe::Educe;
use serde::Serialize;
use crate::arguments::Arguments;
use crate::model::property::Property;
use teo_result::Result;

#[derive(Educe, Serialize)]
#[educe(Debug)]
pub struct Decorator {
    pub path: Vec<String>,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub(crate) call: Arc<dyn Call>,
}

pub trait Call {
    fn call(&self, args: Arguments, field: &mut Property) -> Result<()>;
}

impl<F> Call for F where
        F: Fn(Arguments, &mut Property) -> Result<()> {
    fn call(&self, args: Arguments, field: &mut Property) -> Result<()> {
        self(args, field)
    }
}