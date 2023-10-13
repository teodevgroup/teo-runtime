use crate::namespace::Namespace;
use crate::stdlib::decorators::enum_decorators::load_enum_decorators;
use crate::stdlib::decorators::enum_member_decorators::load_enum_member_decorators;
use crate::stdlib::decorators::model_decorators::load_model_decorators;
use crate::stdlib::decorators::model_field_decorators::load_model_field_decorators;
use crate::stdlib::decorators::model_property_decorators::load_model_property_decorators;
use crate::stdlib::decorators::model_relation_decorators::load_model_relation_decorators;
use crate::stdlib::pipeline_items::math::load_pipeline_math_items;
use crate::stdlib::pipeline_items::model_object::load_pipeline_model_object_items;
use crate::stdlib::pipeline_items::number::load_pipeline_number_items;
use crate::stdlib::pipeline_items::string::generation::load_pipeline_string_generation_items;
use crate::stdlib::pipeline_items::string::transform::load_pipeline_string_transform_items;
use crate::stdlib::pipeline_items::string::validation::load_pipeline_string_validation_items;
use crate::stdlib::pipeline_items::value::load_pipeline_value_items;

pub(crate) fn load(namespace: &mut Namespace) {
    if !namespace.path.is_empty() {
        panic!("Please load standard library in the main namespace.")
    }
    let std_namespace = namespace.namespace_mut_or_create("std");
    // decorators
    load_model_decorators(std_namespace);
    load_model_field_decorators(std_namespace);
    load_model_relation_decorators(std_namespace);
    load_model_property_decorators(std_namespace);
    load_enum_decorators(std_namespace);
    load_enum_member_decorators(std_namespace);
    // pipeline items
    load_pipeline_math_items(std_namespace);
    load_pipeline_number_items(std_namespace);
    load_pipeline_string_generation_items(std_namespace);
    load_pipeline_string_transform_items(std_namespace);
    load_pipeline_string_validation_items(std_namespace);
    load_pipeline_value_items(std_namespace);
}