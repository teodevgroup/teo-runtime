use crate::arguments::Arguments;
use crate::namespace;
use crate::pipeline::{Ctx, Pipeline};
use crate::pipeline::item::item_impl::ItemImpl;

pub(in crate::stdlib) fn load_debug_items(namespace: &namespace::Builder) {

    namespace.define_pipeline_item("print", |args: Arguments| {
        let label: Option<String> = args.get_optional("label")?;
        Ok(ItemImpl::new(move |ctx: Ctx| {
            let label = label.clone();
            async move {
                println!("{}{}", if let Some(label) = &label { format!("{}: ", label) } else { "".to_owned() }, ctx.value());
                Ok(ctx.value().clone())
            }
        }))
    });

    namespace.define_pipeline_item("message", |args: Arguments| {
        let pipeline: Pipeline = args.get("pipeline")?;
        let message: String = args.get("message")?;
        let code: Option<i32> = args.get_optional("code")?;
        Ok(ItemImpl::new(move |ctx: Ctx| {
            let pipeline = pipeline.clone();
            let message = message.clone();
            let code = code;
            async move {
                match ctx.run_pipeline(&pipeline).await {
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
            }
        }))
    });
}