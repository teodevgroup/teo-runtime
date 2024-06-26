use crate::value::Value;
use crate::arguments::Arguments;
use crate::namespace;
use crate::pipeline::Ctx;

pub(in crate::stdlib) fn load_pipeline_request_items(namespace: &namespace::Builder) {
    namespace.define_pipeline_item("account", |_args: Arguments, ctx: Ctx| async move {
        let Some(request_ctx) = ctx.request_ctx() else {
            return Ok(Value::from(Value::Null));
        };
        let binding = request_ctx.data();
        let object: Option<&Value> = binding.get("account");
        let Some(object) = object else {
            return Ok(Value::from(Value::Null));
        };
        Ok(object.clone())
    });
}