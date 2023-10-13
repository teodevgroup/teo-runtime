use crate::namespace::Namespace;
use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use crate::result::ResultExt;
use crate::object::Object;
use pad::{PadStr, Alignment};
use inflector::Inflector;
use regex::Regex;
use teo_teon::Value;

pub(in crate::stdlib) fn load_pipeline_string_transform_items(namespace: &mut Namespace) {

    namespace.define_pipeline_item("regexReplace", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("regexReplace")?;
        let format: &Regex = args.get("format").err_prefix("regexReplace(format)")?;
        let substitute: &str = args.get("substitute").err_prefix("regexReplace(substitute)")?;
        Ok(Object::from(format.replace(input, substitute).to_string()))
    });

    namespace.define_pipeline_item("toWordCase", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("toWordCase")?;
        Ok(Object::from(input.to_word_case()))
    });

    namespace.define_pipeline_item("toLowerCase", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("toLowerCase")?;
        Ok(Object::from(input.to_lowercase()))
    });

    namespace.define_pipeline_item("toUpperCase", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("toUpperCase")?;
        Ok(Object::from(input.to_uppercase()))
    });

    namespace.define_pipeline_item("toTitleCase", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("toTitleCase")?;
        Ok(Object::from(input.to_title_case()))
    });

    namespace.define_pipeline_item("toSentenceCase", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("toSentenceCase")?;
        Ok(Object::from(input.to_sentence_case()))
    });

    namespace.define_pipeline_item("trim", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("trim")?;
        Ok(Object::from(input.trim().to_owned()))
    });

    namespace.define_pipeline_item("split", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("split")?;
        let arg_object = &ctx.resolve_pipeline(
            args.get_object("value").err_prefix("split(value)")?,
            "split(value)",
        ).await?;
        let arg: &str = arg_object.try_into_err_prefix("split(value)")?;
        Ok(Object::from(Value::Array(input.split(arg).map(|input| Value::String(input.to_string())).collect::<Vec<Value>>())))
    });

    namespace.define_pipeline_item("ellipsis", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("ellipsis")?;
        let ellipsis: &str = args.get("ellipsis").err_prefix("ellipsis")?;
        let width: i32 = args.get("width").err_prefix("ellipsis")?;
        if input.len() <= width.try_into().unwrap() {
            Ok(Object::from(input))
        } else {
            Ok( Object::from(input.chars().take(width.try_into().unwrap()).collect::<String>() + ellipsis) )
        }
    });

    // namespace.define_pipeline_item("padStart", |args: Arguments, ctx: Ctx| async move {
    //     let input: &str = ctx.value().try_into_err_prefix("padStart")?;
    //     let width: usize = args.get("width").err_prefix("padStart")? as usize;
    //     let char = args.get("char").err_prefix("padStart")?;
    //     Ok( Object::from(input.pad(width, char, Alignment::Right, false)))
    // });

    // namespace.define_pipeline_item("padEnd", |args: Arguments, ctx: Ctx| async move {
    //     let input: &str = ctx.value().try_into_err_prefix("padEnd")?;
    //     let width: usize = args.get("width").err_prefix("padEnd")? as usize;
    //     let s_char = args.get("char").err_prefix("padEnd")?;
    //     Ok( Object::from(input.pad(width, s_char, Alignment::Left, false)))
    // });

}