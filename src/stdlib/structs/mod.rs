use std::str::FromStr;
use bigdecimal::BigDecimal;
use bson::oid::ObjectId;
use chrono::SecondsFormat;
use maplit::btreemap;
use teo_teon::Value;
use crate::namespace::Namespace;
use crate::arguments::Arguments;
use crate::r#struct;
use crate::object::Object;
use crate::error::Error;

pub(in crate::stdlib) fn load_structs(namespace: &mut Namespace) {

    namespace.define_struct("EnvVars", |path, env_vars| {
        env_vars.define_static_function("new", move |_arguments: Arguments| async move {
            Ok(Object::from(r#struct::Object::new(path.clone(), btreemap! {})))
        });
        env_vars.define_function("subscript", move |_this: Object, arguments: Arguments| async move {
            let key: &str = arguments.get("key")?;
            if let Ok(retval) = std::env::var(key) {
                Ok(Object::from(retval))
            } else {
                Ok(Object::from(Value::Null))
            }
        });
    });

    namespace.define_struct("Null", |path, env_vars| {
        env_vars.define_static_function("new", move |_arguments: Arguments| async move {
            Ok(Object::from(Value::Null))
        });
    });

    namespace.define_struct("Bool", |path, env_vars| {
        env_vars.define_static_function("new", move |arguments: Arguments| async move {
            let from: &str = arguments.get("from")?;
            Ok(Object::from(match from {
                "true" => true,
                "false" => false,
                _ => Err(Error::new("Bool.new: invalid argument"))?
            }))
        });
    });

    namespace.define_struct("Int", |path, env_vars| {
        env_vars.define_static_function("new", move |arguments: Arguments| async move {
            let from: &str = arguments.get("from")?;
            Ok(Object::from(match i32::from_str(from) {
                Ok(v) => v,
                Err(_) => Err(Error::new("Int.new: invalid argument"))?
            }))
        });
    });

    namespace.define_struct("Int64", |path, env_vars| {
        env_vars.define_static_function("new", move |arguments: Arguments| async move {
            let from: &str = arguments.get("from")?;
            Ok(Object::from(match i64::from_str(from) {
                Ok(v) => v,
                Err(_) => Err(Error::new("Int64.new: invalid argument"))?
            }))
        });
    });

    namespace.define_struct("Float", |path, env_vars| {
        env_vars.define_static_function("new", move |arguments: Arguments| async move {
            let from: &str = arguments.get("from")?;
            Ok(Object::from(match f32::from_str(from) {
                Ok(v) => v,
                Err(_) => Err(Error::new("Float32.new: invalid argument"))?
            }))
        });
    });

    namespace.define_struct("Float64", |path, env_vars| {
        env_vars.define_static_function("new", move |arguments: Arguments| async move {
            let from: &str = arguments.get("from")?;
            Ok(Object::from(match f64::from_str(from) {
                Ok(v) => v,
                Err(_) => Err(Error::new("Float.new: invalid argument"))?
            }))
        });
    });

    namespace.define_struct("Decimal", |path, env_vars| {
        env_vars.define_static_function("new", move |arguments: Arguments| async move {
            let from: &str = arguments.get("from")?;
            Ok(Object::from(match BigDecimal::from_str(from) {
                Ok(v) => v,
                Err(_) => Err(Error::new("Float.new: invalid argument"))?
            }))
        });
    });

    namespace.define_struct("Decimal", |path, env_vars| {
        env_vars.define_static_function("new", move |arguments: Arguments| async move {
            let from: &str = arguments.get("from")?;
            Ok(Object::from(match BigDecimal::from_str(from) {
                Ok(v) => v,
                Err(_) => Err(Error::new("Float.new: invalid argument"))?
            }))
        });
    });

    namespace.define_struct("String", |path, env_vars| {
        env_vars.define_static_function("new", move |arguments: Arguments| async move {
            let from: &Value = arguments.get("from")?;
            Ok(Object::from(match from {
                Value::Null => "Null".to_owned(),
                Value::Bool(b) => if *b { "true" } else { "false" }.to_owned(),
                Value::Int(i) => i.to_string(),
                Value::Int64(i) => i.to_string(),
                Value::Float32(f) => f.to_string(),
                Value::Float(f) => f.to_string(),
                Value::Decimal(d) => d.normalized().to_string(),
                Value::ObjectId(o) => o.to_hex(),
                Value::String(s) => s.clone(),
                Value::Date(d) => d.format("%Y-%m-%d").to_string(),
                Value::DateTime(d) => d.to_rfc3339_opts(SecondsFormat::Millis, true),
                Value::Array(_) => Err(Error::new("String.new: array is not valid"))?,
                Value::Dictionary(_) => Err(Error::new("String.new: dictionary is not valid"))?,
                Value::Range(_) => Err(Error::new("String.new: range is not valid"))?,
                Value::Tuple(_) => Err(Error::new("String.new: tuple is not valid"))?,
                Value::EnumVariant(_) => Err(Error::new("String.new: enum variant is not valid"))?,
                Value::Regex(_) => Err(Error::new("String.new: regex is not valid"))?,
                Value::File(_) => Err(Error::new("String.new: file is not valid"))?,
            }))
        });
    });

    namespace.define_struct("ObjectId", |path, env_vars| {
        env_vars.define_static_function("new", move |arguments: Arguments| async move {
            let from: &str = arguments.get("from")?;
            match ObjectId::from_str(from) {
                Ok(o) => Ok(Object::from(o)),
                Err(_) => Err(Error::new("ObjectId.new: argument is invalid"))?,
            }
        });
    });
}