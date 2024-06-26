use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use teo_result::ResultExt;
use cuid2::create_id;
use random_string::generate;
use cuid::{cuid, slug};
use uuid::Uuid;
use crate::namespace;
use crate::value::Value;

pub(in crate::stdlib) fn load_pipeline_string_generation_items(namespace: &namespace::Builder) {

    namespace.define_pipeline_item("cuid", |args: Arguments, ctx: Ctx| async move {
        Ok(Value::from(cuid().unwrap()))
    });

    namespace.define_pipeline_item("cuid2", |args: Arguments, ctx: Ctx| async move {
        Ok(Value::from(create_id()))
    });

    namespace.define_pipeline_item("slug", |args: Arguments, ctx: Ctx| async move {
        Ok(Value::from(slug().unwrap()))
    });

    namespace.define_pipeline_item("uuid", |args: Arguments, ctx: Ctx| async move {
        Ok(Value::from(Uuid::new_v4().to_string()))
    });

    namespace.define_pipeline_item("randomDigits", |args: Arguments, ctx: Ctx| async move {
        let len: usize = args.get("len").error_message_prefixed("randomDigits")?;
        Ok(Value::from(generate(len, "1234567890")))
    });

}