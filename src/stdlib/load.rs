use crate::namespace::Namespace;
use crate::stdlib::decorators::enum_decorators::load_enum_decorators;
use crate::stdlib::decorators::enum_member_decorators::load_enum_member_decorators;
use crate::stdlib::decorators::model_decorators::load_model_decorators;
use crate::stdlib::decorators::model_field_decorators::load_model_field_decorators;
use crate::stdlib::decorators::model_property_decorators::load_model_property_decorators;
use crate::stdlib::decorators::model_relation_decorators::load_model_relation_decorators;

pub(crate) fn load(namespace: &mut Namespace) {
    if !namespace.path.is_empty() {
        panic!("Please load standard library in the main namespace.")
    }
    let std_namespace = namespace.namespace_mut_or_create("std");
    load_model_decorators(std_namespace);
    load_model_field_decorators(std_namespace);
    load_model_relation_decorators(std_namespace);
    load_model_property_decorators(std_namespace);
    load_enum_decorators(std_namespace);
    load_enum_member_decorators(std_namespace);
}