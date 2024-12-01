use hyper::header::CONTENT_TYPE;
use mime::APPLICATION_JSON;
use teo_result::Error;
use crate::response::Response;
use crate::{teon, Value};

impl From<Error> for Response {
    fn from(value: Error) -> Self {
        let code = value.code;
        let t = value.inferred_title();
        let msg = value.message();
        let errors = value.errors.as_ref().map(|e| Value::Dictionary(e.iter().map(|(k ,v)| (k.clone(), Value::String(v.clone()))).collect()));
        let mut teon_value = teon!({
            "type": t.as_ref(),
            "message": msg,
        });
        if let Some(errors) = errors {
            teon_value["errors"] = errors;
        }
        let response = Response::teon(teon_value);
        response.headers().insert(CONTENT_TYPE.as_str(), APPLICATION_JSON.essence_str()).unwrap();
        response.set_code(code);
        response
    }
}