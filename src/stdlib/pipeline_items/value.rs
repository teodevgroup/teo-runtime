use crate::namespace::Namespace;
use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use crate::result::ResultExt;
use crate::object::Object;
use crate::error::Error;
use pad::{PadStr, Alignment};
use inflector::Inflector;
use regex::Regex;
use teo_teon::Value;

pub(in crate::stdlib) fn load_pipeline_value_items(namespace: &mut Namespace) {

    namespace.define_pipeline_item("eq", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("eq")?;
        let rhs_object = &ctx.resolve_pipeline(
            args.get_object("rhs").err_prefix("eq(rhs)")?,
            "eq(rhs)",
        ).await?;
        let rhs: &Value = rhs_object.try_into_err_prefix("eq(rhs)")?;
        if input == rhs {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("input is not equal to rhs"))?
        }
    });

    namespace.define_pipeline_item("gt", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("gt")?;
        let arg_object = &ctx.resolve_pipeline(
            args.get_object("rhs").err_prefix("gt(rhs)")?,
            "gt(rhs)",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("gt(rhs)")?;
        if input > arg {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("input is not greater than rhs"))?
        }
    });

    namespace.define_pipeline_item("gte", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("gte")?;
        let arg_object = &ctx.resolve_pipeline(
            args.get_object("rhs").err_prefix("gte(rhs)")?,
            "gte(rhs)",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("gte(rhs)")?;
        if input >= arg {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("input is not greater than or equal to rhs"))?
        }
    });


    namespace.define_pipeline_item("lt", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("lt")?;
        let arg_object = &ctx.resolve_pipeline(
            args.get_object("value").err_prefix("lt(rhs)")?,
            "lt(rhs)",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("lt(rhs)")?;
        if input < arg {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("input is not less than rhs"))?
        }
    });

    namespace.define_pipeline_item("lte", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("lte")?;
        let arg_object = &ctx.resolve_pipeline(
            args.get_object("rhs").err_prefix("lte(rhs)")?,
            "lte(rhs)",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("lte(rhs)")?;
        if input <= arg {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("input is not less than or equal to rhs"))?
        }
    });

    namespace.define_pipeline_item("neq", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("neq")?;
        let arg_object = &ctx.resolve_pipeline(
            args.get_object("rhs").err_prefix("neq(rhs)")?,
            "neq(rhs)",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("neq(rhs)")?;
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
            Err(Error::new("input is not present"))?
        }
        Ok(ctx.value().clone())
    });

    namespace.define_pipeline_item("isTrue", |args: Arguments, ctx: Ctx| async move {
        let input: bool = ctx.value().try_into_err_prefix("isTrue")?;
        if input {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("input is not true"))?
        }
    });

    namespace.define_pipeline_item("isFalse", |args: Arguments, ctx: Ctx| async move {
        let input: bool = ctx.value().try_into_err_prefix("isFalse")?;
        if !input {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("input is not false"))?
        }
    });

    namespace.define_pipeline_item("oneOf", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("oneOf")?;
        let candidates_object = &ctx.resolve_pipeline(
            args.get_object("candidates").err_prefix("oneOf(candidates)")?,
            "oneOf(candidates)",
        ).await?;
        let candidates: &Vec<Value> = candidates_object.try_into_err_prefix("oneOf(candidates)")?;
        if candidates.iter().find(|candidate| *candidate == input).is_some() {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("input is not one of candidates"))
        }
    });
}