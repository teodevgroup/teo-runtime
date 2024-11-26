use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use teo_result::ResultExt;
use teo_result::Error;
use crate::namespace;
use crate::pipeline::item::item_impl::ItemImpl;
use crate::value::range::Range;
use crate::value::Value;

pub(in crate::stdlib) fn load_pipeline_vector_items(namespace: &namespace::Builder) {

    namespace.define_pipeline_item("append", |args: Arguments| {
        let argument = args.get_value("value").error_message_prefixed("append(value)")?;
        Ok(ItemImpl::new(move |ctx: Ctx| {
            let argument = argument.clone();
            async move {
                let arg_object: Value = ctx.resolve_pipeline_with_err_prefix(
                    argument.clone(),
                    "append(value)",
                ).await?;
                let arg: Value = arg_object.try_into_err_prefix("append(value)")?;
                match ctx.value() {
                    Value::String(s) => {
                        if !arg.is_string() {
                            Err(Error::new("append(value): value is not string"))?
                        }
                        Ok(Value::from(s.clone() + arg.as_str().unwrap()))
                    },
                    Value::Array(v) => {
                        let mut new_array = v.clone();
                        new_array.push(arg);
                        Ok(Value::from(new_array))
                    },
                    _ => Err(Error::new("append: input is not array or string"))
                }
            }
        }))
    });

    namespace.define_pipeline_item("prepend", |args: Arguments| {
        let argument = args.get_value("value").error_message_prefixed("prepend(value)")?;
        Ok(ItemImpl::new(move |ctx: Ctx| {
            let argument = argument.clone();
            async move {
                let input: &Value = ctx.value().try_ref_into_err_prefix("prepend")?;
                let arg_object: Value = ctx.resolve_pipeline_with_err_prefix(
                    argument.clone(),
                    "prepend(value)" ,
                ).await?;
                let arg: Value = arg_object.try_into_err_prefix("prepend(value)")?;
                match input {
                    Value::String(s) => {
                        if !arg.is_string() {
                            Err(Error::new("prepend(value): value is not string"))?
                        }
                        Ok(Value::from(arg.as_str().unwrap().to_owned() + &s))
                    },
                    Value::Array(v) => {
                        let mut new_array = v.clone();
                        new_array.insert(0, arg);
                        Ok(Value::from(new_array))
                    },
                    _ => Err(Error::new("prepend: input is not array or string"))
                }
            }
        }))
    });

    namespace.define_pipeline_item("getLength", |args: Arguments| {
        Ok(ItemImpl::new(|ctx: Ctx| async move {
            let input: &Value = ctx.value().try_ref_into_err_prefix("getLength")?;
            Ok(match input {
                Value::String(s) => Value::from(s.len() as i32),
                Value::Array(v) => Value::from(v.len() as i32),
                _ => Err(Error::new("getLength: input is not array or string"))?
            })
        }))
    });

    namespace.define_pipeline_item("hasLength", |args: Arguments| {
        let len_arg: Option<usize> = args.get_optional("len").error_message_prefixed("hasLength(len)")?.try_into()?;
        let range_arg: Option<Range> = args.get_optional("range").error_message_prefixed("hasLength(range)")?.try_into()?;
        Ok(ItemImpl::new(move |ctx: Ctx| {
            let len_arg = len_arg.clone();
            let range_arg = range_arg.clone();
            async move {
                let input: &Value = ctx.value().try_ref_into_err_prefix("hasLength")?;
                let (lower , upper , closed) = if let Some(len) = len_arg {
                    (len, len, true)
                } else if let Some(range) = range_arg {
                    let start = range.start.to_usize().unwrap();
                    let end = range.end.to_usize().unwrap();
                    (start, end, range.closed)
                } else {
                    unreachable!()
                };
                let len = match input {
                    Value::String(s) => s.len(),
                    Value::Array(v) => v.len(),
                    _ => Err(Error::new("hasLength: input is not array or string"))?
                };
                if len < lower {
                    Err(Error::new(format!("input length is not between {lower} and {upper}")))?
                }
                if closed {
                    if len > upper {
                        Err(Error::new(format!("input length is not between {lower} and {upper}")))?
                    } else {
                        Ok(ctx.value().clone())
                    }
                } else {
                    if len >= upper {
                        Err(Error::new(format!("input length is not between {lower} and {upper}")))?
                    } else {
                        Ok(ctx.value().clone())
                    }
                }
            }
        }))
    });

    namespace.define_pipeline_item("reverse", |args: Arguments| {
        Ok(ItemImpl::new(|ctx: Ctx| async move {
            let input: &Value = ctx.value().try_ref_into_err_prefix("reverse")?;
            match input {
                Value::String(s) => Ok(Value::from(s.chars().rev().collect::<String>())),
                Value::Array(v) => Ok(Value::from(v.into_iter().rev().map(|v| v.clone()).collect::<Vec<Value>>())),
                _ => Err(Error::new("reverse: input is not array or string"))?
            }
        }))
    });

    namespace.define_pipeline_item("truncate", |args: Arguments| {
        let max_len = args.get_value("maxLen").error_message_prefixed("truncate(maxLen)")?;
        Ok(ItemImpl::new(move |ctx: Ctx| {
            let max_len = max_len.clone();
            async move {
                let input: &Value = ctx.value().try_ref_into_err_prefix("truncate")?;
                let arg_object: Value = ctx.resolve_pipeline_with_err_prefix(
                    max_len.clone(),
                    "truncate(maxLen)",
                ).await?;
                let arg: usize = arg_object.try_ref_into_err_prefix("truncate(maxLen)")?;
                match input {
                    Value::String(s) => Ok(Value::from(s.chars().take(arg).collect::<String>())),
                    Value::Array(v) => Ok(Value::from(v.iter().take(arg).map(|v| v.clone()).collect::<Vec<Value>>())),
                    _ => Err(Error::new("truncate: input is not array or string"))?
                }
            }
        }))
    });
}