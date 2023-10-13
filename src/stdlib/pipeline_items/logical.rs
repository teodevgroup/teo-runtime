use crate::arguments::Arguments;
use crate::error::Error;
use crate::namespace::Namespace;
use crate::object::Object;
use crate::pipeline::Ctx;
use crate::pipeline::pipeline::Pipeline;
use crate::result::{Result, ResultExt};

pub(in crate::stdlib) fn load_pipeline_logical_items(namespace: &mut Namespace) {

    namespace.define_pipeline_item("validate", |args: Arguments, ctx: Ctx| async move {
        let pipeline: &Pipeline = args.get("pipeline").err_prefix("validate")?;
        if let Err(err) = ctx.run_pipeline(pipeline).await {
            Err(err)?
        }
        Ok(ctx.value().clone())
    });

    namespace.define_pipeline_item("passed", |args: Arguments, ctx: Ctx| async move {
        let pipeline: &Pipeline = args.get("pipeline").err_prefix("validate")?;
        Ok(Object::from(ctx.run_pipeline(pipeline).await.is_ok()))
    });

    namespace.define_pipeline_item("if", |args: Arguments, ctx: Ctx| async move {
        let cond: &Pipeline = args.get("cond").err_prefix("if")?;
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

    namespace.define_pipeline_item("presents", |args: Arguments, ctx: Ctx| async move {
        if ctx.value().is_null() {
            Err(Error::new("value is not present"))?
        }
        Ok(ctx.value().clone())
    });
}