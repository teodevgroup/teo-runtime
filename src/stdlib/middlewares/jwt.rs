use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use crate::arguments::Arguments;
use crate::middleware::middleware::{Middleware, middleware_wrap_fn};
use crate::middleware::next::Next;
use crate::namespace::Namespace;
use crate::request::ctx::Ctx;
use teo_result::Result;
use teo_result::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: JsonValue,
    pub model: Vec<String>,
    pub exp: usize
}

pub fn encode_token(claims: Claims, secret: &str) -> String {
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()));
    token.unwrap()
}

pub fn decode_token(token: &String, secret: &str) -> Result<Claims> {
    let token = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default());
    return match token {
        Ok(token) => {
            Ok(token.claims)
        }
        Err(_) => {
            Err(Error::new("invalid jwt token"))
        }
    }
}

pub(in crate::stdlib) fn load_jwt_middleware(namespace: &mut Namespace) {
    namespace.define_middleware("jwt", |arguments: Arguments| {
        let secret_string: String = arguments.get("secret")?;
        let secret = Box::leak(Box::new(secret_string)).as_str();
        Ok(middleware_wrap_fn(move |ctx: Ctx, next: &'static dyn Next| async move {
            if let Some(authorization) = ctx.request().headers().get("authorization") {
                if authorization.len() < 7 {
                    return Err(crate::path::Error::value_error_message_only("invalid jwt token"));
                }
                let token = &authorization[7..];
                if let Ok(claims) = decode_token(&token.to_string(), &secret) {
                    let json_identifier = &claims.id;
                    // fetch object and set to ctx
                    ctx.data_mut().insert("identity", 2);
                } else {
                    return Err(crate::path::Error::value_error_message_only("invalid jwt token"));
                }
            }
            let res = next.call(ctx).await?;
            return Ok(res);
        }))
    });
}
