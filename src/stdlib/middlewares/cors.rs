use crate::arguments::Arguments;
use crate::middleware::next::Next;
use crate::middleware::next_imp::NextImp;
use crate::namespace;
use crate::request::Request;
use crate::response::Response;

pub(in crate::stdlib) fn load_cors_middleware(namespace: &namespace::Builder) {
    namespace.define_request_middleware("cors", |arguments: Arguments| async move {
        let origin: String = arguments.get("origin")?;
        let methods: Vec<String> = arguments.get("methods")?;
        let headers: Vec<String> = arguments.get("headers")?;
        let max_age: i32 = arguments.get("maxAge")?;
        Ok(move |request: Request, next: Next| {
            let origin = origin.clone();
            let methods = methods.clone();
            let headers = headers.clone();
            async move {
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
            }
        })
    });
}