use std::time::SystemTime;
use teo_result::Result;
use crate::arguments::Arguments;
use crate::message::{request_message, unhandled_request_message};
use crate::middleware::next::{Next, NextImp};
use crate::namespace;
use crate::request::Request;
use crate::response::Response;

fn get_code(res_or_err: &Result<Response>) -> u16 {
    match res_or_err {
        Ok(res) => res.code(),
        Err(err) => err.code,
    }
}

pub(in crate::stdlib) fn load_log_request_middleware(namespace: &namespace::Builder) {
    namespace.define_request_middleware("logRequest", |arguments: Arguments| {
        Ok(|request: Request, next: Next| async move {
            let start = SystemTime::now();
            let res_or_err = next.call(request.clone()).await;
            let handler_found_info = request.handler_match();
            let time_elapsed = SystemTime::now().duration_since(start).unwrap();
            let path = request.path();
            let method = request.method();
            if let Ok(handler_found_info) = handler_found_info {
                request_message(time_elapsed, method.as_str(), path, handler_found_info.path(), handler_found_info.name(), get_code(&res_or_err));
            } else {
                unhandled_request_message(time_elapsed, method.as_str(), path, get_code(&res_or_err));
            }
            return res_or_err;
        })
    });
}