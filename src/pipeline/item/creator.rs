use crate::arguments::Arguments;
use teo_result::Result;
use super::templates::call::Call;

pub trait Creator {
    fn call(&self, arguments: Arguments) -> Result<ItemImpl>;
}

impl<F> Creator for F where
    F: Fn(Arguments) -> Result<ItemImpl> {
    fn call(&self, args: Arguments) -> Result<ItemImpl> {
        self(args)
    }
}