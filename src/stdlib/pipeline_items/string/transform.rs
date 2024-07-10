use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use teo_result::ResultExt;
use pad::{PadStr, Alignment};
use inflector::Inflector;
use regex::Regex;
use crate::value::Value;
use teo_result::Error;
use crate::namespace;

pub(in crate::stdlib) fn load_pipeline_string_transform_items(namespace: &namespace::Builder) {

    namespace.define_pipeline_item("regexReplace", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_ref_into_err_prefix("regexReplace")?;
        let format: &Regex = args.get("format").error_message_prefixed("regexReplace(format)")?;
        let substitute: &str = args.get("substitute").error_message_prefixed("regexReplace(substitute)")?;
        Ok(Value::from(format.replace(input, substitute).to_string()))
    });

    namespace.define_pipeline_item("toWordCase", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_ref_into_err_prefix("toWordCase")?;
        Ok(Value::from(input.to_word_case()))
    });

    namespace.define_pipeline_item("toLowerCase", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_ref_into_err_prefix("toLowerCase")?;
        Ok(Value::from(input.to_lowercase()))
    });

    namespace.define_pipeline_item("toUpperCase", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_ref_into_err_prefix("toUpperCase")?;
        Ok(Value::from(input.to_uppercase()))
    });

    namespace.define_pipeline_item("toTitleCase", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_ref_into_err_prefix("toTitleCase")?;
        Ok(Value::from(input.to_title_case()))
    });

    namespace.define_pipeline_item("toSentenceCase", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_ref_into_err_prefix("toSentenceCase")?;
        Ok(Value::from(input.to_sentence_case()))
    });

    namespace.define_pipeline_item("trim", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_ref_into_err_prefix("trim")?;
        Ok(Value::from(input.trim()))
    });

    namespace.define_pipeline_item("split", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_ref_into_err_prefix("split")?;
        let arg_object: Value = ctx.resolve_pipeline_with_err_prefix(
            args.get_object("separator").error_message_prefixed("split(separator)")?,
            "split(separator)",
        ).await?;
        let separator: &str = arg_object.try_ref_into_err_prefix("split(separator)")?;
        Ok(Value::from(Value::Array(input.split(separator).map(|input| Value::String(input.to_string())).collect::<Vec<Value>>())))
    });

    namespace.define_pipeline_item("ellipsis", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_ref_into_err_prefix("ellipsis")?;
        let ellipsis: &str = args.get("ellipsis").error_message_prefixed("ellipsis(ellipsis)")?;
        let width_object: Value = ctx.resolve_pipeline_with_err_prefix(
            args.get_object("width").error_message_prefixed("ellipsis(width)")?,
            "ellipsis(width)",
        ).await?;
        let width: i32 = width_object.try_into_err_prefix("ellipsis(width)")?;
        if input.len() <= width as usize {
            Ok(ctx.value().clone())
        } else {
            Ok(Value::from(input.chars().take(width.try_into().unwrap()).collect::<String>() + ellipsis))
        }
    });

    namespace.define_pipeline_item("padStart", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_ref_into_err_prefix("padStart")?;
        let width_object: Value = ctx.resolve_pipeline_with_err_prefix(
            args.get_object("width").error_message_prefixed("padStart(width)")?,
            "padStart(width)",
        ).await?;
        let width: usize = width_object.try_ref_into_err_prefix("padStart(width)")?;
        let char_str: &str = args.get("char").error_message_prefixed("padStart(char)")?;
        if char_str.len() != 1 {
            Err(Error::new("padStart(char): char is not 1 length string"))?
        }
        let char = char_str.chars().nth(0).unwrap();
        Ok(Value::from(input.pad(width, char, Alignment::Right, false)))
    });

    namespace.define_pipeline_item("padEnd", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_ref_into_err_prefix("padEnd")?;
        let width_object: Value = ctx.resolve_pipeline_with_err_prefix(
            args.get_object("width").error_message_prefixed("padEnd(width)")?,
            "padEnd(width)",
        ).await?;
        let width: usize = width_object.try_ref_into_err_prefix("padEnd(width")?;
        let char_str: &str = args.get("char").error_message_prefixed("padEnd(char)")?;
        if char_str.len() != 1{
            Err(Error::new("padEnd(char): char is not 1 length string"))?
        }
        let char = char_str.chars().nth(0).unwrap();
        Ok(Value::from(input.pad(width, char, Alignment::Left, false)))
    });

}