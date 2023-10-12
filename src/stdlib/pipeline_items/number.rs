use num_integer::Integer;
use teo_teon::types::range::Range;
use teo_teon::Value;
use crate::arguments::Arguments;
use crate::error::Error;
use crate::namespace::Namespace;
use crate::object::Object;
use crate::pipeline::Ctx;
use crate::result::Result;
use rand::{thread_rng, Rng};

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

    namespace.define_pipeline_item("isOdd", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("isOdd")?;
        match input {
            Value::Int(i) => if !i.is_odd() {
                Err(Error::new("input is not odd"))?
            },
            Value::Int64(i) => if !i.is_odd() {
                Err(Error::new("input is not odd"))?
            },
            _ => Err(Error::new("isOdd: invalid input"))?
        }
        Ok(ctx.value().clone())
    });

    namespace.define_pipeline_item("randomFloat", |args: Arguments, ctx: Ctx| async move {
        let range: Result<&Range> = args.get("range");
        if range.is_err() {
            Err(Error::new("randomFloat: invalid argument"))?
        }
        if let Ok(range) = range {
            let (start, end, closed) = {
                let r = range.clone() ;
                let start = r.start.to_float().unwrap();
                let end   = r.end  .to_float().unwrap();
                (start , end , r.closed)
            };
            let mut rng = thread_rng();
            let random_number;
            if closed {
                random_number = rng.gen_range(start..end);
            }else {
                random_number = rng.gen_range(start..end);
            }
            Ok(Object::from(Value::Float(random_number)))
        } else {
            unreachable!()
        }
    });

    namespace.define_pipeline_item("randomInt", |args: Arguments, ctx: Ctx| async move {
        let length: Result<i32> = args.get("length");
        let range: Result<&Range> = args.get("range");
        if length.is_err() && range.is_err() {
            Err(Error::new("randomInt: invalid argument"))?
        }
        if let Ok(length) = length {
            if length > 0 && length < 10 {
                let len_int = length.try_into().unwrap();
                let ran_num: i32 = thread_rng().gen_range(10_i32.pow(len_int - 1)..10_i32.pow(len_int));
                Ok(Object::from(Value::Int(ran_num)))
            } else if length == 10 {
                let ran_num: i32 = thread_rng().gen_range(10_i32.pow(9)..2147483647);
                Ok(Object::from(Value::Int(ran_num)))
            } else { 
                Err(Error::new("randomInt: invalid argument"))?
            }
        } else if let Ok(range) = range {
            let start = range.start.to_int().unwrap();
            let end   = range.end.  to_int().unwrap();
            let ran_num: i32 = thread_rng().gen_range(start..end);
            Ok(Object::from(Value::Int(ran_num)))
        } else {
            unreachable!()
        }
    });

}
