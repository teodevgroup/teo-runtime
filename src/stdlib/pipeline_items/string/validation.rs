use regex::Regex;
use crate::arguments::Arguments;
use crate::namespace::Namespace;
use crate::pipeline::Ctx;
use once_cell::sync::Lazy;
use crate::error::Error;

pub(super) static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b$").unwrap()
});

pub(in crate::stdlib) fn load_pipeline_string_validation_items(namespace: &mut Namespace) {

    namespace.define_pipeline_item("isEmail", |args: Arguments, ctx: Ctx| async move {
        let input: &str = ctx.value().try_into_err_prefix("isEmail")?;
        if !EMAIL_REGEX.is_match(input) {
            Err(Error::new("input is not email"))?
        }
        Ok(ctx.value().clone())
    });
}