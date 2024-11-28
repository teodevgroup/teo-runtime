use crate::arguments::Arguments;
use crate::middleware::MiddlewareImpl;
use crate::middleware::next::Next;
use crate::namespace;
use crate::request::Request;
use crate::response::Response;

pub(in crate::stdlib) fn load_cors_middleware(namespace: &namespace::Builder) {
    namespace.define_request_middleware("cors", |arguments: Arguments| async move {
        let origin_: String = arguments.get("origin")?;
        let methods_: Vec<String> = arguments.get("methods")?;
        let headers_: Vec<String> = arguments.get("headers")?;
        let max_age: i32 = arguments.get("maxAge")?;
        let origin = Box::leak(Box::new(origin_)).as_str();
        let methods = &*Box::leak(Box::new(methods_));
        let headers = &*Box::leak(Box::new(headers_));
        Ok(move |request: Request, next: &'static dyn Next| async move {
            let res_or_err = next.call(request).await;
            let res = if res_or_err.is_ok() {
                res_or_err.unwrap()
            } else {
                Response::from(res_or_err.err().unwrap())
            };
            res.headers().set("Access-Control-Allow-Origin", origin);
            res.headers().set("Access-Control-Allow-Methods", methods.join(", "));
            res.headers().set("Access-Control-Allow-Headers", headers.join(", "));
            res.headers().set("Access-Control-Max-Age", max_age.to_string());
            return Ok(res);
        })
    });
}