use std::time::SystemTime;
use crate::arguments::Arguments;
use crate::message::{request_message, unhandled_request_message};
use crate::middleware::middleware::Middleware;
use crate::middleware::next::Next;
use crate::namespace;
use crate::request::Request;

pub(in crate::stdlib) fn load_log_request_middleware(namespace: &namespace::Builder) {
    namespace.define_request_middleware("logRequest", |arguments: Arguments| async move {
        Ok(Box::leak(Box::new(move |request: Request, next: &'static dyn Next| async move {
            let start = SystemTime::now();
            let res = next.call(request.clone()).await?;
            let handler_found_info = request.handler_match();
            let time_elapsed = SystemTime::now().duration_since(start).unwrap();
            let path = request.path();
            let method = request.method();
            if let Some(handler_found_info) = handler_found_info {
                request_message(time_elapsed, method, path, handler_found_info.path(), handler_found_info.name(), res.code());
            } else {
                unhandled_request_message(time_elapsed, method, path, res.code());
            }
            return Ok(res);
        })) as &dyn Middleware)
    });
}