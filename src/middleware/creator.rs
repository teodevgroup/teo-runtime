use crate::arguments::Arguments;
use crate::middleware::middleware::Middleware;
use teo_result::Result;

pub trait Creator {
    fn call(&self, arguments: Arguments) -> Result<&'static dyn Middleware>;
}

impl<F> Creator for F where
    F: Fn(Arguments) -> Result<&'static dyn Middleware> {
    fn call(&self, arguments: Arguments) -> Result<&'static dyn Middleware> {
        self(arguments)
    }
}