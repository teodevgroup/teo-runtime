use crate::handler::handler::Method;
use crate::namespace::Namespace;

pub(in crate::stdlib) fn load_handler_decorators(namespace: &mut Namespace) {

    namespace.define_handler_decorator("map", |arguments, handler| {
        let method: Option<Method> = arguments.get_optional("method")?;
        let path: Option<String> = arguments.get_optional("path")?;
        let ignore_prefix: Option<bool> = arguments.get_optional("ignorePrefix")?;
        if let Some(method) = method {
            handler.method = method;
        }
        handler.url = path;
        handler.ignore_prefix = ignore_prefix.unwrap_or(false);
        Ok(())
    });
}