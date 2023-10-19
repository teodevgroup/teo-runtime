use crate::namespace::Namespace;
use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use teo_result::ResultExt;
use crate::object::Object;
use teo_teon::Value;
use crate::pipeline::pipeline::Pipeline;

pub(in crate::stdlib) fn load_pipeline_array_items(namespace: &mut Namespace) {

    namespace.define_pipeline_item("join", |args: Arguments, ctx: Ctx| async move {
        let input: Vec<&str> = ctx.value().try_into_err_prefix("join")?;
        let separator_object = ctx.resolve_pipeline(
            args.get_object("separator").err_prefix("join(separator)")?,
            "join(separator)",
        ).await?;
        let separator: &str = separator_object.try_into_err_prefix("join(separator)")?;
        Ok(Object::from(input.join(separator)))
    });

    namespace.define_pipeline_item("map", |args: Arguments, ctx: Ctx| async move {
        let input: &Vec<Value> = ctx.value().try_into_err_prefix("map")?;
        let pipeline: &Pipeline = args.get("pipeline").err_prefix("map(pipeline)")?;
        let mut result: Vec<Value> = vec![];
        for (index, item) in input.iter().enumerate() {
            result.push(ctx.alter_value(Object::from(item)).run_pipeline_with_err_prefix(pipeline, format!("map({index}))")).await?.try_into_err_prefix(format!("map({index})"))?);
        }
        Ok(Object::from(result))
    });

    namespace.define_pipeline_item("filter", |args: Arguments, ctx: Ctx| async move {
        let input: &Vec<Value> = ctx.value().try_into_err_prefix("filter")?;
        let pipeline: &Pipeline = args.get("pipeline").err_prefix("filter(pipeline)")?;
        let mut result: Vec<Value> = vec![];
        for item in input.iter() {
            if ctx.alter_value(Object::from(item)).run_pipeline(pipeline).await.is_ok() {
                result.push(item.clone());
            }
        }
        Ok(Object::from(result))
    });
}