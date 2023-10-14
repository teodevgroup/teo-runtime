use crate::namespace::Namespace;
use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use crate::result::ResultExt;
use crate::object::Object;
use crate::error::Error;
use teo_teon::Value;

pub(in crate::stdlib) fn load_pipeline_vector_items(namespace: &mut Namespace) {

    namespace.define_pipeline_item("join", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("join")?;
        let arg_object = ctx.resolve_pipeline(
            args.get_object("value").err_prefix("join(value)")?,
            "(join(value)",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("join(value)")?;
        // may be can improve (try into String directly)
        match input{
            Value::Array(v) => {
                let separator = arg.as_str().unwrap();
                Ok(Object::from(v.iter().map(|v| v.as_str().unwrap()).collect::<Vec<&str>>().join(separator)))
            }
            _ => Err(Error::new("join: input is not array or string"))
        }
    });

    // namespace.define_pipeline_item("at", |args: Arguments, ctx: Ctx| async move {
    //     let input: &Value = ctx.value().try_into_err_prefix("at")?;
    //     let arg_object = ctx.resolve_pipeline(
    //         args.get_object("value").err_prefix("at(value)")?,
    //         "(at(value)",
    //     ).await?;
    //     let arg: &Value = arg_object.try_into_err_prefix("at(value)")?;
    //     // may be can improve (try into Usize directly)
    //     match input{
    //         Value::Array(v) => {
    //             let index = arg.to_usize().unwrap();
    //             let new_path = ctx.path().as_ref() + index as usize ;
    //             // Ok(Object::from(v.get(index).unwrap().clone().with_path(new_path))) 
    //             Ok(Object::from(v.get(index).unwrap().clone()))
    //         }
    //         _ => Err(Error::new("at: input is not array or string"))
    //     }
    // });

    // namespace.define_pipeline_item("map", |args: Arguments, ctx: Ctx| async move {
    //     let input: &Value = ctx.value().try_into_err_prefix("map")?;
    //     let arg_object = ctx.resolve_pipeline(
    //         args.get_object("value").err_prefix("map(value)")?,
    //         "(map(value)",
    //     ).await?;
    //     let arg: &Value = arg_object.try_into_err_prefix("map(value)")?;
    // });

    // namespace.define_pipeline_item("filter", |args: Arguments, ctx: Ctx| async move {
    //     let input: &Value = ctx.value().try_into_err_prefix("filter")?;
    //     let arg_object = ctx.resolve_pipeline(
    //         args.get_object("value").err_prefix("filter(value)")?,
    //         "(filter(value)",
    //     ).await?;
    //     let arg: &Value = arg_object.try_into_err_prefix("filter(value)")?;
    // });

}