use teo_teon::Value;
use crate::arguments::Arguments;
use crate::namespace::Namespace;
use crate::object::Object;
use crate::pipeline::Ctx;
use crate::result::ResultExt;

pub(in crate::stdlib) fn load_pipeline_math_items(namespace: &mut Namespace) {
    namespace.define_pipeline_item("add", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("add")?;
        let arg_object = &ctx.resolve_pipeline(
            args.get_object("value").err_prefix("add(value)")?,
            "add(value)",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("add(value)")?;
        Ok(Object::from((input + arg).err_prefix("add")?))
    });

    namespace.define_pipeline_item("sub", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("sub")?;
        let arg_object = &ctx.resolve_pipeline(
            args.get_object("value").err_prefix("sub(value)")?,
            "sub(value)",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("sub(value)")?;
        Ok(Object::from((input - arg).err_prefix("sub")?))
    });

    namespace.define_pipeline_item("mul", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("mul")?;
        let arg_object = &ctx.resolve_pipeline(
            args.get_object("value").err_prefix("mul(value)")?,
            "mul(value)",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("mul(value)")?;
        Ok(Object::from((input * arg).err_prefix("mul")?))
    });

    namespace.define_pipeline_item("div", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("div")?;
        let arg_object = &ctx.resolve_pipeline(
            args.get_object("value").err_prefix("div(value)")?,
            "div(value)",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("div(value)")?;
        Ok(Object::from((input / arg).err_prefix("div")?))
    });

    namespace.define_pipeline_item("mod", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("mod")?;
        let arg_object = &ctx.resolve_pipeline(
            args.get_object("value").err_prefix("mod(value)")?,
            "mod(value)",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("mod(value)")?;
        Ok(Object::from((input % arg).err_prefix("mod")?))
    });

    namespace.define_pipeline_item("max", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("max")?;
        let arg_object = ctx.resolve_pipeline(
            args.get_object("value").err_prefix("max(value)")?,
            "max(value)",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("max(value)")?;
        Ok(if input > arg {
            arg_object
        } else {
            ctx.value().clone()
        })
    });

}
