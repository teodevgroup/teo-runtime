use teo_result::Result;
use crate::value::Value;
use crate::arguments::Arguments;
use crate::namespace;
use crate::pipeline::Ctx;
use crate::pipeline::item::item_impl::ItemImpl;

pub(in crate::stdlib) fn load_pipeline_request_items(namespace: &namespace::Builder) {
    namespace.define_pipeline_item("account", |_args: Arguments| {
        Ok(ItemImpl::new(|ctx: Ctx| async move {
            let Some(request) = ctx.request() else {
                return Ok(Value::from(Value::Null));
            };
            let binding = request.local_values();
            let object: Result<Value> = binding.get("account");
            let Ok(object) = object else {
                return Ok(Value::from(Value::Null));
            };
            Ok(object)
        }))
    });
}