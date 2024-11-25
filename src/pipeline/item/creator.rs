use crate::arguments::Arguments;
use super::templates::call::Call;

pub trait Creator: Send + Sync {
    fn call(&self, arguments: Arguments) -> teo_result::Result<impl Call>;
}

impl<F> Creator for F where
    F: Fn(Arguments) -> impl Call {
    fn call(&self, args: Arguments) -> teo_result::Result<impl Call> {
        self(args)
    }
}