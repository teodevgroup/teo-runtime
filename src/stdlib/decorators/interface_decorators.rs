use crate::handler::handler::Method;
use crate::namespace::Namespace;

pub(in crate::stdlib) fn load_interface_decorators(namespace: &mut Namespace) {

    namespace.define_interface_decorator("generateClient", |arguments, interface| {
        let gen: bool = arguments.get("generate")?;
        interface.generate_client = gen;
        Ok(())
    });

    namespace.define_interface_decorator("generateEntity", |arguments, interface| {
        let gen: bool = arguments.get("generate")?;
        interface.generate_entity = gen;
        Ok(())
    });

}