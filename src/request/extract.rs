use crate::request::Request;
use crate::value::Value;

pub trait ExtractFromRequest<'a> {
    fn extract(request: &'a Request) -> Self;
}

impl<'a> ExtractFromRequest<'a> for &'a Value {
    fn extract(request: &'a Request) -> Self {
        request.body_value().unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for Value {
    fn extract(request: &'a Request) -> Self {
        request.body_value().map(|v| v.clone()).unwrap_or(Value::Null)
    }
}
