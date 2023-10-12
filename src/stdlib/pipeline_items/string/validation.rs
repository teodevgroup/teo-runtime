use regex::{Regex};
use crate::arguments::Arguments;
use crate::namespace::Namespace;
use crate::pipeline::Ctx;
use once_cell::sync::Lazy;
use crate::error::Error;
use teo_teon::Value;
use crate::result::{Result, ResultExt};

pub(super) static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b$").unwrap()
});

pub(super) static CORLOR_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Fa-f0-9]{6}$").unwrap()
});

pub(super) static SECURE_REGEX: [Lazy<Regex>; 4] = [
        Lazy::new(|| Regex::new(r#"[A-Z]"#).unwrap()),
        Lazy::new(|| Regex::new(r#"[a-z]"#).unwrap()),
        Lazy::new(|| Regex::new(r#"\d"#).unwrap()),
        Lazy::new(|| Regex::new(r#"[!@#$&*`~()\-_+=\[\]{}:;'",<>.?\\|/]"#).unwrap()),
];


pub(in crate::stdlib) fn load_pipeline_string_validation_items(namespace: &mut Namespace) {

    namespace.define_pipeline_item("isEmail", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("isEmail")?;
        if !EMAIL_REGEX.is_match(input) {
            Err(Error::new("input is not email"))?
        }
        Ok(ctx.value().clone())
    });

    namespace.define_pipeline_item("isHexColor", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("isHexColor")?;
        if !CORLOR_REGEX.is_match(input) {
            Err(Error::new("input is not hex color"))?
        }
        Ok(ctx.value().clone())
    });

    namespace.define_pipeline_item("isSecurePassword", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("isSecurePassword")?;
        for regex in &SECURE_REGEX{
            if regex.is_match(input) {
                Err(Error::new("input is not secure password"))?
            }
        }
        Ok(ctx.value().clone())
    });

    namespace.define_pipeline_item("isNumeric", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("isNumeric")?;
        for c in input.chars(){
            if !c.is_numeric(){
                Err(Error::new("input is not numeric"))?
            }
        }
        Ok(ctx.value().clone())
    });

    namespace.define_pipeline_item("isAlphanumeric", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("isAlphanumeric")?;
        for c in input.chars(){
            if !c.is_alphanumeric(){
                Err(Error::new("input is not alphanumeric"))?
            }
        }
        Ok(ctx.value().clone())
    });

    namespace.define_pipeline_item("isAlphabetic", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("isAlphabetic")?;
        for c in input.chars(){
            if !c.is_alphabetic(){
                Err(Error::new("input is not alphabetic"))?
            }
        }
        Ok(ctx.value().clone())
    });

    namespace.define_pipeline_item("isSuffixOf", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("isSuffixOf")?;
        let arg_object = ctx.resolve_pipeline(
            args.get_object("value").err_prefix("isSuffixOf")?,
            "isSuffixOf",
        ).await?;
        let arg: &Value = arg_object.try_into_err_prefix("isSuffixOf")?;

        if !arg.to_string().ends_with(input) {
            Err(Error::new("input is not alphabetic"))?
        }
        Ok(ctx.value().clone())
    });

}