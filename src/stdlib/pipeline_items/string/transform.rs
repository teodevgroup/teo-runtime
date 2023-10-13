use crate::namespace::Namespace;
use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use crate::error::Error;
use crate::result::ResultExt;

use crate::object::Object;
use inflector::cases::wordcase::to_word_case;
use teo_teon::Value;


pub(in crate::stdlib) fn load_pipeline_string_transform_items(namespace: &mut Namespace) {

    // namespace.define_pipeline_item("hasPrefix", |args: Arguments, ctx: Ctx| async move {
    //     let input: &str = ctx.value().try_into_err_prefix("hasPrefix")?;
    //     Ok(
    //         // str => 
    //         Object::from(Value::String(to_word_case(s)))
    //     )
    // });
}