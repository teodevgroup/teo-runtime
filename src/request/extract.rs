use crate::request::Request;
use crate::value::Value;

pub trait ExtractFromRequest {
    fn extract(request: &Request) -> Self;
}

impl ExtractFromRequest for Value {
    fn extract(request: &Request) -> Self {
        request.body_value().as_ref().clone()
    }
}
