use chrono::Utc;
use indexmap::{IndexMap, indexmap};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use key_path::path;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use crate::pipeline::Pipeline;
use crate::{namespace, pipeline};
use teo_result::Error;
use crate::value::Value;
use crate::teon;
use crate::{model, model::{Field}};
use crate::action::action::{CODE_AMOUNT, CODE_NAME, CODE_POSITION};
use crate::arguments::Arguments;
use crate::middleware::next::{Next, NextImp};
use crate::request::Request;
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
    match decode::<JwtClaims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()) {
        Ok(token) => {
            Ok(token.claims)
        }
        Err(reason) => {
            Err(reason)
        }
    }
}

pub(super) fn load_identity_library(std_namespace: &namespace::Builder) {

    let identity_namespace = std_namespace.child_namespace_or_create("identity");

    identity_namespace.define_model_decorator("tokenIssuer", |arguments, model| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        model.insert_data_entry("identity:tokenIssuer".to_owned(), pipeline.into());
        Ok(())
    });

    identity_namespace.define_model_decorator("validateAccount", |arguments, model| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        model.insert_data_entry("identity:validateAccount".to_owned(), pipeline.into());
        Ok(())
    });

    identity_namespace.define_model_decorator("jwtSecret", |arguments, model| {
        let secret: String = arguments.get("secret")?;
        model.insert_data_entry("identity:jwtSecret".to_owned(), secret.into());
        Ok(())
    });

    identity_namespace.define_model_field_decorator("id", |arguments, field| {
        field.insert_data_entry("identity:id".to_owned(), true.into());
        Ok(())
    });

    identity_namespace.define_model_field_decorator("checker", |arguments, field| {
        let pipeline: Pipeline = arguments.get("pipeline")?;
        field.insert_data_entry("identity:checker".to_owned(), pipeline.into());
        Ok(())
    });

    identity_namespace.define_model_field_decorator("companion", |arguments, field| {
        field.insert_data_entry("identity:companion".to_owned(), true.into());
        Ok(())
    });

    identity_namespace.define_pipeline_item("jwt", |arguments: Arguments| {
        let expired: Option<Value> = arguments.get_optional("expired")?;
        Ok(move |pipeline_ctx: pipeline::Ctx| {
            let expired = expired.clone();
            async move {
                let object = pipeline_ctx.object();
                let Some(jwt_secret) = object.model().data().get("identity:jwtSecret") else {
                    return Err(Error::internal_server_error_message("missing @identity.jwtSecret"));
                };
                let jwt_secret: String = jwt_secret.try_into()?;
                let json_identifier: JsonValue = object.identifier().try_into()?;
                let claims = JwtClaims {
                    id: json_identifier,
                    model: object.model().path().clone(),
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
            }
        })
    });

    identity_namespace.define_handler_template("signIn", |request: Request| async move {
        let model = request.transaction_ctx().namespace().model_at_path(&request.handler_match().unwrap().path()).unwrap().clone();
        let model_ctx = request.transaction_ctx().model_ctx_for_model_at_path(request.handler_match().unwrap().path()).unwrap();
        let input = request.body_value()?;
        let credentials = input.get("credentials").unwrap().as_dictionary().unwrap();
        let mut identity_key: Option<&String> = None;
        let mut identity_value: Option<&Value> = None;
        let mut checker_key: Option<&String> = None;
        let mut checker_value: Option<&Value> = None;
        let mut checker_field: Option<&Field> = None;
        let mut companion_values: IndexMap<String, Value> = indexmap!{};
        let mut id_values: IndexMap<String, Value> = indexmap!{};
        let id_fields: Vec<&Field> = model.fields().values().filter(|f| f.data().get("identity:id").is_some()).collect();
        if id_fields.len() == 0 {
            return Err(Error::internal_server_error_pathed(path!["credentials"], "no @identity.id defined on this model"));
        }
        let checker_fields: Vec<&Field> = model.fields().values().filter(|f| f.data().get("identity:checker").is_some()).collect();
        let companion_fields: Vec<&Field> = model.fields().values().filter(|f| f.data().get("identity:companion").is_some()).collect();
        for (k, v) in credentials {
            if id_fields.iter().find(|f| f.name() == k.as_str()).is_some() {
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
        let auth_checker_pipeline = checker_field.unwrap().data().get("identity:checker").unwrap().as_pipeline().unwrap();
        let pipeline_input = teon!({
            "value": checker_value.unwrap(),
            "companions": companion_values,
            "ids": id_values,
        });
        let pipeline_ctx = pipeline::Ctx::new(Value::from(pipeline_input), object.clone(), path!["credentials"], CODE_NAME | CODE_AMOUNT | CODE_POSITION, request.transaction_ctx(), Some(request.clone()));
        let _ = pipeline_ctx.run_pipeline_ignore_return_value(auth_checker_pipeline).await.map_err(|mut e| { e.code = 401; e})?;
        let credentials_pipeline_ctx = pipeline::Ctx::new(Value::from(Value::Dictionary(credentials.clone())), object.clone(), path!["credentials"], CODE_NAME | CODE_AMOUNT | CODE_POSITION, request.transaction_ctx(), Some(request.clone()));
        let self_pipeline_ctx = pipeline::Ctx::new(Value::from(&object), object.clone(), path![], CODE_NAME | CODE_AMOUNT | CODE_POSITION, request.transaction_ctx(), Some(request.clone()));
        if let Some(validator) = model.data().get("identity:validateAccount") {
            let validator = validator.as_pipeline().unwrap();
            match self_pipeline_ctx.run_pipeline_ignore_return_value(validator).await {
                Ok(_) => (),
                Err(mut error) => {
                    error.code = 401;
                    return Err(error);
                }
            }
        }
        let Some(token_issuer) = model.data().get("identity:tokenIssuer") else {
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

    identity_namespace.define_handler_template("identity", |request: Request| async move {
        let model = request.transaction_ctx().namespace().model_at_path(&request.handler_match().unwrap().path()).unwrap().clone();
        let model_ctx = request.transaction_ctx().model_ctx_for_model_at_path(request.handler_match().unwrap().path()).unwrap();
        let Some(jwt_secret) = model.data().get("identity:jwtSecret") else {
            return Err(Error::internal_server_error_message("missing @identity.jwtSecret"));
        };
        let jwt_secret: String = jwt_secret.try_into()?;
        let Some(authorization) = request.headers().get("authorization")? else {
            return Err(Error::unauthorized_message("missing authorization header value"));
        };
        if authorization.len() < 7 {
            return Err(Error::unauthorized_message("invalid jwt token"));
        }
        let token = &authorization[7..];
        let Ok(claims) = decode_token(token, jwt_secret.as_str()) else {
            return Err(Error::unauthorized_message("invalid jwt token"));
        };
        if &claims.model != model.path() {
            return Err(Error::unauthorized_message("wrong model of identity"));
        }
        let teon_value: Value = Value::from(claims.id);
        let object: Option<model::Object> = model_ctx.find_unique(&teon_value).await?;
        if let Some(object) = object {
            let include = request.body_value()?.get("include");
            let select = request.body_value()?.get("select");
            let obj = object.refreshed(include, select).await?;
            let obj_teon = obj.to_teon().await?;
            Ok(Response::data_meta(obj_teon, teon!({
                "token": token
            })))
        } else {
            Err(Error::unauthorized_message("identity not found"))
        }
    });

    identity_namespace.define_handler_middleware("identityFromJwt", |arguments: Arguments| {
        let secret: String = arguments.get("secret")?;
        Ok(move |request: Request, next: Next| {
            let secret = secret.clone();
            async move {
                if let Some(authorization) = request.headers().get("authorization")? {
                    if authorization.len() < 7 {
                        return Err(Error::unauthorized_message("invalid jwt token"));
                    }
                    let token = &authorization[7..];
                    match decode_token(token, &secret) {
                        Ok(claims) => {
                            let json_identifier = &claims.id;
                            let Some(model_ctx) = request.transaction_ctx().model_ctx_for_model_at_path(&claims.model) else {
                                return Err(Error::unauthorized_message("invalid jwt token"));
                            };
                            let teon_identifier = Value::from(json_identifier);
                            let object: Option<model::Object> = model_ctx.find_unique(&teon_identifier).await?;
                            if let Some(object) = object {
                                if let Some(validator) = object.model().data().get("identity:validateAccount") {
                                    let validator = validator.as_pipeline().unwrap();
                                    let self_pipeline_ctx = pipeline::Ctx::new(Value::from(&object), object.clone(), path![], CODE_NAME | CODE_AMOUNT | CODE_POSITION, request.transaction_ctx(), Some(request.clone()));
                                    match self_pipeline_ctx.run_pipeline_ignore_return_value(validator).await {
                                        Ok(_) => (),
                                        Err(mut error) => {
                                            error.code = 401;
                                            return Err(error);
                                        }
                                    }
                                }
                                request.local_values().insert("account", Value::from(object));
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
                let res = next.call(request).await?;
                Ok(res)
            }
        })
    });
}

