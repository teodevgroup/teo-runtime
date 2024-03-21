use crate::namespace::Namespace;
use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use teo_result::ResultExt;
use teo_result::Error;
use pad::{PadStr, Alignment};
use inflector::Inflector;
use regex::Regex;
use crate::value::Value;

pub(in crate::stdlib) fn load_pipeline_value_items(namespace: &mut Namespace) {

    namespace.define_pipeline_item("is", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value();
        let rhs: Value = ctx.resolve_pipeline_with_err_prefix(
            args.get_object("value").error_message_prefixed("is(value)")?,
            "is(value)"
        ).await?;
        if input == &rhs {
            Ok(ctx.value().clone())
        } else if input.is_model_object() && rhs.is_model_object() {
            let input = input.as_model_object().unwrap();
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
    });

    namespace.define_pipeline_item("eq", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value();
        let rhs_object: Value = ctx.resolve_pipeline_with_err_prefix(
            args.get_object("rhs").error_message_prefixed("eq(rhs)")?,
            "eq(rhs)",
        ).await?;
        let rhs: &Value = rhs_object.try_ref_into_err_prefix("eq(rhs)")?;
        if input == rhs {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("input is not equal to rhs"))?
        }
    });

    namespace.define_pipeline_item("gt", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_ref_into_err_prefix("gt")?;
        let arg_object: Value = ctx.resolve_pipeline_with_err_prefix(
            args.get_object("rhs").error_message_prefixed("gt(rhs)")?,
            "gt(rhs)",
        ).await?;
        let arg: &Value = arg_object.try_ref_into_err_prefix("gt(rhs)")?;
        if input > arg {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("input is not greater than rhs"))?
        }
    });

    namespace.define_pipeline_item("gte", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_ref_into_err_prefix("gte")?;
        let arg_object: Value = ctx.resolve_pipeline_with_err_prefix(
            args.get_object("rhs").error_message_prefixed("gte(rhs)")?,
            "gte(rhs)",
        ).await?;
        let arg: &Value = arg_object.try_ref_into_err_prefix("gte(rhs)")?;
        if input >= arg {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("input is not greater than or equal to rhs"))?
        }
    });


    namespace.define_pipeline_item("lt", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_ref_into_err_prefix("lt")?;
        let arg_object: Value = ctx.resolve_pipeline_with_err_prefix(
            args.get_object("value").error_message_prefixed("lt(rhs)")?,
            "lt(rhs)",
        ).await?;
        let arg: &Value = arg_object.try_ref_into_err_prefix("lt(rhs)")?;
        if input < arg {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("input is not less than rhs"))?
        }
    });

    namespace.define_pipeline_item("lte", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_ref_into_err_prefix("lte")?;
        let arg_object: Value = ctx.resolve_pipeline_with_err_prefix(
            args.get_object("rhs").error_message_prefixed("lte(rhs)")?,
            "lte(rhs)",
        ).await?;
        let arg: &Value = arg_object.try_ref_into_err_prefix("lte(rhs)")?;
        if input <= arg {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("input is not less than or equal to rhs"))?
        }
    });

    namespace.define_pipeline_item("neq", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_ref_into_err_prefix("neq")?;
        let arg_object: Value = ctx.resolve_pipeline_with_err_prefix(
            args.get_object("rhs").error_message_prefixed("neq(rhs)")?,
            "neq(rhs)",
        ).await?;
        let arg: &Value = arg_object.try_ref_into_err_prefix("neq(rhs)")?;
        if input != arg {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("input is equal to rhs"))?
        }
    });

    namespace.define_pipeline_item("isNull", |args: Arguments, ctx: Ctx| async move {
        if !ctx.value().is_null() {
            Err(Error::new("input is not null"))?
        }
        Ok(ctx.value().clone())
    });

    namespace.define_pipeline_item("presents", |args: Arguments, ctx: Ctx| async move {
        if ctx.value().is_null() {
            Err(Error::new_with_code("input is not present", 400))?
        }
        Ok(ctx.value().clone())
    });

    namespace.define_pipeline_item("isTrue", |args: Arguments, ctx: Ctx| async move {
        let input: bool = ctx.value().try_ref_into_err_prefix("isTrue")?;
        if input {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new_with_code("input is not true", 400))?
        }
    });

    namespace.define_pipeline_item("isFalse", |args: Arguments, ctx: Ctx| async move {
        let input: bool = ctx.value().try_ref_into_err_prefix("isFalse")?;
        if !input {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("input is not false"))?
        }
    });

    namespace.define_pipeline_item("oneOf", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_ref_into_err_prefix("oneOf")?;
        let candidates_object: Value = ctx.resolve_pipeline_with_err_prefix(
            args.get_object("candidates").error_message_prefixed("oneOf(candidates)")?,
            "oneOf(candidates)",
        ).await?;
        let candidates: &Vec<Value> = candidates_object.try_ref_into_err_prefix("oneOf(candidates)")?;
        if candidates.iter().find(|candidate| *candidate == input).is_some() {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("input is not one of candidates"))
        }
    });
}