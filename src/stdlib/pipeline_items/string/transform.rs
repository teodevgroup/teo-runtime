use crate::namespace::Namespace;
use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use crate::error::Error;
use crate::result::ResultExt;

use crate::object::Object;
use inflector::cases::wordcase::to_word_case;
use inflector::Inflector;
use teo_teon::Value;


pub(in crate::stdlib) fn load_pipeline_string_transform_items(namespace: &mut Namespace) {


    // namespace.define_pipeline_item("regexReplace", |args: Arguments, ctx: Ctx| async move {
    //     let input: &str = ctx.value().try_into_err_prefix("regexReplace")?;
    //     let arg_object = &ctx.resolve_pipeline(
    //         args.get_object("value").err_prefix("regexReplace(value)")?,
    //         "regexReplace(value)",
    //     ).await?;
    //     let arg: &str = arg_object.try_into_err_prefix("regexReplace(value)")?;
    //     let regex = arg.as_regexp().unwrap();
    //     Ok(Object::from(Value::String(regex.replace(input, arg).to_string())))
    // });


    namespace.define_pipeline_item("toWordCase", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("toWordCase")?;
        Ok(Object::from(Value::String(to_word_case(input))))
    });

    namespace.define_pipeline_item("toLowerCase", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("toLowerCase")?;
        Ok(Object::from(Value::String(input.to_lowercase())))
    });

    namespace.define_pipeline_item("toUpperCase", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("toUpperCase")?;
        Ok(Object::from(Value::String(input.to_uppercase())))
    });

    namespace.define_pipeline_item("toTitleCase", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("toTitleCase")?;
        Ok(Object::from(Value::String(input.to_title_case())))
    });

    namespace.define_pipeline_item("toSentenceCase", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("toSentenceCase")?;
        Ok(Object::from(Value::String(input.to_sentence_case())))
    });


}