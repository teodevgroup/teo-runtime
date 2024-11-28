use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use teo_result::ResultExt;
use teo_result::Error;
use crate::namespace;
use crate::value::Value;

pub(in crate::stdlib) fn load_pipeline_value_items(namespace: &namespace::Builder) {

    namespace.define_pipeline_item("is", |args: Arguments| {
        let argument = args.get_value("value").error_message_prefixed("is(value)")?;
        Ok(move |ctx: Ctx| {
            let argument = argument.clone();
            async move {
                let rhs: Value = ctx.resolve_pipeline_with_err_prefix(
                    argument.clone(),
                    "is(value)"
                ).await?;
                if ctx.value() == &rhs {
                    Ok(ctx.value().clone())
                } else if ctx.value().is_model_object() && rhs.is_model_object() {
                    let input = ctx.value().as_model_object().unwrap();
                    let rhs = rhs.as_model_object().unwrap();
                    if input.model().path() == rhs.model().path() {
                        if input.identifier() == rhs.identifier() {
                            Ok(ctx.value().clone())
                        } else {
                            Err(Error::new("input is not value"))?
                        }
                    } else {
                        Err(Error::new("input is not value"))?
                    }
                } else {
                    Err(Error::new("input is not value"))?
                }
            }
        })
    });

    namespace.define_pipeline_item("eq", |args: Arguments| {
        let rhs = args.get_value("rhs").error_message_prefixed("eq(rhs)")?;
        Ok(move |ctx: Ctx| {
            let rhs = rhs.clone();
            async move {
                let rhs_object: Value = ctx.resolve_pipeline_with_err_prefix(
                    rhs.clone(),
                    "eq(rhs)",
                ).await?;
                if ctx.value() == &rhs_object {
                    Ok(ctx.value().clone())
                } else {
                    Err(Error::new("input is not equal to rhs"))?
                }
            }
        })
    });

    namespace.define_pipeline_item("gt", |args: Arguments| {
        let rhs = args.get_value("rhs").error_message_prefixed("gt(rhs)")?;
        Ok(move |ctx: Ctx| {
            let rhs = rhs.clone();
            async move {
                let arg_object: Value = ctx.resolve_pipeline_with_err_prefix(
                    rhs.clone(),
                    "gt(rhs)",
                ).await?;
                if ctx.value() > &arg_object {
                    Ok(ctx.value().clone())
                } else {
                    Err(Error::new("input is not greater than rhs"))?
                }
            }
        })
    });

    namespace.define_pipeline_item("gte", |args: Arguments| {
        let rhs = args.get_value("rhs").error_message_prefixed("gte(rhs)")?;
        Ok(move |ctx: Ctx| {
            let rhs = rhs.clone();
            async move {
                let arg_object: Value = ctx.resolve_pipeline_with_err_prefix(
                    rhs.clone(),
                    "gte(rhs)",
                ).await?;
                if ctx.value() >= &arg_object {
                    Ok(ctx.value().clone())
                } else {
                    Err(Error::new("input is not greater than or equal to rhs"))?
                }
            }
        })
    });

    namespace.define_pipeline_item("lt", |args: Arguments| {
        let rhs = args.get_value("rhs").error_message_prefixed("lt(rhs)")?;
        Ok(move |ctx: Ctx| {
            let rhs = rhs.clone();
            async move {
                let arg_object: Value = ctx.resolve_pipeline_with_err_prefix(
                    rhs.clone(),
                    "lt(rhs)",
                ).await?;
                if ctx.value() < &arg_object {
                    Ok(ctx.value().clone())
                } else {
                    Err(Error::new("input is not less than rhs"))?
                }
            }
        })
    });

    namespace.define_pipeline_item("lte", |args: Arguments| {
        let rhs = args.get_value("rhs").error_message_prefixed("lte(rhs)")?;
        Ok(move |ctx: Ctx| {
            let rhs = rhs.clone();
            async move {
                let arg_object: Value = ctx.resolve_pipeline_with_err_prefix(
                    rhs.clone(),
                    "lte(rhs)",
                ).await?;
                if ctx.value() <= &arg_object {
                    Ok(ctx.value().clone())
                } else {
                    Err(Error::new("input is not less than or equal to rhs"))?
                }
            }
        })
    });

    namespace.define_pipeline_item("neq", |args: Arguments| {
        let rhs = args.get_value("rhs").error_message_prefixed("neq(rhs)")?;
        Ok(move |ctx: Ctx| {
            let rhs = rhs.clone();
            async move {
                let arg_object: Value = ctx.resolve_pipeline_with_err_prefix(
                    rhs.clone(),
                    "neq(rhs)",
                ).await?;
                if ctx.value() != &arg_object {
                    Ok(ctx.value().clone())
                } else {
                    Err(Error::new("input is equal to rhs"))?
                }
            }
        })
    });

    namespace.define_pipeline_item("isNull", |args: Arguments| {
        Ok(|ctx: Ctx| async move {
            if !ctx.value().is_null() {
                Err(Error::new_with_code("input is not null", 400))?
            }
            Ok(ctx.value().clone())
        })
    });

    namespace.define_pipeline_item("presents", |args: Arguments| {
        Ok(|ctx: Ctx| async move {
            if ctx.value().is_null() {
                Err(Error::new_with_code("input is not present", 400))?
            }
            Ok(ctx.value().clone())
        })
    });

    namespace.define_pipeline_item("isTrue", |args: Arguments| {
        Ok(|ctx: Ctx| async move {
            let input: bool = ctx.value().try_ref_into_err_prefix("isTrue")?;
            if input {
                Ok(ctx.value().clone())
            } else {
                Err(Error::new_with_code("input is not true", 400))?
            }
        })
    });

    namespace.define_pipeline_item("isFalse", |args: Arguments| {
        Ok(|ctx: Ctx| async move {
            let input: bool = ctx.value().try_ref_into_err_prefix("isFalse")?;
            if !input {
                Ok(ctx.value().clone())
            } else {
                Err(Error::new("input is not false"))?
            }
        })
    });

    namespace.define_pipeline_item("oneOf", |args: Arguments| {
        let candidates = args.get_value("candidates").error_message_prefixed("oneOf(candidates)")?;
        Ok(move |ctx: Ctx| {
            let candidates = candidates.clone();
            async move {
                let input: &Value = ctx.value().try_ref_into_err_prefix("oneOf")?;
                let candidates_object: Value = ctx.resolve_pipeline_with_err_prefix(
                    candidates.clone(),
                    "oneOf(candidates)",
                ).await?;
                let candidates: &Vec<Value> = candidates_object.try_ref_into_err_prefix("oneOf(candidates)")?;
                if candidates.iter().find(|candidate| *candidate == input).is_some() {
                    Ok(ctx.value().clone())
                } else {
                    Err(Error::new("oneOf: input is not one of candidates"))
                }
            }
        })
    });
}