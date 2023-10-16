use crate::arguments::Arguments;
use crate::middleware::middleware::Middleware;
use crate::middleware::next::Next;
use crate::namespace::Namespace;
use crate::request::ctx::Ctx;

pub(in crate::stdlib) fn load_log_request_middleware(namespace: &mut Namespace) {
    namespace.define_middleware("logRequest", |arguments: Arguments| async move {
        let timing: bool = arguments.get("timing")?;
        Ok(Box::leak(Box::new(move |ctx: Ctx, next: &'static dyn Next| async move {
            if timing {
                let start = 1;
            }
            let res = next.call(ctx).await;
            return res;
        })) as &dyn Middleware)
    });
}