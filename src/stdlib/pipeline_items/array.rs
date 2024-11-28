use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use teo_result::ResultExt;
use crate::namespace;
use crate::value::Value;
use crate::pipeline::pipeline::Pipeline;

pub(in crate::stdlib) fn load_pipeline_array_items(namespace: &namespace::Builder) {

    namespace.define_pipeline_item("join", |args: Arguments| {
        let separator = args.get_value("separator").error_message_prefixed("join(separator)")?;
        Ok(move |ctx: Ctx| {
            let separator = separator.clone();
            async move {
                let input: Vec<&str> = ctx.value().try_ref_into_err_prefix("join")?;
                let separator_object: Value = ctx.resolve_pipeline_with_err_prefix(
                    separator.clone(),
                    "join(separator)",
                ).await?;
                let separator: &str = separator_object.try_ref_into_err_prefix("join(separator)")?;
                Ok(Value::from(input.join(separator)))
            }
        })
    });

    namespace.define_pipeline_item("map", |args: Arguments| {
        let pipeline: Pipeline = args.get("pipeline").error_message_prefixed("map(pipeline)")?;
        Ok(move |ctx: Ctx| {
            let pipeline = pipeline.clone();
            async move {
                let input: &Vec<Value> = ctx.value().try_ref_into_err_prefix("map")?;
                let mut result: Vec<Value> = vec![];
                for (index, item) in input.iter().enumerate() {
                    result.push(ctx.alter_value(Value::from(item)).run_pipeline_with_err_prefix(&pipeline, format!("map({index})")).await?);
                }
                Ok(Value::from(result))
            }
        })
    });

    namespace.define_pipeline_item("filter", |args: Arguments| {
        let pipeline: Pipeline = args.get("pipeline").error_message_prefixed("filter(pipeline)")?;
        Ok(move |ctx: Ctx| {
            let pipeline = pipeline.clone();
            async move {
                let input: &Vec<Value> = ctx.value().try_ref_into_err_prefix("filter")?;
                let mut result: Vec<Value> = vec![];
                for item in input.iter() {
                    if ctx.alter_value(Value::from(item)).run_pipeline_ignore_return_value(&pipeline).await.is_ok() {
                        result.push(item.clone());
                    }
                }
                Ok(Value::from(result))
            }
        })
    });
}