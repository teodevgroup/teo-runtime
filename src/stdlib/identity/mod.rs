use chrono::Utc;
use indexmap::{IndexMap, indexmap};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use key_path::path;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use crate::namespace::Namespace;
use crate::pipeline::Pipeline;
use crate::{pipeline, request};
use teo_result::Error;
use crate::value::Value;
use crate::teon;
use crate::{model, model::{Field}};
use crate::action::action::{CODE_AMOUNT, CODE_NAME, CODE_POSITION};
use crate::arguments::Arguments;
use crate::middleware::middleware::middleware_wrap_fn;
use crate::middleware::next::Next;
use crate::request::Ctx;
use crate::response::Response;
use crate::traits::named::Named;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub id: JsonValue,
    pub model: Vec<String>,
    pub exp: usize
}

pub fn encode_token(claims: JwtClaims, secret: &str) -> String {
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()));
    token.unwrap()
}

pub fn decode_token(token: &str, secret: &str) -> Result<JwtClaims, jsonwebtoken::errors::Error> {
    return match decode::<JwtClaims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()) {
        Ok(token) => {
            Ok(token.claims)
        }
        Err(reason) => {
            Err(reason)
        }
    }
}

pub(super) fn load_identity_library(std_namespace: &mut Namespace) {

    let mut identity_namespace = std_namespace.namespace_mut_or_create("identity");

    identity_namespace.define_model_decorator("tokenIssuer", |arguments, model| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        model.data.insert("identity:tokenIssuer".to_owned(), pipeline.into());
        Ok(())
    });

    identity_namespace.define_model_decorator("validateAccount", |arguments, model| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        model.data.insert("identity:validateAccount".to_owned(), pipeline.into());
        Ok(())
    });

    identity_namespace.define_model_decorator("jwtSecret", |arguments, model| {
        let secret: String = arguments.get("secret")?;
        model.data.insert("identity:jwtSecret".to_owned(), secret.into());
        Ok(())
    });

    identity_namespace.define_model_field_decorator("id", |arguments, field| {
        field.data.insert("identity:id".to_owned(), true.into());
        Ok(())
    });

    identity_namespace.define_model_field_decorator("checker", |arguments, field| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        field.data.insert("identity:checker".to_owned(), pipeline.into());
        Ok(())
    });

    identity_namespace.define_model_field_decorator("companion", |arguments, field| {
        field.data.insert("identity:companion".to_owned(), true.into());
        Ok(())
    });

    identity_namespace.define_pipeline_item("jwt", |arguments: Arguments, pipeline_ctx: pipeline::Ctx| async move {
        let object = pipeline_ctx.object();
        let Some(jwt_secret) = object.model().data.get("identity:jwtSecret") else {
            return Err(Error::internal_server_error_message("missing @identity.jwtSecret"));
        };
        let jwt_secret: String = jwt_secret.try_into()?;
        let expired: Option<Value> = arguments.get_optional("expired")?;

        let json_identifier: JsonValue = object.identifier().try_into()?;
        let claims = JwtClaims {
            id: json_identifier,
            model: object.model().path.clone(),
            exp: if let Some(expired) = expired {
                let expired_at = if let Some(pipeline) = expired.as_pipeline() {
                    let result: Value = pipeline_ctx.run_pipeline(pipeline).await?;
                    result.as_int64().unwrap()
                } else {
                    expired.as_int64().unwrap()
                };
                (Utc::now().timestamp() + expired_at) as usize
            } else { usize::MAX },
        };
        Ok(encode_token(claims, &jwt_secret).into())
    });

    identity_namespace.define_handler_template("signIn", |req_ctx: request::Ctx| async move {
        let model = req_ctx.namespace().model_at_path(&req_ctx.handler_match().path()).unwrap();
        let model_ctx = req_ctx.transaction_ctx().model_ctx_for_model_at_path(&req_ctx.handler_match().path()).unwrap();
        let input = req_ctx.body();
        let credentials = input.get("credentials").unwrap().as_dictionary().unwrap();
        let mut identity_key: Option<&String> = None;
        let mut identity_value: Option<&Value> = None;
        let mut checker_key: Option<&String> = None;
        let mut checker_value: Option<&Value> = None;
        let mut checker_field: Option<&Field> = None;
        let mut companion_values: IndexMap<String, Value> = indexmap!{};
        let mut id_values: IndexMap<String, Value> = indexmap!{};
        let id_fields: Vec<&Field> = model.fields.values().filter(|f| f.data.get("identity:id").is_some()).collect();
        if id_fields.len() == 0 {
            return Err(Error::internal_server_error_pathed(path!["credentials"], "no @identity.id defined on this model"));
        }
        let checker_fields: Vec<&Field> = model.fields.values().filter(|f| f.data.get("identity:checker").is_some()).collect();
        let companion_fields: Vec<&Field> = model.fields.values().filter(|f| f.data.get("identity:companion").is_some()).collect();
        for (k, v) in credentials {
            if let Some(f) = id_fields.iter().find(|f| f.name() == k.as_str()) {
                id_values.insert(k.to_string(), v.clone());
                if identity_key.is_none() {
                    identity_key = Some(k);
                    identity_value = Some(v);
                } else {
                    return Err(Error::invalid_request_pathed(path!["credentials", k.as_str()], "multiple @identity.id value received"));
                }
            }
            if let Some(f) = checker_fields.iter().find(|f| f.name() == k.as_str()) {
                if checker_key.is_none() {
                    checker_key = Some(k);
                    checker_value = Some(v);
                    checker_field = Some(*f);
                } else {
                    return Err(Error::invalid_request_pathed(path!["credentials", k.as_str()], "multiple @identity.checker value received"));
                }
            }
            if let Some(_) = companion_fields.iter().find(|f| f.name() == k.as_str()) {
                companion_values.insert(k.to_string(), v.clone());
            }
        }
        if identity_key == None {
            return Err(Error::invalid_request_pathed(path!["credentials"], "missing @identity.id value"));
        } else if checker_key == None {
            return Err(Error::invalid_request_pathed(path!["credentials"], "missing @identity.checker value"));
        }
        let object: Option<model::Object> = model_ctx.find_unique(&teon!({
            "where": {
                identity_key.unwrap(): identity_value.unwrap()
            }
        })).await?;
        let Some(object) = object else {
            return Err(Error::not_found_pathed(path!["credentials"], "this identity is not found"));
        };
        let auth_checker_pipeline = checker_field.unwrap().data.get("identity:checker").unwrap().as_pipeline().unwrap();
        let pipeline_input = teon!({
            "value": checker_value.unwrap(),
            "companions": companion_values,
            "ids": id_values,
        });
        let pipeline_ctx = pipeline::Ctx::new(Value::from(pipeline_input), object.clone(), path!["credentials"], CODE_NAME | CODE_AMOUNT | CODE_POSITION, req_ctx.transaction_ctx(), Some(req_ctx.clone()));
        let _ = pipeline_ctx.run_pipeline_ignore_return_value(auth_checker_pipeline).await?;
        let credentials_pipeline_ctx = pipeline::Ctx::new(Value::from(Value::Dictionary(credentials.clone())), object.clone(), path!["credentials"], CODE_NAME | CODE_AMOUNT | CODE_POSITION, req_ctx.transaction_ctx(), Some(req_ctx.clone()));
        let self_pipeline_ctx = pipeline::Ctx::new(Value::from(&object), object.clone(), path![], CODE_NAME | CODE_AMOUNT | CODE_POSITION, req_ctx.transaction_ctx(), Some(req_ctx.clone()));
        if let Some(validator) = model.data.get("identity:validateAccount") {
            let validator = validator.as_pipeline().unwrap();
            match self_pipeline_ctx.run_pipeline_ignore_return_value(validator).await {
                Ok(_) => (),
                Err(mut error) => {
                    error.code = 401;
                    return Err(error);
                }
            }
        }
        let Some(token_issuer) = model.data.get("identity:tokenIssuer") else {
            return Err(Error::internal_server_error_message("missing identity token issuer"));
        };
        let token_issuer = token_issuer.as_pipeline().unwrap();
        let token_string: String = credentials_pipeline_ctx.run_pipeline(token_issuer).await?;
        // Output to the client
        let include = input.get("include");
        let select = input.get("select");
        let obj = object.refreshed(include, select).await?;
        let obj_teon = obj.to_teon().await?;
        Ok(Response::data_meta(obj_teon, teon!({
            "token": token_string
        })))
    });

    identity_namespace.define_handler_template("identity", |req_ctx: request::Ctx| async move {
        let model = req_ctx.namespace().model_at_path(&req_ctx.handler_match().path()).unwrap();
        let model_ctx = req_ctx.transaction_ctx().model_ctx_for_model_at_path(&req_ctx.handler_match().path()).unwrap();
        let Some(jwt_secret) = model.data.get("identity:jwtSecret") else {
            return Err(Error::internal_server_error_message("missing @identity.jwtSecret"));
        };
        let jwt_secret: String = jwt_secret.try_into()?;
        let Some(authorization) = req_ctx.request().headers().get("authorization") else {
            return Err(Error::unauthorized_message("missing authorization header value"));
        };
        if authorization.len() < 7 {
            return Err(Error::unauthorized_message("invalid jwt token"));
        }
        let token = &authorization[7..];
        let Ok(claims) = decode_token(token, jwt_secret.as_str()) else {
            return Err(Error::unauthorized_message("invalid jwt token"));
        };
        if claims.model != model.path {
            return Err(Error::unauthorized_message("wrong model of identity"));
        }
        let teon_value: Value = Value::from(claims.id);
        let object: Option<model::Object> = model_ctx.find_unique(&teon_value).await?;
        if let Some(object) = object {
            return Ok(Response::data(object.to_teon().await?));
        } else {
            return Err(Error::unauthorized_message("identity not found"));
        }

        Ok::<Response, Error>(Response::html("")?)
    });

    identity_namespace.define_middleware("identityFromJwt", |arguments: Arguments| async move {
        let secret_string: String = arguments.get("secret")?;
        let secret = Box::leak(Box::new(secret_string)).as_str();
        Ok(middleware_wrap_fn(move |ctx: Ctx, next: &'static dyn Next| async move {
            if let Some(authorization) = ctx.request().headers().get("authorization") {
                if authorization.len() < 7 {
                    return Err(Error::unauthorized_message("invalid jwt token"));
                }
                let token = &authorization[7..];
                match decode_token(token, &secret) {
                    Ok(claims) => {
                        let json_identifier = &claims.id;
                        let Some(model_ctx) = ctx.transaction_ctx().model_ctx_for_model_at_path(&claims.model.iter().map(|s| s.as_str()).collect()) else {
                            return Err(Error::unauthorized_message("invalid jwt token"));
                        };
                        let teon_identifier = Value::from(json_identifier);
                        let object: Option<model::Object> = model_ctx.find_unique(&teon_identifier).await?;
                        if let Some(object) = object {
                            if let Some(validator) = object.model().data.get("identity:validateAccount") {
                                let validator = validator.as_pipeline().unwrap();
                                let self_pipeline_ctx = pipeline::Ctx::new(Value::from(&object), object.clone(), path![], CODE_NAME | CODE_AMOUNT | CODE_POSITION, ctx.transaction_ctx(), Some(ctx.clone()));
                                match self_pipeline_ctx.run_pipeline_ignore_return_value(validator).await {
                                    Ok(_) => (),
                                    Err(mut error) => {
                                        error.code = 401;
                                        return Err(error);
                                    }
                                }
                            }
                            ctx.data_mut().insert("account", Value::from(object));
                        } else {
                            return Err(Error::unauthorized_message("invalid jwt token"));
                        }
                    }
                    Err(error) => {
                        return match error.kind() {
                            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                                Err(Error::unauthorized_message("token expired"))
                            }
                            _ => {
                                Err(Error::unauthorized_message("invalid jwt token"))
                            }
                        };
                    }
                }
            }
            let res = next.call(ctx).await?;
            return Ok(res);
        }))
    });
}

