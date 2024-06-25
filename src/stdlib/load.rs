use crate::namespace;
use crate::stdlib::admin::load_admin_library;
use crate::stdlib::decorators::enum_decorators::load_enum_decorators;
use crate::stdlib::decorators::enum_member_decorators::load_enum_member_decorators;
use crate::stdlib::decorators::handler_decorators::load_handler_decorators;
use crate::stdlib::decorators::interface_decorators::load_interface_decorators;
use crate::stdlib::decorators::model_decorators::load_model_decorators;
use crate::stdlib::decorators::model_field_decorators::load_model_field_decorators;
use crate::stdlib::decorators::model_property_decorators::load_model_property_decorators;
use crate::stdlib::decorators::model_relation_decorators::load_model_relation_decorators;
use crate::stdlib::middlewares::log_request::load_log_request_middleware;
use crate::stdlib::pipeline_items::logical::load_pipeline_logical_items;
use crate::stdlib::pipeline_items::math::load_pipeline_math_items;
use crate::stdlib::pipeline_items::model_object::load_pipeline_model_object_items;
use crate::stdlib::pipeline_items::number::load_pipeline_number_items;
use crate::stdlib::pipeline_items::string::generation::load_pipeline_string_generation_items;
use crate::stdlib::pipeline_items::string::transform::load_pipeline_string_transform_items;
use crate::stdlib::pipeline_items::string::validation::load_pipeline_string_validation_items;
use crate::stdlib::pipeline_items::value::load_pipeline_value_items;
use crate::stdlib::pipeline_items::array::load_pipeline_array_items;
use crate::stdlib::pipeline_items::bcrypt::load_bcrypt_items;
use crate::stdlib::pipeline_items::vector::load_pipeline_vector_items;
use crate::stdlib::pipeline_items::datetime::load_pipeline_datetime_items;
use crate::stdlib::pipeline_items::debug::load_debug_items;
use crate::stdlib::structs::load_structs;
use crate::stdlib::identity::load_identity_library;
use crate::stdlib::pipeline_items::request::load_pipeline_request_items;

pub fn load(namespace_builder: &namespace::Builder) {
    if !namespace_builder.path().is_empty() {
        panic!("Please load standard library in the main namespace.")
    }
    let std_namespace_builder = namespace_builder.namespace_or_create("std");
    // structs
    load_structs(&std_namespace_builder);
    // decorators
    load_model_decorators(&std_namespace_builder);
    load_model_field_decorators(&std_namespace_builder);
    load_model_relation_decorators(&std_namespace_builder);
    load_model_property_decorators(&std_namespace_builder);
    load_enum_decorators(&std_namespace_builder);
    load_enum_member_decorators(&std_namespace_builder);
    load_interface_decorators(&std_namespace_builder);
    load_handler_decorators(&std_namespace_builder);
    // pipeline items
    load_pipeline_math_items(&std_namespace_builder);
    load_pipeline_number_items(&std_namespace_builder);
    load_pipeline_string_generation_items(&std_namespace_builder);
    load_pipeline_string_transform_items(&std_namespace_builder);
    load_pipeline_string_validation_items(&std_namespace_builder);
    load_pipeline_value_items(&std_namespace_builder);
    load_pipeline_model_object_items(&std_namespace_builder);
    load_pipeline_logical_items(&std_namespace_builder);
    load_pipeline_array_items(&std_namespace_builder);
    load_pipeline_vector_items(&std_namespace_builder);
    load_pipeline_datetime_items(&std_namespace_builder);
    load_pipeline_request_items(&std_namespace_builder);
    load_debug_items(&std_namespace_builder);
    load_bcrypt_items(&std_namespace_builder);
    // middlewares
    load_log_request_middleware(&std_namespace_builder);
    // libraries
    load_identity_library(&std_namespace_builder);
    load_admin_library(&std_namespace_builder);
}