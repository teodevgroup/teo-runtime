use std::collections::VecDeque;
use std::pin::Pin;
use bytes::Bytes;
use http_body_util::{Either, Full};
use hyper::body::Body;
use hyper::header::CONTENT_TYPE;
use hyper_static::ErrorBoxed;
use hyper_static::serve::{static_file, ErrorKind};
use mime::APPLICATION_JSON;
use teo_result::{Error, Result};
use crate::request::Request;
use crate::response::body::BodyInner;
use crate::response::Response;

pub async fn hyper_response_from(request: Request, response: Response) -> Result<hyper::Response<Either<Full<Bytes>, Pin<Box<dyn Body<Data = VecDeque<u8>, Error = ErrorBoxed> + 'static + Send>>>>> {
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
            Ok(builder.body(Either::Left(body_bytes.into())).unwrap())
        },
        BodyInner::String(content) => {
            let body_bytes = content.to_string();
            Ok(builder.body(Either::Left(body_bytes.into())).unwrap())
        },
        BodyInner::Teon(value) => {
            builder = builder.header(CONTENT_TYPE, APPLICATION_JSON.essence_str());
            let json_value = serde_json::Value::try_from(value).unwrap();
            let string_value = serde_json::to_string(&json_value).unwrap();
            Ok(builder.body(Either::Left(string_value.into())).unwrap())
        },
        BodyInner::File(path_buf) => {
            let mime_type = if let Some(extension) = path_buf.extension() {
                mime_guess::from_ext(extension.to_str().unwrap()).first().unwrap_or(mime::APPLICATION_OCTET_STREAM)
            } else {
                mime::APPLICATION_OCTET_STREAM
            };
            let serve_static_result = static_file(path_buf, Some(mime_type.essence_str()), request.headers(), 65536).await;
            match serve_static_result {
                Err(err) => {
                    match err.kind() {
                        ErrorKind::Internal => return Err(Error::internal_server_error_message(format!("error sending file: {}", err.to_string()))),
                        ErrorKind::Forbidden => return Err(Error::invalid_request_message(format!("forbidden: {}", err.to_string()))),
                        ErrorKind::NotFound => return Err(Error::not_found()),
                        ErrorKind::BadRequest => return Err(Error::invalid_request_message(format!("bad request: {}", err.to_string()))),
                    }
                }
                Ok(response_result) => {
                    match response_result {
                        Err(err) => {
                            return Err(Error::invalid_request_message(format!("error occurred sending file: {}", err.to_string())));
                        }
                        Ok(response) => {
                            let (parts, body) = response.into_parts();
                            Ok(hyper::Response::from_parts(parts, Either::Right(body)))
                        }
                    }
                }
            }
        }
    }
}