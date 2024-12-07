use crate::arguments::Arguments;
use teo_result::Result;
use crate::middleware::Middleware;

pub trait Creator {
    fn call(&self, arguments: Arguments) -> Result<Middleware>;
}

impl<F> Creator for F where
    F: Fn(Arguments) -> Result<Middleware> + 'static {
    fn call(&self, args: Arguments) -> Result<Middleware> {
        self(args)
    }
}