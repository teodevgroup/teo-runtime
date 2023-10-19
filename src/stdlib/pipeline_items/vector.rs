use crate::namespace::Namespace;
use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use teo_result::ResultExt;
use crate::object::Object;
use teo_result::Error;
use teo_teon::Value;

pub(in crate::stdlib) fn load_pipeline_vector_items(namespace: &mut Namespace) {

    namespace.define_pipeline_item("append", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("append")?;
        let arg_object = ctx.resolve_pipeline(
            args.get_object("value").err_prefix("append(value)")?,
            "append(value)",
        ).await?;
        let arg: Value = arg_object.try_into_err_prefix("append(value)")?;
        match input {
            Value::String(s) => {
                if !arg.is_string() {
                    Err(Error::new("append(value): value is not string"))?
                }
                Ok(Object::from(s.clone() + arg.as_str().unwrap()))
            },
            Value::Array(v) => {
                let mut new_array = v.clone();
                new_array.push(arg);
                Ok(Object::from(new_array))
            },
            _ => Err(Error::new("append: input is not array or string"))
        }
    });

    namespace.define_pipeline_item("prepend", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("prepend")?;
        let arg_object = ctx.resolve_pipeline(
            args.get_object("value").err_prefix("prepend(value)")?,
            "prepend(value)" ,
        ).await?;
        let mut arg: Value = arg_object.try_into_err_prefix("prepend(value)")?;
        match input {
            Value::String(s) => {
                if !arg.is_string() {
                    Err(Error::new("prepend(value): value is not string"))?
                }
                Ok(Object::from(arg.as_str().unwrap().to_owned() + &s))
            },
            Value::Array(v) => {
                let mut new_array = v.clone();
                new_array.insert(0, arg);
                Ok(Object::from(new_array))
            },
            _ => Err(Error::new("prepend: input is not array or string"))
        }
    });

    namespace.define_pipeline_item("getLength", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("getLength")?;
        Ok(match input {
            Value::String(s) => Object::from(s.len() as i32),
            Value::Array(v) => Object::from(v.len() as i32),
            _ => Err(Error::new("getLength: input is not array or string"))?
        })
    });

    namespace.define_pipeline_item("hasLength", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("hasLength")?;
        let (lower , upper , closed) = if input.is_any_int(){
            let n = input.to_usize().unwrap();
            (n ,n ,true)
        } else if input.is_range() {
            let r = input.as_range().unwrap();
            let start = r.start.to_usize().unwrap();
            let end = r.end.to_usize().unwrap();
            (start, end, r.closed)
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
                Ok(Object::from(input.as_int().unwrap()))
            }
        } else {
            if len >= upper {
                Err(Error::new(format!("input length is not between {lower} and {upper}")))?
            } else {
                Ok(Object::from(input.as_int().unwrap()))
            }
        }
    });

    namespace.define_pipeline_item("reverse", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("reverse")?;
        match input {
            Value::String(s) => Ok(Object::from(s.chars().rev().collect::<String>())),
            Value::Array(v) => Ok(Object::from(v.into_iter().rev().map(|v| v.clone()).collect::<Vec<Value>>())),
            _ => Err(Error::new("reverse: input is not array or string"))?
        }
    });

    namespace.define_pipeline_item("truncate", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("truncate")?;
        let arg_object = &ctx.resolve_pipeline(
            args.get_object("maxLen").err_prefix("truncate(maxLen)")?,
            "truncate(maxLen)",
        ).await?;
        let arg: usize = arg_object.try_into_err_prefix("truncate(maxLen)")?;
        match input {
            Value::String(s) => Ok(Object::from(s.chars().take(arg).collect::<String>())),
            Value::Array(v) => Ok(Object::from(v.iter().take(arg).map(|v| v.clone()).collect::<Vec<Value>>())),
            _ => Err(Error::new("truncate: input is not array or string"))?
        }
    });
}