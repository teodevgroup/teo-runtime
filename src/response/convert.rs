use bytes::Bytes;
use http_body_util::Full;
use teo_result::Result;
use crate::request::Request;
use crate::response::body::BodyInner;
use crate::response::Response;

pub fn hyper_response_from(request: Request, response: Response) -> Result<hyper::Response<Full<Bytes>>> {
    let mut builder = hyper::Response::builder().status(response.code());
    for key in response.headers().keys() {
        builder = builder.header(key.clone(), response.headers().get(&key).unwrap().clone());
    }
    for cookie in response.cookies() {
        builder = builder.header("Set-Cookie", cookie.encoded().to_string());
    }
    match response.body().inner.as_ref() {
        BodyInner::Empty => {
            let body_bytes = "".to_owned();
            Ok(builder.body(body_bytes.into()).unwrap())
        },
        BodyInner::String(content) => {
            let body_bytes = content.to_string();
            Ok(builder.body(body_bytes.into()).unwrap())
        },
        BodyInner::Teon(value) => {
            builder = builder.header("Content-Type", "application/json");
            let json_value = serde_json::Value::try_from(value).unwrap();
            let string_value = serde_json::to_string(&json_value).unwrap();
            Ok(builder.body(string_value.into()).unwrap())
        },
        BodyInner::File(path_buf) => {
            unreachable!()
        }
    }
}