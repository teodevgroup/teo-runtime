use hyper::Method;
use crate::namespace;

pub(in crate::stdlib) fn load_handler_decorators(namespace: &namespace::Builder) {

    namespace.define_handler_decorator("map", |arguments, handler| {
        let method: Option<Method> = arguments.get_optional("method")?;
        let path: Option<String> = arguments.get_optional("path")?;
        let ignore_prefix: Option<bool> = arguments.get_optional("ignorePrefix")?;
        let interface: Option<String> = arguments.get_optional("interface")?;
        if let Some(method) = method {
            handler.set_method(method);
        }
        handler.set_url(path);
        handler.set_ignore_prefix(ignore_prefix.unwrap_or(false));
        handler.set_interface(interface);
        Ok(())
    });
}