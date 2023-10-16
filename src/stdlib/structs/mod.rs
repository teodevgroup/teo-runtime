use maplit::btreemap;
use teo_teon::Value;
use crate::namespace::Namespace;
use crate::arguments::Arguments;
use crate::r#struct;
use crate::object::Object;
use crate::utils::next_path;

pub(in crate::stdlib) fn load_structs(namespace: &mut Namespace) {

    namespace.define_struct("EnvVars", |path, env_vars| {
        env_vars.define_static_function("new", move |_arguments: Arguments| async move {
            Ok(Object::from(r#struct::Object::new(path.clone(), btreemap! {})))
        });
        env_vars.define_function("subscript", move |_this: Object, arguments: Arguments| async move {
            let key: &str = arguments.get("key")?;
            if let Ok(retval) = std::env::var(key) {
                Ok(Object::from(retval))
            } else {
                Ok(Object::from(Value::Null))
            }
        });
    });
}