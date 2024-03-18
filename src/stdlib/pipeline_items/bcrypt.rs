use crate::namespace::Namespace;
use crate::arguments::Arguments;
use crate::pipeline::{Ctx, Pipeline};
use crate::object::Object;
use bcrypt::{DEFAULT_COST, hash, verify};
use teo_result::{Error, ResultExt};

pub(in crate::stdlib) fn load_bcrypt_items(namespace: &mut Namespace) {

    let mut bcrypt_namespace = namespace.namespace_mut_or_create("bcrypt");

    bcrypt_namespace.define_pipeline_item("salt", |_: Arguments, ctx: Ctx| async move {
        let value: &str = ctx.value().try_into_err_message("salt: value is not string")?;
        Ok(Object::from(hash(value, DEFAULT_COST).unwrap()))
    });

    bcrypt_namespace.define_pipeline_item("verify", |args: Arguments, ctx: Ctx| async move {
        let value: &str = ctx.value().try_into_err_message("verify: value is not string")?;
        let pipeline: Pipeline = args.get("pipeline").error_message_prefixed("verify")?;
        let hash: String = ctx.run_pipeline(&pipeline).await?.try_into_err_prefix("verify")?;
        if verify(value, &hash).unwrap() {
            Ok(ctx.value().clone())
        } else {
            Err(Error::new_with_code("verify: value doesn't match", 401))
        }
    });
}