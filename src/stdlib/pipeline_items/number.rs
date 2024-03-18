use num_integer::Integer;
use teo_teon::types::range::Range;
use teo_teon::Value;
use crate::arguments::Arguments;
use teo_result::Error;
use crate::namespace::Namespace;
use crate::object::Object;
use crate::pipeline::Ctx;
use teo_result::{Result, ResultExt};
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
        let range: &Range = args.get("range").error_message_prefixed("randomFloat")?;
        let (start, end, closed) = {
            let start = if let Some(f) = range.start.to_float() {
                f
            } else {
                Err(Error::new("randomFloat: range start is not float"))?
            };
            let end   = if let Some(f) = range.end.to_float() {
                f
            } else {
                Err(Error::new("randomFloat: range end is not float"))?
            };
            (start, end, range.closed)
        };
        let mut rng = thread_rng();
        Ok(Object::from(if closed {
            rng.gen_range(start..=end)
        } else {
            rng.gen_range(start..end)
        }))
    });

    namespace.define_pipeline_item("randomInt", |args: Arguments, ctx: Ctx| async move {
        let length: Result<i32> = args.get("length");
        let range: Result<&Range> = args.get("range");
        if length.is_err() && range.is_err() {
            Err(Error::new("randomInt: invalid argument"))?
        }
        let (start, end, closed) = {
            if let Ok(length) = length {
                if length > 0 && length < 10 {
                    (10_i32.pow((length - 1) as u32), 10_i32.pow(length as u32), false)
                } else if length == 10 {
                    (10_i32.pow(9), 2147483647, true)
                } else {
                    Err(Error::new("randomInt(length): length should be between 1 and 10"))?
                }
            } else if let Ok(range) = range {
                let start = if let Some(f) = range.start.to_int() {
                    f
                } else {
                    Err(Error::new("randomInt: range start is not int"))?
                };
                let end   = if let Some(f) = range.end.to_int() {
                    f
                } else {
                    Err(Error::new("randomInt: range end is not int"))?
                };
                (start, end, range.closed)
            } else {
                unreachable!()
            }
        };
        let mut rng = thread_rng();
        Ok(Object::from(if closed {
            rng.gen_range(start..=end)
        } else {
            rng.gen_range(start..end)
        }))
    });

}
