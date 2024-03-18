use crate::namespace::Namespace;
use crate::arguments::Arguments;
use crate::pipeline::{Ctx, Pipeline};

pub(in crate::stdlib) fn load_debug_items(namespace: &mut Namespace) {

    namespace.define_pipeline_item("print", |args: Arguments, ctx: Ctx| async move {
        let label: Option<&str> = args.get_optional("label")?;
        println!("{}{}", if let Some(label) = label { format!("{}: ", label) } else { "".to_owned() }, ctx.value());
        Ok(ctx.value().clone())
    });

    namespace.define_pipeline_item("message", |args: Arguments, ctx: Ctx| async move {
        let pipeline: &Pipeline = args.get("pipeline")?;
        let message: &str = args.get("message")?;
        let code: Option<i32> = args.get_optional("code")?;
        match ctx.run_pipeline(pipeline).await {
            Ok(result) => Ok(result),
            Err(mut error) => {
                if let Some(errors) = &error.errors {
                    error.errors = Some(errors.iter().map(|(k, v)| (k.to_owned(), message.to_owned())).collect());
                    if let Some(code) = code {
                        error.code = code as u16;
                    };
                    Err(error)
                } else {
                    error.message = message.to_owned();
                    if let Some(code) = code {
                        error.code = code as u16;
                    };
                    Err(error)
                }
            }
        }
    });


}