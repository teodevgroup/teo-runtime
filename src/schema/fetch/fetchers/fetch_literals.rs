use std::hint::unreachable_unchecked;
use indexmap::indexmap;
use teo_parser::ast::literals::{TupleLiteral, ArrayLiteral, DictionaryLiteral, EnumVariantLiteral};
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::r#type::Type;
use teo_parser::traits::named_identifiable::NamedIdentifiable;
use teo_parser::traits::resolved::Resolve;
use teo_result::Result;
use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::Value;
use crate::namespace::Namespace;
use crate::object::Object;
use crate::schema::fetch::fetch_argument_list::fetch_argument_list;
use crate::schema::fetch::fetch_expression::{fetch_dictionary_key_expression, fetch_expression};

pub fn fetch_tuple_literal<I>(tuple_literal: &TupleLiteral, schema: &Schema, info_provider: &I, expect: &Type, namespace: &Namespace) -> Result<Object> where I: InfoProvider {
    let mut result = vec![];
    for (index, expression) in tuple_literal.expressions().enumerate() {
        result.push(fetch_expression(expression, schema, info_provider, expect.unwrap_optional().unwrap_tuple_index(index).unwrap(), namespace)?.as_teon().unwrap().clone());
    }
    Ok(Object::from(Value::Tuple(result)))
}

pub fn fetch_array_literal<I>(array_literal: &ArrayLiteral, schema: &Schema, info_provider: &I, expect: &Type, namespace: &Namespace) -> Result<Object> where I: InfoProvider {
    let mut result = vec![];
    for expression in array_literal.expressions() {
        result.push(fetch_expression(expression, schema, info_provider, expect.unwrap_optional().unwrap_array(), namespace)?.as_teon().unwrap().clone());
    }
    Ok(Object::from(Value::Array(result)))
}

pub fn fetch_dictionary_literal<I>(dictionary_literal: &DictionaryLiteral, schema: &Schema, info_provider: &I, expect: &Type, namespace: &Namespace) -> Result<Object> where I: InfoProvider {
    let mut result = indexmap!{};
    for named_expression in dictionary_literal.expressions() {
        let k = fetch_dictionary_key_expression(named_expression.key(), schema, info_provider, namespace)?.as_teon().unwrap().as_str().unwrap().to_owned();
        let v = fetch_expression(named_expression.value(), schema, info_provider, expect.unwrap_optional().unwrap_dictionary(), namespace)?.as_teon().unwrap().clone();
        result.insert(k, v);
    }
    Ok(Object::from(Value::Dictionary(result)))
}

pub fn fetch_enum_variant_literal<I>(e: &EnumVariantLiteral, schema: &Schema, info_provider: &I, expect: &Type, namespace: &Namespace) -> Result<Object> where I: InfoProvider {
    let expect = expect.unwrap_optional().unwrap_union_enum().unwrap();
    match expect {
        Type::EnumVariant(reference) => {
            let r#enum = schema.find_top_by_path(reference.path()).unwrap().as_enum().unwrap();
            if let Some(member) = r#enum.members().find(|m| m.identifier().name() == e.identifier().name()) {
                let mut args = None;
                if let Some(argument_list) = e.argument_list() {
                    let arg_list_result = fetch_argument_list(argument_list, schema, info_provider, namespace)?;
                }
                Ok(Object::from(Value::EnumVariant(EnumVariant {
                    value: member.identifier().name().to_owned(),
                    args,
                })))
            } else {
                unreachable!()
            }
        },
        Type::ModelScalarFields(t, _) => {
            if let Some((model_object, model_name)) = t.as_model_object() {
                let model = schema.find_top_by_path(model_object).unwrap().as_model().unwrap();
                if model.resolved().scalar_fields.contains(&e.identifier().name) {
                    Ok(Object::from(Value::EnumVariant(EnumVariant {
                        value: Box::new(Value::String(e.identifier().name().to_owned())),
                        display: format!(".{}", e.identifier().name()),
                        path: model_name.clone(),
                        args: None,
                    })))
                } else {
                    panic!()
                }
            } else {
                panic!()
            }
        },
        Type::ModelScalarFieldsWithoutVirtuals(t, _) => {
            if let Some((model_object, model_name)) = t.as_model_object() {
                let model = schema.find_top_by_path(model_object).unwrap().as_model().unwrap();
                if model.resolved().scalar_fields_without_virtuals.contains(&e.identifier().name) {
                    Ok(Object::from(Value::EnumVariant(EnumVariant {
                        value: Box::new(Value::String(e.identifier().name().to_owned())),
                        display: format!(".{}", e.identifier().name()),
                        path: model_name.clone(),
                        args: None,
                    })))
                } else {
                    panic!()
                }
            } else {
                panic!()
            }
        }
        Type::ModelScalarFieldsAndCachedPropertiesWithoutVirtuals(t, _) => {
            if let Some((model_object, model_name)) = t.as_model_object() {
                let model = schema.find_top_by_path(model_object).unwrap().as_model().unwrap();
                if model.resolved().scalar_fields_and_cached_properties_without_virtuals.contains(&e.identifier().name) {
                    Ok(Object::from(Value::EnumVariant(EnumVariant {
                        value: Box::new(Value::String(e.identifier().name().to_owned())),
                        display: format!(".{}", e.identifier().name()),
                        path: model_name.clone(),
                        args: None,
                    })))
                } else {
                    panic!()
                }
            } else {
                panic!()
            }
        }
        Type::ModelRelations(t, _) => {
            if let Some((model_object, model_name)) = t.as_model_object() {
                let model = schema.find_top_by_path(model_object).unwrap().as_model().unwrap();
                if model.resolved().relations.contains(&e.identifier().name) {
                    Ok(Object::from(Value::EnumVariant(EnumVariant {
                        value: Box::new(Value::String(e.identifier().name().to_owned())),
                        display: format!(".{}", e.identifier().name()),
                        path: model_name.clone(),
                        args: None,
                    })))
                } else {
                    panic!()
                }
            } else {
                panic!()
            }
        }
        Type::ModelDirectRelations(t, _) => {
            if let Some((model_object, model_name)) = t.as_model_object() {
                let model = schema.find_top_by_path(model_object).unwrap().as_model().unwrap();
                if model.resolved().direct_relations.contains(&e.identifier().name) {
                    Ok(Object::from(Value::EnumVariant(EnumVariant {
                        value: Box::new(Value::String(e.identifier().name().to_owned())),
                        display: format!(".{}", e.identifier().name()),
                        path: model_name.clone(),
                        args: None,
                    })))
                } else {
                    panic!()
                }
            } else {
                panic!()
            }
        },
        _ => panic!()
    }
}