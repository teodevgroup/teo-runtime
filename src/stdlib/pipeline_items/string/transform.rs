use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use teo_result::ResultExt;
use pad::{PadStr, Alignment};
use inflector::Inflector;
use regex::Regex;
use crate::value::Value;
use teo_result::Error;
use crate::namespace;
use crate::pipeline::item::item_impl::ItemImpl;

pub(in crate::stdlib) fn load_pipeline_string_transform_items(namespace: &namespace::Builder) {

    namespace.define_pipeline_item("regexReplace", |args: Arguments| {
        let format: Regex = args.get("format").error_message_prefixed("regexReplace(format)")?;
        let substitute: String = args.get("substitute").error_message_prefixed("regexReplace(substitute)")?;
        Ok(ItemImpl::new(move |ctx: Ctx| {
            let format = format.clone();
            let substitute = substitute.clone();
            async move {
                let input: &str = ctx.value().try_ref_into_err_prefix("regexReplace")?;
                Ok(Value::from(format.replace(input, substitute).to_string()))
            }
        }))
    });

    namespace.define_pipeline_item("toWordCase", |args: Arguments| {
        Ok(ItemImpl::new(|ctx: Ctx| async move {
            let input: &str = ctx.value().try_ref_into_err_prefix("toWordCase")?;
            Ok(Value::from(input.to_word_case()))
        }))
    });

    namespace.define_pipeline_item("toLowerCase", |args: Arguments| {
        Ok(ItemImpl::new(|ctx: Ctx| async move {
            let input: &str = ctx.value().try_ref_into_err_prefix("toLowerCase")?;
            Ok(Value::from(input.to_lowercase()))
        }))
    });

    namespace.define_pipeline_item("toUpperCase", |args: Arguments| {
        Ok(ItemImpl::new(|ctx: Ctx| async move {
            let input: &str = ctx.value().try_ref_into_err_prefix("toUpperCase")?;
            Ok(Value::from(input.to_uppercase()))
        }))
    });

    namespace.define_pipeline_item("toTitleCase", |args: Arguments| {
        Ok(ItemImpl::new(|ctx: Ctx| async move {
            let input: &str = ctx.value().try_ref_into_err_prefix("toTitleCase")?;
            Ok(Value::from(input.to_title_case()))
        }))
    });

    namespace.define_pipeline_item("toSentenceCase", |args: Arguments| {
        Ok(ItemImpl::new(|ctx: Ctx| async move {
            let input: &str = ctx.value().try_ref_into_err_prefix("toSentenceCase")?;
            Ok(Value::from(input.to_sentence_case()))
        }))
    });

    namespace.define_pipeline_item("trim", |args: Arguments| {
        Ok(ItemImpl::new(|ctx: Ctx| async move {
            let input: &str = ctx.value().try_ref_into_err_prefix("trim")?;
            Ok(Value::from(input.trim()))
        }))
    });

    namespace.define_pipeline_item("split", |args: Arguments| {
        let separator = args.get_value("separator").error_message_prefixed("split(separator)")?;
        Ok(ItemImpl::new(move |ctx: Ctx| {
            let separator = separator.clone();
            async move {
                let input: &str = ctx.value().try_ref_into_err_prefix("split")?;
                let arg_object: Value = ctx.resolve_pipeline_with_err_prefix(
                    separator.clone(),
                    "split(separator)",
                ).await?;
                let separator: &str = arg_object.try_ref_into_err_prefix("split(separator)")?;
                Ok(Value::from(Value::Array(input.split(separator).map(|input| Value::String(input.to_string())).collect::<Vec<Value>>())))
            }
        }))

    });

    namespace.define_pipeline_item("ellipsis", |args: Arguments| {
        let ellipsis: String = args.get("ellipsis").error_message_prefixed("ellipsis(ellipsis)")?;
        let width: Value = args.get_value("width").error_message_prefixed("ellipsis(width)")?;
        Ok(ItemImpl::new(move |ctx: Ctx| {
            let ellipsis = ellipsis.clone();
            let width = width.clone();
            async move {
                let input: &str = ctx.value().try_ref_into_err_prefix("ellipsis")?;
                let width_object: Value = ctx.resolve_pipeline_with_err_prefix(
                    width.clone(),
                    "ellipsis(width)",
                ).await?;
                let width: i32 = width_object.try_into_err_prefix("ellipsis(width)")?;
                if input.len() <= width as usize {
                    Ok(ctx.value().clone())
                } else {
                    Ok(Value::from(input.chars().take(width.try_into().unwrap()).collect::<String>() + &ellipsis))
                }
            }
        }))
    });

    namespace.define_pipeline_item("padStart", |args: Arguments| {
        let char_str: String = args.get("char").error_message_prefixed("padStart(char)")?;
        if char_str.len() != 1 {
            Err(Error::new("padStart(char): char is not 1 length string"))?
        }
        let width = args.get_value("width").error_message_prefixed("padStart(width)")?;
        Ok(ItemImpl::new(move |ctx: Ctx| {
            let char_str = char_str.clone();
            let width = width.clone();
            async move {
                let input: &str = ctx.value().try_ref_into_err_prefix("padStart")?;
                let width_object: Value = ctx.resolve_pipeline_with_err_prefix(
                    width.clone(),
                    "padStart(width)",
                ).await?;
                let width: usize = width_object.try_ref_into_err_prefix("padStart(width)")?;

                let char = char_str.chars().nth(0).unwrap();
                Ok(Value::from(input.pad(width, char, Alignment::Right, false)))
            }
        }))
    });

    namespace.define_pipeline_item("padEnd", |args: Arguments| {
        let char_str: String = args.get("char").error_message_prefixed("padEnd(char)")?;
        if char_str.len() != 1 {
            Err(Error::new("padEnd(char): char is not 1 length string"))?
        }
        let width = args.get_value("width").error_message_prefixed("padEnd(width)")?;
        Ok(ItemImpl::new(move |ctx: Ctx| {
            let char_str = char_str.clone();
            let width = width.clone();
            async move {
                let input: &str = ctx.value().try_ref_into_err_prefix("padEnd")?;
                let width_object: Value = ctx.resolve_pipeline_with_err_prefix(
                    width.clone(),
                    "padEnd(width)",
                ).await?;
                let width: usize = width_object.try_ref_into_err_prefix("padEnd(width")?;
                let char = char_str.chars().nth(0).unwrap();
                Ok(Value::from(input.pad(width, char, Alignment::Left, false)))
            }
        }))

    });

}