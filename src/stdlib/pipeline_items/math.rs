use teo_teon::Value;
use crate::arguments::Arguments;
use crate::namespace::Namespace;
use crate::object::Object;
use crate::pipeline::Ctx;
use crate::result::ResultExt;

pub(in crate::stdlib) fn load_pipeline_math_items(namespace: &mut Namespace) {
    namespace.define_pipeline_item("add", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into().err_prefix("add")?;
        let arg_object = &ctx.resolve_pipeline(
            args.get_object("value").err_prefix("add(value)")?,
            "add(value)",
        ).await?;
        let arg: &Value = arg_object.try_into().err_prefix("add(value)")?;
        Ok(Object::from((input + arg).err_prefix("add")?))
    })
}
