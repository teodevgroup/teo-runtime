use crate::namespace::Namespace;

pub(in crate::stdlib) fn load_pipeline_math_items(namespace: &mut Namespace) {
    namespace.define_pipeline_item("add", |args, ctx| async {
        Ok(ctx)
    })
}