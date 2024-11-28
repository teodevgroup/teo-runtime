use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use teo_result::ResultExt;
use chrono::{DateTime, Duration, Utc};
use crate::namespace;
use crate::value::Value;

pub(in crate::stdlib) fn load_pipeline_datetime_items(namespace: &namespace::Builder) {

    namespace.define_pipeline_item("now", |_args: Arguments| {
        Ok(|_ctx: Ctx| async {
            Ok(Value::from(Utc::now()))
        })
    });

    namespace.define_pipeline_item("today", |args: Arguments| {
        let tz = args.get_value("tz").error_message_prefixed("today(tz)")?;
        Ok(move |ctx: Ctx| {
            let tz = tz.clone();
            async move {
                let arg_object: Value = ctx.resolve_pipeline_with_err_prefix(
                    tz.clone(),
                    "today(tz)",
                ).await?;
                let arg: i32 = arg_object.try_into_err_prefix("today(tz)")?;
                let now = Utc::now();
                let calculated = now + Duration::hours(arg as i64);
                Ok(Value::from(calculated.date_naive()))
            }
        })
    });

    namespace.define_pipeline_item("toDate", |args: Arguments| {
        let tz = args.get_value("tz").error_message_prefixed("toDate(tz)")?;
        Ok(move |ctx: Ctx| {
            let tz = tz.clone();
            async move {
                let datetime: &DateTime<Utc> = ctx.value().try_ref_into_err_prefix("toDate")?;
                let arg_object: Value = ctx.resolve_pipeline_with_err_prefix(
                    tz.clone(),
                    "toDate(tz)",
                ).await?;
                let arg: i32 = arg_object.try_into_err_prefix("toDate(tz)")?;
                let calculated = *datetime + Duration::hours(arg as i64);
                Ok(Value::from(calculated.date_naive()))
            }
        })
    });
}