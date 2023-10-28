use crate::handler::handler::Method;
use crate::namespace::Namespace;

pub(in crate::stdlib) fn load_handler_decorators(namespace: &mut Namespace) {

    namespace.define_handler_decorator("method", |arguments, handler| {
        let method: Method = arguments.get("method")?;
        handler.method = method;
        Ok(())
    });

    namespace.define_handler_decorator("path", |arguments, handler| {
        let path: String = arguments.get("path")?;
        let ignore_namespace_prefix: Option<bool> = arguments.get_optional("ignoreNamespacePrefix")?;
        handler.url = Some(path);
        handler.ignore_namespace = ignore_namespace_prefix.unwrap_or(false);
        Ok(())
    });
}