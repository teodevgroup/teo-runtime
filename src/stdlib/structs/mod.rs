use std::str::FromStr;
use bigdecimal::BigDecimal;
use bson::oid::ObjectId;
use chrono::{DateTime, NaiveDate, SecondsFormat, Utc};
use indexmap::IndexMap;
use maplit::btreemap;
use crate::value::Value;
use crate::arguments::Arguments;
use crate::r#struct;
use teo_result::Error;
use crate::namespace::builder::NamespaceBuilder;

pub(in crate::stdlib) fn load_structs(namespace_builder: &NamespaceBuilder) {

    namespace_builder.define_struct("EnvVars", |path, r#struct| {
        r#struct.define_static_function("new", move |_arguments: Arguments| {
            Ok(Value::from(r#struct::Object::new(path.clone(), btreemap! {})))
        });
        r#struct.define_function("subscript", move |_this: Value, arguments: Arguments| {
            let key: &str = arguments.get("key")?;
            if let Ok(retval) = std::env::var(key) {
                Ok(Value::from(retval))
            } else {
                Ok(Value::from(Value::Null))
            }
        });
    });

    namespace_builder.define_struct("Null", |path, r#struct| {
        r#struct.define_static_function("new", move |_arguments: Arguments| {
            Ok(Value::from(Value::Null))
        });
    });

    namespace_builder.define_struct("Bool", |path, r#struct| {
        r#struct.define_static_function("new", move |arguments: Arguments| {
            let from: &str = arguments.get("from")?;
            Ok(Value::from(match from {
                "true" => true,
                "false" => false,
                _ => Err(Error::new("Bool.new: invalid argument"))?
            }))
        });
    });

    namespace_builder.define_struct("Int", |path, r#struct| {
        r#struct.define_static_function("new", move |arguments: Arguments| {
            let from: &str = arguments.get("from")?;
            Ok(Value::from(match i32::from_str(from) {
                Ok(v) => v,
                Err(_) => Err(Error::new("Int.new: invalid argument"))?
            }))
        });
    });

    namespace_builder.define_struct("Int64", |path, r#struct| {
        r#struct.define_static_function("new", move |arguments: Arguments| {
            let from: &str = arguments.get("from")?;
            Ok(Value::from(match i64::from_str(from) {
                Ok(v) => v,
                Err(_) => Err(Error::new("Int64.new: invalid argument"))?
            }))
        });
    });

    namespace_builder.define_struct("Float32", |path, r#struct| {
        r#struct.define_static_function("new", move |arguments: Arguments| {
            let from: &str = arguments.get("from")?;
            Ok(Value::from(match f32::from_str(from) {
                Ok(v) => v,
                Err(_) => Err(Error::new("Float32.new: invalid argument"))?
            }))
        });
    });

    namespace_builder.define_struct("Float", |path, r#struct| {
        r#struct.define_static_function("new", move |arguments: Arguments| {
            let from: &str = arguments.get("from")?;
            Ok(Value::from(match f64::from_str(from) {
                Ok(v) => v,
                Err(_) => Err(Error::new("Float.new: invalid argument"))?
            }))
        });
    });

    namespace_builder.define_struct("Decimal", |path, r#struct| {
        r#struct.define_static_function("new", move |arguments: Arguments| {
            let from: &str = arguments.get("from")?;
            Ok(Value::from(match BigDecimal::from_str(from) {
                Ok(v) => v,
                Err(_) => Err(Error::new("Float.new: invalid argument"))?
            }))
        });
    });

    namespace_builder.define_struct("String", |path, r#struct| {
        r#struct.define_static_function("new", move |arguments: Arguments| {
            let from: &Value = arguments.get("from")?;
            Ok(Value::from(match from {
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
                Value::Regex(_) => Err(Error::new("String.new: regex is not valid"))?,
                Value::File(_) => Err(Error::new("String.new: file is not valid"))?,
                Value::OptionVariant(_) => Err(Error::new("String.new: option variant is not valid"))?,
                Value::InterfaceEnumVariant(_) => Err(Error::new("String.new: interface enum variant is not valid"))?,
                Value::ModelObject(_) => Err(Error::new("String.new: model object is not valid"))?,
                Value::StructObject(_) => Err(Error::new("String.new: struct object is not valid"))?,
                Value::Pipeline(_) => Err(Error::new("String.new: pipeline is not valid"))?,
                Value::Type(_) => Err(Error::new("String.new: type as value argument is not valid"))?,
            }))
        });
    });

    namespace_builder.define_struct("ObjectId", |path, r#struct| {
        r#struct.define_static_function("new", move |arguments: Arguments| {
            let from: &str = arguments.get("from")?;
            match ObjectId::from_str(from) {
                Ok(o) => Ok(Value::from(o)),
                Err(_) => Err(Error::new("ObjectId.new: argument is invalid"))?,
            }
        });
    });

    namespace_builder.define_struct("Date", |path, r#struct| {
        r#struct.define_static_function("new", move |arguments: Arguments| {
            let from: &str = arguments.get("from")?;
            match NaiveDate::parse_from_str(from, "%Y-%m-%d") {
                Ok(o) => Ok(Value::from(o)),
                Err(_) => Err(Error::new("Date.new: argument is invalid"))?,
            }
        });
    });

    namespace_builder.define_struct("DateTime", |path, r#struct| {
        r#struct.define_static_function("new", move |arguments: Arguments| {
            let from: &str = arguments.get("from")?;
            match DateTime::parse_from_rfc3339(from) {
                Ok(o) => Ok(Value::from(o.with_timezone(&Utc))),
                Err(_) => Err(Error::new("DateTime.new: argument is invalid"))?,
            }
        });
    });

    namespace_builder.define_struct("File", |path, r#struct| { });

    namespace_builder.define_struct("Regex", |path, r#struct| { });

    namespace_builder.define_struct("Range", |path, r#struct| { });

    namespace_builder.define_struct("Array", |path, r#struct| {
        r#struct.define_static_function("new", move |arguments: Arguments| {
            Ok(Value::from(Vec::<Value>::new()))
        });
        r#struct.define_function("subscript", move |this: Value, arguments: Arguments| {
            let index: usize = arguments.get("key")?;
            let this: &Vec<Value> = this.as_ref().try_into()?;
            if let Some(value) = this.get(index) {
                Ok(Value::from(value))
            } else {
                Err(Error::new("Array.subscript: index out of bounds"))
            }
        });
    });

    namespace_builder.define_struct("Dictionary", |path, r#struct| {
        r#struct.define_static_function("new", move |arguments: Arguments| {
            Ok(Value::from(IndexMap::<String, Value>::new()))
        });
        r#struct.define_function("subscript", move |this: Value, arguments: Arguments| {
            let index: &str = arguments.get("key")?;
            let this: &IndexMap<String, Value> = this.as_ref().try_into()?;
            if let Some(value) = this.get(index) {
                Ok(Value::from(value))
            } else {
                Err(Error::new("Dictionary.subscript: index out of bounds"))
            }
        });
    });
}