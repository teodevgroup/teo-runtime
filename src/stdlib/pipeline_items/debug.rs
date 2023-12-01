use crate::namespace::Namespace;
use crate::arguments::Arguments;
use crate::pipeline::Ctx;

pub(in crate::stdlib) fn load_debug_items(namespace: &mut Namespace) {

    namespace.define_pipeline_item("print", |args: Arguments, ctx: Ctx| async move {
        let label: Option<&str> = args.get_optional("label")?;
        println!("{}{}", if let Some(label) = label { format!("{}: ", label) } else { "".to_owned() }, ctx.value());
        Ok(ctx.value().clone())
    });

}