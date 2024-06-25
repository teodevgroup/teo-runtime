use crate::namespace;

pub(in crate::stdlib) fn load_interface_decorators(namespace: &namespace::Builder) {

    namespace.define_interface_decorator("generateClient", |arguments, interface| {
        let gen: bool = arguments.get("generate")?;
        interface.set_generate_client(gen);
        Ok(())
    });

    namespace.define_interface_decorator("generateEntity", |arguments, interface| {
        let gen: bool = arguments.get("generate")?;
        interface.set_generate_entity(gen);
        Ok(())
    });

}