use std::sync::Arc;
use serde::Serialize;
use crate::arguments::Arguments;
use crate::interface::field::Field;
use teo_result::Result;

pub trait Call {
    fn call(&self, args: Arguments, field: &mut Field) -> Result<()>;
}

impl<F> crate::interface::decorator::Call for F where
    F: Fn(Arguments, &mut Field) -> Result<()> {
    fn call(&self, args: Arguments, field: &mut Field) -> Result<()> {
        self(args, field)
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Decorator {
    inner: Arc<DecoratorInner>,
}

#[derive(Debug, Serialize)]
pub struct DecoratorInner {
    pub path: Vec<String>,
    #[serde(skip)]
    pub(crate) call: Arc<dyn Call>,

}

impl Decorator {

    pub fn new<T>(path: Vec<String>, call: T) -> Self where T: Call + 'static {
        Self {
            inner: Arc::new(DecoratorInner {
                path,
                call: Arc::new(call),
            })
        }
    }

        pub fn path(&self) -> &Vec<String> {
            &self.inner.path
        }

        pub fn call(&self) -> &dyn Call {
            &self.inner.call
        }
}