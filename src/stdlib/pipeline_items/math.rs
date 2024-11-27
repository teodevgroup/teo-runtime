use num_integer::Roots;
use std::ops::Add;
use bigdecimal::num_traits::{Pow};
use bigdecimal::BigDecimal;
use crate::value::Value;
use crate::arguments::Arguments;
use teo_result::Error;
use crate::pipeline::Ctx;
use teo_result::ResultExt;
use crate::namespace;
use crate::pipeline::item::item_impl::ItemImpl;

pub(in crate::stdlib) fn load_pipeline_math_items(namespace: &namespace::Builder) {

    namespace.define_pipeline_item("add", |args: Arguments| {
        let argument: Value = args.get("value").error_message_prefixed("add(value)")?;
        Ok(move |ctx: Ctx| {
            let argument = argument.clone();
            async move {
                let input: &Value = ctx.value().try_ref_into_err_prefix("add")?;
                let unwrapped_argument: Value = ctx.resolve_pipeline_with_err_prefix(
                    argument.clone(),
                    "add(value)",
                ).await?;
                Ok(Value::from((input + &unwrapped_argument).error_message_prefixed("add")?))
            }
        })
    });

    namespace.define_pipeline_item("sub", |args: Arguments| {
        let argument: Value = args.get("value").error_message_prefixed("sub(value)")?;
        Ok(move |ctx: Ctx| {
            let argument = argument.clone();
            async move {
                let input: &Value = ctx.value().try_ref_into_err_prefix("sub")?;
                let arg: &Value = &ctx.resolve_pipeline_with_err_prefix(
                    argument.clone(),
                    "sub(value)",
                ).await?;
                Ok(Value::from((input - arg).error_message_prefixed("sub")?))
            }
        })
    });

    namespace.define_pipeline_item("mul", |args: Arguments| {
        let argument: Value = args.get("value").error_message_prefixed("mul(value)")?;
        Ok(move |ctx: Ctx| {
            let argument = argument.clone();
            async move {
                let input: &Value = ctx.value().try_ref_into_err_prefix("mul")?;
                let arg: &Value = &ctx.resolve_pipeline_with_err_prefix(
                    argument.clone(),
                    "mul(value)",
                ).await?;
                Ok(Value::from((input * arg).error_message_prefixed("mul")?))
            }
        })
    });

    namespace.define_pipeline_item("div", |args: Arguments| {
        let argument: Value = args.get("value").error_message_prefixed("div(value)")?;
        Ok(move |ctx: Ctx| {
            let argument = argument.clone();
            async move {
                let input: &Value = ctx.value().try_ref_into_err_prefix("div")?;
                let arg: &Value = &ctx.resolve_pipeline_with_err_prefix(
                    argument.clone(),
                    "div(value)",
                ).await?;
                Ok(Value::from((input / arg).error_message_prefixed("div")?))
            }
        })
    });

    namespace.define_pipeline_item("mod", |args: Arguments| {
        let argument: Value = args.get("value").error_message_prefixed("mod(value)")?;
        Ok(move |ctx: Ctx| {
            let argument = argument.clone();
            async move {
                let input: &Value = ctx.value().try_ref_into_err_prefix("mod")?;
                let arg: &Value = &ctx.resolve_pipeline_with_err_prefix(
                    argument.clone(),
                    "mod(value)",
                ).await?;
                Ok(Value::from((input % arg).error_message_prefixed("mod")?))
            }
        })
    });

    namespace.define_pipeline_item("max", |args: Arguments| {
        let argument = args.get_value("value").error_message_prefixed("max(value)")?;
        Ok(move |ctx: Ctx| {
            let argument = argument.clone();
            async move {
                let arg_object: Value = ctx.resolve_pipeline_with_err_prefix(
                    argument.clone(),
                    "max(value)",
                ).await?;
                Ok(if ctx.value() > &arg_object {
                    arg_object
                } else {
                    ctx.value().clone()
                })
            }
        })
    });

    namespace.define_pipeline_item("min", |args: Arguments| {
        let argument = args.get_value("value").error_message_prefixed("min(value)")?;
        Ok(move |ctx: Ctx| {
            let argument = argument.clone();
            async move {
                let arg_object: Value = ctx.resolve_pipeline_with_err_prefix(
                    argument.clone(),
                    "min(value)",
                ).await?;
                Ok(if ctx.value() < &arg_object {
                    arg_object
                } else {
                    ctx.value().clone()
                })
            }
        })
    });

    namespace.define_pipeline_item("floor", |_args: Arguments| {
        Ok(|ctx: Ctx| async move {
            Ok(match ctx.value() {
                Value::Float32(f) => Value::from(f.floor()),
                Value::Float(f) => Value::from(f.floor()),
                Value::Decimal(d) => Value::from(d.with_scale(0)),
                _ => Err(Error::new("floor: invalid input"))?
            })
        })
    });

    namespace.define_pipeline_item("ceil", |_args: Arguments| {
        Ok(|ctx: Ctx| async move {
            Ok(match ctx.value() {
                Value::Float32(f) => Value::from(f.ceil()),
                Value::Float(f) => Value::from(f.ceil()),
                Value::Decimal(d) => Value::from(if d.digits() == 0 {
                    d.clone()
                } else {
                    d.with_scale(0).add(BigDecimal::from(1))
                }),
                _ => Err(Error::new("ceil: invalid input"))?
            })
        })
    });

    namespace.define_pipeline_item("round", |args: Arguments| {
        Ok(|ctx: Ctx| async move {
            Ok(match ctx.value() {
                Value::Float32(f) => Value::from(f.round()),
                Value::Float(f) => Value::from(f.round()),
                Value::Decimal(d) => Value::from(d.round(0)),
                _ => Err(Error::new("round: invalid input"))?
            })
        })
    });

    namespace.define_pipeline_item("abs", |args: Arguments| {
        Ok(|ctx: Ctx| async move {
            Ok(match ctx.value() {
                Value::Int(i) => Value::from(i.abs()) ,
                Value::Int64(i) => Value::from(i.abs()) ,
                Value::Float32(f) => Value::from(f.abs()),
                Value::Float(f) => Value::from(f.abs()),
                Value::Decimal(d) => Value::from(d.abs()),
                _ => Err(Error::new("abs: invalid input"))?
            })
        })
    });

    namespace.define_pipeline_item("sqrt", |args: Arguments| {
        Ok(|ctx: Ctx| async move {
            Ok(match ctx.value() {
                Value::Int(i)   => Value::from(i.sqrt()),
                Value::Int64(i) => Value::from(i.sqrt()),
                Value::Float32(f) => Value::from(f.sqrt()),
                Value::Float(f) => Value::from(f.sqrt()),
                Value::Decimal(d) => Value::from(if let Some(d) = d.sqrt() {
                    d
                } else {
                    Err(Error::new(format!("sqrt: decimal value '{d}' is invalid")))?
                }),
                _ => Err(Error::new("sqrt: invalid input"))?
            })
        })
    });

    namespace.define_pipeline_item("cbrt", |args: Arguments| {
        Ok(|ctx: Ctx| async move {
            Ok(match ctx.value() {
                Value::Int(i)   => Value::from((*i as f64).cbrt() as i32),
                Value::Int64(i) => Value::from((*i as f64).cbrt() as i64),
                Value::Float32(f) => Value::from(f.cbrt()),
                Value::Float(f) => Value::from(f.cbrt()),
                Value::Decimal(d) => Value::from(d.cbrt()),
                _ => Err(Error::new("cbrt: invalid input"))?
            })
        })
    });

    namespace.define_pipeline_item("pow", |args: Arguments| {
        let argument = args.get_value("value").error_message_prefixed("pow(value)")?;
        Ok(move |ctx: Ctx| {
            let argument = argument.clone();
            async move {
                let arg: Value = ctx.resolve_pipeline_with_err_prefix(
                    argument.clone(),
                    "pow(value)",
                ).await?;
                if ctx.value().is_any_int() {
                    if !arg.is_any_int() {
                        return Err(Error::new("pow(value): value is not integer"));
                    }
                } else if ctx.value().is_any_float() {
                    if !arg.is_any_int_or_float() {
                        return Err(Error::new("pow(value): value is not int or float"));
                    }
                }
                Ok(match ctx.value() {
                    Value::Int(i) => Value::from(i.pow(arg.to_int().unwrap() as u32)),
                    Value::Int64(i)   => Value::from(i.pow(arg.to_int().unwrap() as u32)),
                    Value::Float32(f) => Value::from(f.powf(arg.to_float().unwrap() as f32)),
                    Value::Float(f)   => Value::from(f.powf(arg.to_float().unwrap())),
                    _ => Err(Error::new("pow: invalid input"))?
                })

            }
        })
    });

    namespace.define_pipeline_item("root", |args: Arguments| {
        let argument = args.get_value("value").error_message_prefixed("root(value)")?;
        Ok(move |ctx: Ctx| {
            let argument = argument.clone();
            async move {
                let arg_object: Value = ctx.resolve_pipeline_with_err_prefix(
                    argument.clone(),
                    "root(value)",
                ).await?;
                let arg: i32 = arg_object.try_into_err_prefix("root(value)")?;
                Ok( match ctx.value() {
                    Value::Int(i)     => Value::from(i.nth_root(arg as u32)),
                    Value::Int64(i)   => Value::from(i.nth_root(arg as u32)),
                    _ => Err(Error::new("root: invalid input"))?
                })
            }
        })
    });
}
