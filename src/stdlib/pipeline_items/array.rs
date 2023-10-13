use crate::namespace::Namespace;
use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use crate::result::ResultExt;
use crate::object::Object;
use crate::error::Error;
use teo_teon::Value;

pub(in crate::stdlib) fn load_pipeline_array_items(namespace: &mut Namespace) {

    // namespace.define_pipeline_item("append", |args: Arguments, ctx: Ctx| async move {
    //     let input: &Value = ctx.value().try_into_err_prefix("append")?;
    //     let array_object = ctx.resolve_pipeline(
    //         args.get_object("value").err_prefix("append(value)")?,
    //         "append(value)" ,
    //     ).await?;
    //     let arg_array: &Value = array_object.try_into_err_prefix("append(value)")?;
    //     match input{
    //         Value::String(s) => {
    //             match arg_array {
    //                 Value::String(a) => Ok(Object::from(s.to_owned() + &a)),
    //                 _ => Err(Error::new("Argument can't match"))?
    //             }
    //         }
    //         Value::Array(v) => {
    //             let mut v = v.clone();
    //             match arg_array {
    //                 Value::Array(a) => { 
    //                     for item in a.iter() {
    //                         v.push(item.clone()) ;
    //                     }
    //                     Ok(Object::from(  v  )) 
    //                 }
    //                 _ => Err(Error::new("Argument can't match"))?
    //             }
    //         }
    //         _ => Err(Error::new("input is not array"))?
    //     }
    // });

    // namespace.define_pipeline_item("prepend", |args: Arguments, ctx: Ctx| async move {
    //     let input: &str = ctx.value().try_into_err_prefix("prepend")?;
    //     let array_object = ctx.resolve_pipeline( 
    //         args.get_object("value").err_prefix("prepend(value)"),
    //         "prepend(value)" ,
    //     ).await?;
    //     let mut arg_array: Vec = array_object.get("value").err_prefix("prepend(value)").clone();
    //     arg_array.push(input);
    //     Ok(Object::from(arg_array))
    // });

    namespace.define_pipeline_item("getLength", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("getLength")?;
        Ok( match input {
            Value::String(s) =>   Object::from(s.len() as i32),
            Value::Array(v) =>    Object::from(v.len() as i32),
            _ => Err(Error::new("input is not array or string"))?
        })
    });

//     namespace.define_pipeline_item("hasLength", |args: Arguments, ctx: Ctx| async move {
//         let input: &str = ctx.value().try_into_err_prefix("hasLength")?;
//     });

//     namespace.define_pipeline_item("reverse", |args: Arguments, ctx: Ctx| async move {
//         let input: &str = ctx.value().try_into_err_prefix("reverse")?;
//     });

//     namespace.define_pipeline_item("truncate", |args: Arguments, ctx: Ctx| async move {
//         let input: &str = ctx.value().try_into_err_prefix("truncate")?;
//     });
}