use teo_teon::Value;
use crate::arguments::Arguments;
use crate::namespace::Namespace;
use crate::object::Object;
use crate::pipeline::Ctx;
use crate::result::ResultExt;

pub(in crate::stdlib) fn load_pipeline_math_items(namespace: &mut Namespace) {
    namespace.define_pipeline_item("add", |args: Arguments, ctx: Ctx| async move {
        //let input = ctx.value().as_teon_or_err("add: input is not teon")?;
        let arg: &Value = args.get("value").err_prefix("add")?;
        let arg: &str = args.get("value").err_prefix("add")?;
        Ok(ctx)
        // ctx.resolve()
        // let argument = self.argument.resolve(ctx.clone()).await?;
        // Ok(ctx.with_value_result(ctx.get_value() + argument)?)
    })
}