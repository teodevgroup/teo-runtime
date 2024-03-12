async fn get_identity(r: &HttpRequest, graph: &'static Graph, conf: &ServerConf, connection: Arc<dyn Connection>, req: Req) -> Result<Option<Object>> {
    let header_value = r.headers().get("authorization");
    if let None = header_value {
        return Ok(None);
    }
    let auth_str = header_value.unwrap().to_str().unwrap();
    if auth_str.len() < 7 {
        return Err(Error::invalid_auth_token());
    }
    let token_str = &auth_str[7..];
    let claims_result = decode_token(&token_str.to_string(), &conf.jwt_secret.as_ref().unwrap());
    if let Err(_) = claims_result {
        return Err(Error::invalid_auth_token());
    }
    let claims = claims_result.unwrap();
    let json_identifier = &claims.id;
    let teon_identifier = Decoder::decode_object(AppCtx::get().unwrap().model(claims.model_path()).unwrap().unwrap(), graph, &json_identifier)?;
    let identity = graph.find_unique_internal(
        AppCtx::get().unwrap().model(claims.model_path()).unwrap().unwrap(),
        &teon!({
            "where": teon_identifier
        }),
        true, Action::from_u32(IDENTITY | FIND | SINGLE | ENTRY), Initiator::ProgramCode(Some(req)), connection).await;
    match identity {
        Err(_) => Err(Error::invalid_auth_token()),
        Ok(identity) => Ok(identity),
    }
}

async fn handle_sign_in<'a>(graph: &'static Graph, input: &'a Value, model: &'static Model, conf: &'a ServerConf, connection: Arc<dyn Connection>, req: Req) -> Result<Res> {
    if let Err(_err) = obj_result {
        return Err(Error::unexpected_input_value("This identity is not found.", path!["credentials", identity_key.unwrap()]));
    }
    let obj = obj_result.unwrap();

    let action_by_input = by_value.unwrap();
    let ctx = PipelineCtx::initial_state_with_object(obj.clone(), connection.clone(), Some(req)).with_value(action_by_input.clone());
    let result = pipeline.process(ctx).await;
    return match result {
        Err(_err) => {
            Err(Error::unexpected_input_value_with_reason("Authentication failed.", path!["credentials", by_key.unwrap()]))
        }
        Ok(_v) => {
            let include = input.get("include");
            let select = input.get("select");
            let obj = obj.refreshed(include, select).await.unwrap();
            let json_data = obj.to_teon_internal(&path!["data"]).await;
            let exp: usize = (Utc::now() + Duration::days(365)).timestamp() as usize;
            let teon_identifier = obj.identifier();
            let json_identifier: JsonValue = teon_identifier.into();
            let claims = Claims {
                id: json_identifier,
                model: obj.model().path().iter().map(|s| s.to_string()).collect(),
                exp
            };
            if conf.jwt_secret.as_ref().is_none() {
                return Err(Error::internal_server_error("Missing JWT secret."));
            }
            let token = encode_token(claims, &conf.jwt_secret.as_ref().unwrap());
            Ok(Res::teon_data_meta(json_data?, teon!({"token": token})))
        }
    }
}

async fn handle_identity<'a>(_graph: &'static Graph, input: &'a Value, model: &'static Model, _conf: &'a ServerConf, source: Initiator, _connection: Arc<dyn Connection>) -> Result<Res> {
    let identity = source.as_identity();
    return if let Some(identity) = identity {
        if identity.model() != model {
            return Err(Error::wrong_identity_model());
        }
        let select = input.get("select");
        let include = input.get("include");
        let refreshed = identity.refreshed(include, select).await.unwrap();
        let json_data = refreshed.to_teon_internal(&path!["data"]).await;
        Ok(Res::TeonDataRes(json_data?))
    } else {
        Ok(Res::TeonDataRes(teon!(null)))
    }
}
