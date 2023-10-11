use bigdecimal::num_traits::{Pow, Float};
use teo_teon::Value;
use crate::arguments::Arguments;
use crate::error::Error;
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

    namespace.define_pipeline_item("min", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("min")?;
        let arg_object = ctx.resolve_pipeline(
            args.get_object("value").err_prefix("min(value)")?,
            "min(value)",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("min(value)")?;
        Ok(if input < arg {
            arg_object
        } else {
            ctx.value().clone()
        })
    });

    namespace.define_pipeline_item("floor", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("floor")?;
        Ok(match input {
            Value::Float32(f) => Object::from(f.floor()),
            Value::Float(f) => Object::from(f.floor()),
            Value::Decimal(d) => Object::from(d.with_scale(0)),
            _ => Err(Error::new("floor: value cannot be floored"))?
        })
    });

    namespace.define_pipeline_item("ceil", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("ceil")?;
        Ok(match input {
            Value::Float32(f) => Object::from(f.ceil()),
            Value::Float(f) => Object::from(f.ceil()),
            // Value::Decimal(d) => Object::from(d.with_scale(0) + 1),
            _ => Err(Error::new("ceil: value cannot be ceiled"))?
        })
    });

    namespace.define_pipeline_item("round", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("round")?;
        Ok(match input {
            Value::Float32(f) => Object::from(f.round()),
            Value::Float(f) => Object::from(f.round()),
            Value::Decimal(d) => Object::from(d.round(1)),
            _ => Err(Error::new("round: value cannot be rounded"))?
        })
    });

    namespace.define_pipeline_item("abs", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("abs")?;
        Ok(match input {
            Value::Int(i)   => Object::from(i.abs()) ,
            Value::Int64(i) => Object::from(i.abs()) ,
            Value::Float32(f) => Object::from(f.abs()),
            Value::Float(f) => Object::from(f.abs()),
            Value::Decimal(d) => Object::from(d.abs()),
            _ => Err(Error::new("abs: value cannot be absed"))?
        })
    });

    // namespace.define_pipeline_item("sqrt", |args: Arguments, ctx: Ctx| async move {
    //     let input: &Value = ctx.value().try_into_err_prefix("sqrt")?;
    //     Ok(match input {
    //         Value::Int(i)   => Object::from(i.sqrt()) ,
    //         Value::Int64(i) => Object::from(i.sqrt()) ,
    //         Value::Float32(f) => Object::from(f.sqrt()),
    //         Value::Float(f) => Object::from(f.sqrt()),
    //         Value::Decimal(d) => Object::from(d.sqrt()),
    //         _ => Err(Error::new("sqrt: value cannot be sqrted"))?
    //     })
    // });

    namespace.define_pipeline_item("pow", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("pow")?;
        let arg_object = ctx.resolve_pipeline(
            args.get_object("value").err_prefix("min(value)")?,
            "min(value)",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("min(value)")?;
        Ok(match input {
            Value::Int(i)    => Object::from(i.pow(arg.as_int().unwrap() as u32)) ,
            Value::Int64(i)  => Object::from(i.pow(arg.as_int().unwrap() as u32)) ,
            Value::Float32(f)=> Object::from(f.powf(arg.as_float32().unwrap() as f32)),
            Value::Float(f)  => Object::from(f.powf(arg.as_float32().unwrap() as f64)),
            // Value::Decimal(d) => Object::from(d.ten_to_the()),
            _ => Err(Error::new("pow: value cannot be powed"))?
        })
    });

}
