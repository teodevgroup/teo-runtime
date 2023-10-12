use num_integer::Integer;
use teo_teon::types::range::Range;
use teo_teon::Value;
use crate::arguments::Arguments;
use crate::error::Error;
use crate::namespace::Namespace;
use crate::object::Object;
use crate::pipeline::Ctx;
use crate::result::Result;

pub(in crate::stdlib) fn load_pipeline_number_items(namespace: &mut Namespace) {

    namespace.define_pipeline_item("isEven", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("isEven")?;
        match input {
            Value::Int(i) => if !i.is_even() {
                Err(Error::new("input is not even"))?
            },
            Value::Int64(i) => if !i.is_even() {
                Err(Error::new("input is not even"))?
            },
            _ => Err(Error::new("isEven: invalid input"))?
        }
        Ok(ctx.value().clone())
    });

    namespace.define_pipeline_item("randomInt", |args: Arguments, ctx: Ctx| async move {
        let length: Result<i32> = args.get("length");
        let range: Result<&Range> = args.get("range");
        if length.is_err() && range.is_err() {
            Err(Error::new("randomInt: invalid argument"))?
        }
        if let Ok(length) = length {
            Ok(Object::from(Value::Int(0)))
        } else if let Ok(range) = range {
            Ok(Object::from(Value::Int(0)))
        } else {
            unreachable!()
        }
    });
}
