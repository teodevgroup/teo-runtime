use teo_parser::r#type::Type;
use crate::arguments::Arguments;
use teo_result::Error;
use crate::namespace::Namespace;
use crate::pipeline::Ctx;
use crate::pipeline::pipeline::Pipeline;
use teo_result::{Result, ResultExt};
use tokio::net::unix::pipe::pipe;
use crate::action::Action;
use crate::value::Value;

pub(in crate::stdlib) fn load_pipeline_logical_items(namespace: &mut Namespace) {

    namespace.define_pipeline_item("valid", |args: Arguments, ctx: Ctx| async move {
        Ok(ctx.value().clone())
    });

    namespace.define_pipeline_item("invalid", |args: Arguments, ctx: Ctx| async move {
        Err(Error::new("input is invalid"))
    });

    namespace.define_pipeline_item("validate", |args: Arguments, ctx: Ctx| async move {
        let pipeline: &Pipeline = args.get("pipeline").error_message_prefixed("validate")?;
        if let Err(err) = ctx.run_pipeline_ignore_return_value(pipeline).await {
            Err(err)?
        }
        Ok(ctx.value().clone())
    });

    namespace.define_pipeline_item("passed", |args: Arguments, ctx: Ctx| async move {
        let pipeline: &Pipeline = args.get("pipeline").error_message_prefixed("validate")?;
        Ok(Value::from(ctx.run_pipeline_ignore_return_value(pipeline).await.is_ok()))
    });

    namespace.define_pipeline_item("if", |args: Arguments, ctx: Ctx| async move {
        let cond: &Pipeline = args.get("cond").error_message_prefixed("if")?;
        let then: Result<&Pipeline> = args.get("then");
        let r#else: Result<&Pipeline> = args.get("else");
        match ctx.run_pipeline(cond).await {
            Ok(object) => {
                if let Ok(then) = then {
                    ctx.alter_value(object).run_pipeline(then).await
                } else {
                    Ok(object)
                }
            },
            Err(_) => {
                if let Ok(r#else) = r#else {
                    ctx.run_pipeline(r#else).await
                } else {
                    Ok(ctx.value().clone())
                }
            },
        }
    });

    namespace.define_pipeline_item("do", |args: Arguments, ctx: Ctx| async move {
        let pipeline: &Pipeline = args.get("pipeline").error_message_prefixed("do")?;
        let _ = ctx.run_pipeline_ignore_return_value(pipeline).await?;
        Ok(ctx.value().clone())
    });

    namespace.define_pipeline_item("not", |args: Arguments, ctx: Ctx| async move {
        let pipeline: &Pipeline = args.get("pipeline").error_message_prefixed("not")?;
        match ctx.run_pipeline_ignore_return_value(pipeline).await {
            Ok(_) => Err(Error::invalid_request_message("not: value is not invalid")),
            Err(_) => Ok(ctx.value().clone())
        }
    });

    namespace.define_pipeline_item("all", |args: Arguments, ctx: Ctx| async move {
        let pipelines: Vec<&Pipeline> = args.get("pipeline").error_message_prefixed("all")?;
        for pipeline in pipelines {
            ctx.run_pipeline_ignore_return_value(pipeline).await?;
        }
        Ok(ctx.value().clone())
    });

    namespace.define_pipeline_item("any", |args: Arguments, ctx: Ctx| async move {
        let pipelines: Vec<&Pipeline> = args.get("pipeline").error_message_prefixed("all")?;
        for pipeline in pipelines {
            if let Ok(_) = ctx.run_pipeline_ignore_return_value(pipeline).await {
                return Ok(ctx.value().clone());
            };
        }
        Err(Error::new("any: none of the conditions succeed"))
    });

    namespace.define_pipeline_item("when", |args: Arguments, ctx: Ctx| async move {
        let action: Action = args.get("action")?;
        let pipeline: Pipeline = args.get("pipeline")?;
        let object_action = if ctx.action().is_empty() {
            ctx.object().action()
        } else {
            ctx.action()
        };
        if object_action.passes(&vec![action]) {
            let result = ctx.run_pipeline(&pipeline).await?;
            Ok(result)
        } else {
            Ok(ctx.value().clone())
        }
    });

    namespace.define_pipeline_item("match", |args: Arguments, ctx: Ctx| async move {
        // todo!()
    });

    namespace.define_pipeline_item("case", |args: Arguments, ctx: Ctx| async move {
        // todo!()
    });

    namespace.define_pipeline_item("cast", |args: Arguments, ctx: Ctx| async move {

        let target_type: &Type = args.get("target")?;
        if ctx.value().is_of_type(target_type, ctx.transaction_ctx().namespace()) {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new("cannot cast to target type"))
        }
    });

    namespace.define_pipeline_item("asAny", |args: Arguments, ctx: Ctx| async move {
        Ok::<Value, Error>(ctx.value().clone())
    });
}