use crate::namespace::Namespace;
use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use crate::result::ResultExt;
use crate::object::Object;
use cuid2::create_id;
use random_string::generate;
use cuid::{cuid, slug};
use uuid::Uuid;

pub(in crate::stdlib) fn load_pipeline_string_generation_items(namespace: &mut Namespace) {

    namespace.define_pipeline_item("cuid", |args: Arguments, ctx: Ctx| async move {
        Ok(Object::from(cuid().unwrap()))
    });

    namespace.define_pipeline_item("cuid2", |args: Arguments, ctx: Ctx| async move {
        Ok(Object::from(create_id()))
    });

    namespace.define_pipeline_item("slug", |args: Arguments, ctx: Ctx| async move {
        Ok(Object::from(slug().unwrap()))
    });

    namespace.define_pipeline_item("uuid", |args: Arguments, ctx: Ctx| async move {
        Ok(Object::from(Uuid::new_v4().to_string()))
    });

    namespace.define_pipeline_item("randomDigits", |args: Arguments, ctx: Ctx| async move {
        let len: usize = args.get("len").err_prefix("randomDigits")?;
        Ok(Object::from(generate(len, "1234567890")))
    });

}