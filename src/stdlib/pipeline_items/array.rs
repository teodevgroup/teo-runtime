use crate::namespace::Namespace;
use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use teo_result::ResultExt;
use crate::object::Object;
use crate::value::Value;
use crate::pipeline::pipeline::Pipeline;

pub(in crate::stdlib) fn load_pipeline_array_items(namespace: &mut Namespace) {

    namespace.define_pipeline_item("join", |args: Arguments, ctx: Ctx| async move {
        let input: Vec<&str> = ctx.value().try_ref_into_err_prefix("join")?;
        let separator_object: Object = ctx.resolve_pipeline_with_err_prefix(
            args.get_object("separator").error_message_prefixed("join(separator)")?,
            "join(separator)",
        ).await?;
        let separator: &str = separator_object.try_ref_into_err_prefix("join(separator)")?;
        Ok(Object::from(input.join(separator)))
    });

    namespace.define_pipeline_item("map", |args: Arguments, ctx: Ctx| async move {
        let input: &Vec<Value> = ctx.value().try_ref_into_err_prefix("map")?;
        let pipeline: &Pipeline = args.get("pipeline").error_message_prefixed("map(pipeline)")?;
        let mut result: Vec<Value> = vec![];
        for (index, item) in input.iter().enumerate() {
            result.push(ctx.alter_value(Object::from(item)).run_pipeline_with_err_prefix(pipeline, format!("map({index}))")).await?);
        }
        Ok(Object::from(result))
    });

    namespace.define_pipeline_item("filter", |args: Arguments, ctx: Ctx| async move {
        let input: &Vec<Value> = ctx.value().try_ref_into_err_prefix("filter")?;
        let pipeline: &Pipeline = args.get("pipeline").error_message_prefixed("filter(pipeline)")?;
        let mut result: Vec<Value> = vec![];
        for item in input.iter() {
            if ctx.alter_value(Object::from(item)).run_pipeline_ignore_return_value(pipeline).await.is_ok() {
                result.push(item.clone());
            }
        }
        Ok(Object::from(result))
    });
}