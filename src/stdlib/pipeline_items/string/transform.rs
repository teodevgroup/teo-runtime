use crate::namespace::Namespace;
use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use crate::result::ResultExt;
use crate::object::Object;
use inflector::Inflector;
use regex::Regex;

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


}