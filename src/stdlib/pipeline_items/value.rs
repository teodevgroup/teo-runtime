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
        let arg_object = &ctx.resolve_pipeline(
            args.get_object("value").err_prefix("eq(value)")?,
            "eq(value)",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("eq(value)")?;
        if input == arg {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("eq: values do not equal"))?
        }
    });

    namespace.define_pipeline_item("gt", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("gt")?;
        let arg_object = &ctx.resolve_pipeline(
            args.get_object("value").err_prefix("gt(value)")?,
            "gt(value)",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("gt(value)")?;
        if input > arg {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("gt: values not greater than rhs"))?
        }
    });

    namespace.define_pipeline_item("gte", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("gte")?;
        let arg_object = &ctx.resolve_pipeline(
            args.get_object("value").err_prefix("gte(value)")?,
            "gte(value)",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("gte(value)")?;
        if input >= arg {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("gte: values is not greater than or equal to rh"))?
        }
    });


    namespace.define_pipeline_item("lt", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("lt")?;
        let arg_object = &ctx.resolve_pipeline(
            args.get_object("value").err_prefix("lt(value)")?,
            "lt(value)",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("lt(value)")?;
        if input < arg {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("lt: values is less than rhs"))?
        }
    });

    namespace.define_pipeline_item("lte", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("lte")?;
        let arg_object = &ctx.resolve_pipeline(
            args.get_object("value").err_prefix("lte(value)")?,
            "lte(value)",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("lte(value)")?;
        if input <= arg {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("lte: values is not less than or equal to rhs"))?
        }
    });

    namespace.define_pipeline_item("neq", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("neq")?;
        let arg_object = &ctx.resolve_pipeline(
            args.get_object("value").err_prefix("neq(value)")?,
            "neq(value)",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("neq(value)")?;
        if input != arg {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("value is equal to rhs"))?
        }
    });
}