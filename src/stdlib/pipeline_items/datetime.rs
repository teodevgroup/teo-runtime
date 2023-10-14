use crate::namespace::Namespace;
use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use crate::result::ResultExt;
use crate::object::Object;
use crate::error::Error;
use teo_teon::Value;
use chrono::{Duration,Utc};

pub(in crate::stdlib) fn load_pipeline_datetime_items(namespace: &mut Namespace) {

    namespace.define_pipeline_item("now", |args: Arguments, ctx: Ctx| async move {
        Ok(Object::from(Utc::now()))
    });

    namespace.define_pipeline_item("today", |args: Arguments, ctx: Ctx| async move {
        let input: &Value = ctx.value().try_into_err_prefix("today")?;
        let arg_object = ctx.resolve_pipeline(
            args.get_object("tz").err_prefix("today(tz)")?,
            "today(tz)",
        ).await?;
        let arg: i32 = arg_object.try_into_err_prefix("today(tz)")?;
        let now = Utc::now();
        let calculated = now + Duration::hours(arg.into());
        Ok(Object::from(calculated.date_naive()))
    });

    // namespace.define_pipeline_item("toDate", |args: Arguments, ctx: Ctx| async move {
    //     let datetime: &DateTime<Utc> = ctx.value().try_into_err_prefix("toDate")?;
    //     let arg_object = ctx.resolve_pipeline(
    //         args.get_object("tz").err_prefix("toDate(tz)")?,
    //         "toDate(tz)",
    //     ).await?;
    //     let arg: i32 = arg_object.try_into_err_prefix("toDate(tz)")?;
    //     let calculated = datetime.clone() + Duration::hours(arg.into());
    //     Ok(Object::from(calculated.date_naive()))
    // });

}