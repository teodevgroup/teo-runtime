use teo_parser::ast::expression::{Expression, ExpressionKind};
use teo_parser::ast::reference_space::ReferenceSpace;

use teo_parser::ast::unit::Unit;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::r#type::Type;
use teo_parser::traits::named_identifiable::NamedIdentifiable;
use teo_parser::utils::top_filter::{top_filter_for_pipeline, top_filter_for_reference_type};
use teo_result::{Error, Result};
use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::Value;
use crate::namespace::Namespace;
use crate::object::Object;
use crate::object::traits::PrimitiveStruct;
use crate::schema::fetch::fetch_argument_list::fetch_argument_list;
use crate::schema::fetch::fetch_decorator_arguments::fetch_decorator_arguments;
use crate::schema::fetch::fetch_expression::fetch_expression;
use crate::schema::fetch::fetchers::fetch_identifier::{fetch_identifier, fetch_identifier_to_node};

#[derive(Debug)]
pub(super) enum UnitFetchResult {
    Reference(Vec<usize>),
    Object(Object),
}

impl UnitFetchResult {

    pub(super) fn into_object(self, schema: &Schema) -> Result<Object> {
        match self {
            UnitFetchResult::Object(o) => Ok(o),
            UnitFetchResult::Reference(r) => {
                let top = schema.find_top_by_path(&r).unwrap();
                if top.is_model() {
                    Ok(Object::from(Value::from(top.as_model().unwrap().string_path().clone())))
                } else if top.is_data_set() {
                    Ok(Object::from(Value::from(top.as_data_set().unwrap().string_path().clone())))
                } else {
                    Err(Error::new("cannot convert reference into object"))?
                }
            }
        }
    }
}

pub fn fetch_unit<I>(unit: &Unit, schema: &Schema, info_provider: &I, expect: &Type, namespace: &Namespace) -> Result<Object> where I: InfoProvider {
    if unit.expressions().count() == 1 {
        fetch_expression(unit.expression_at(0).unwrap(), schema, info_provider, expect, namespace)
    } else {
        let first_expression = unit.expression_at(0).unwrap();
        let expected = Type::Undetermined;
        let mut current = if let Some(identifier) = first_expression.kind.as_identifier() {
            let reference = fetch_identifier_to_node(identifier, schema, info_provider, &expected, namespace, &top_filter_for_pipeline())?;
            let top = schema.find_top_by_path(&reference).unwrap();
            if let Some(constant) = top.as_constant() {
                UnitFetchResult::Object(fetch_expression(&constant.expression, schema, info_provider, &expected, namespace)?)
            } else {
                UnitFetchResult::Reference(reference)
            }
        } else {
            UnitFetchResult::Object(fetch_expression(first_expression, schema, info_provider, &expected, namespace)?)
        };
        for (index, item) in unit.expressions().enumerate() {
            if index == 0 { continue }
            current = fetch_current_item_for_unit(&current, unit.expression_at(index - 1).unwrap(), schema, info_provider, &expected, namespace)?;
        }
        Ok(current.into_object(schema)?)
    }
}

fn fetch_current_item_for_unit<I>(current: &UnitFetchResult, item: &Expression, schema: &Schema, info_provider: &I, expect: &Type, namespace: &Namespace) -> Result<UnitFetchResult> where I: InfoProvider {
    match current {
        UnitFetchResult::Object(current_value) => {
            let path = if current_value.is_teon() {
                current_value.as_teon().unwrap().default_struct_path()?
            } else if current_value.is_struct_object() {
                current_value.as_struct_object().unwrap().struct_path()
            } else {
                Err(Error::new("not supported"))?
            };
            match &item.kind {
                ExpressionKind::Identifier(_) => {
                    Err(Error::new("not implemented"))
                }
                ExpressionKind::Subscript(subscript) => {
                    let r#struct = namespace.struct_at_path(&path).unwrap();
                    let function = r#struct.function("subscript").unwrap();
                    // let arguments = fetch_argument_list(&call.argument_list, schema, info_provider, namespace)?;
                    // let result = function.body.call(current_value.clone(), arguments)?;
                    // return Ok(UnitFetchResult::Object(result));
                    unreachable!()
                }
                _ => unreachable!(),
            }
        }
        UnitFetchResult::Reference(path) => {
            match schema.find_top_by_path(path).unwrap() {
                Top::StructDeclaration(struct_declaration) => {
                    let struct_object = Type::StructObject(struct_declaration.path.clone(), struct_declaration.string_path.clone());
                    match &item.kind {
                        ExpressionKind::ArgumentList(argument_list) => {
                            let r#struct = namespace.struct_at_path(&struct_declaration.str_path()).unwrap();
                            let function = r#struct.static_function("new").unwrap();
                            let arguments = fetch_argument_list(argument_list, schema, info_provider, namespace)?;
                            let result = function.body.call(arguments)?;
                            return Ok(UnitFetchResult::Object(result));
                        }
                        ExpressionKind::Call(call) => {
                            let r#struct = namespace.struct_at_path(&struct_declaration.str_path()).unwrap();
                            let function = r#struct.static_function(call.identifier().name()).unwrap();
                            let arguments = fetch_argument_list(&call.argument_list, schema, info_provider, namespace)?;
                            let result = function.body.call(arguments)?;
                            return Ok(UnitFetchResult::Object(result));
                        }
                        ExpressionKind::Subscript(s) => {
                            Err(Error::new("not implemented"))
                        }
                        ExpressionKind::Identifier(i) => {
                            Err(Error::new("not implemented"))
                        }
                        _ => unreachable!()
                    }
                },
                Top::Config(config) => {
                    match &item.kind {
                        ExpressionKind::Identifier(identifier) => {
                            if let Some(item) = config.items.iter().find(|i| i.identifier().name() == identifier.name()) {
                                return Ok(UnitFetchResult::Object(fetch_expression(&item.expression, schema, info_provider, expect, namespace)?));
                            } else {
                                Err(Error::new("config item not found"))?
                            }
                        },
                        ExpressionKind::ArgumentList(a) => {
                            Err(Error::new("config cannot be called"))?
                        }
                        ExpressionKind::Call(c) => {
                            Err(Error::new("config cannot be called"))?
                        }
                        ExpressionKind::Subscript(s) => {
                            Err(Error::new("config cannot be subscript"))?
                        }
                        _ => unreachable!()
                    }
                }
                Top::Constant(constant) => {
                    return Ok(UnitFetchResult::Object(fetch_expression(&constant.expression, schema, info_provider, expect, namespace)?));
                }
                Top::Enum(r#enum) => {
                    match &item.kind {
                        ExpressionKind::Identifier(i) => {
                            return Ok(UnitFetchResult::Object(Object::from(Value::EnumVariant(EnumVariant {
                                value: Box::new(Value::String(i.name().to_owned())),
                                display: format!(".{}", i.name()),
                                path: r#enum.string_path.clone(),
                                args: None,
                            }))));
                        }
                        ExpressionKind::Call(c) => {
                            return Ok(UnitFetchResult::Object(Object::from(Value::EnumVariant(EnumVariant {
                                value: Box::new(Value::String(c.identifier().name().to_owned())),
                                display: format!(".{}", c.identifier().name()),
                                path: r#enum.string_path.clone(),
                                args: None,
                            }))));
                        }
                        ExpressionKind::ArgumentList(a) => {
                            Err(Error::new("enum cannot be called"))?
                        }
                        ExpressionKind::Subscript(s) => {
                            Err(Error::new("enum cannot be subscript"))?
                        }
                        _ => unreachable!()
                    }
                }
                Top::Model(_) => {
                    match &item.kind {
                        ExpressionKind::Identifier(_) => todo!("return model field enum here"),
                        ExpressionKind::ArgumentList(a) => {
                            Err(Error::new("model cannot be called"))?
                        }
                        ExpressionKind::Call(c) => {
                            Err(Error::new("model cannot be called"))?
                        }
                        ExpressionKind::Subscript(s) => {
                            Err(Error::new("model cannot be subscript"))?
                        }
                        _ => unreachable!()
                    }
                }
                Top::Interface(_) => {
                    match &item.kind {
                        ExpressionKind::Identifier(_) => todo!("return interface field enum here"),
                        ExpressionKind::ArgumentList(a) => {
                            Err(Error::new("interface cannot be called"))?
                        }
                        ExpressionKind::Call(c) => {
                            Err(Error::new("interface cannot be called"))?
                        }
                        ExpressionKind::Subscript(s) => {
                            Err(Error::new("interface cannot be subscript"))?
                        }
                        _ => unreachable!()
                    }
                }
                Top::Namespace(namespace) => {
                    match &item.kind {
                        ExpressionKind::Identifier(identifier) => {
                            if let Some(top) = namespace.find_top_by_name(identifier.name(), &top_filter_for_reference_type(ReferenceSpace::Default), info_provider.availability()) {
                                return Ok(UnitFetchResult::Reference(top.path().clone()))
                            } else {
                                Err(Error::new("invalid reference"))?
                            }
                        },
                        ExpressionKind::Call(c) => {
                            todo!("resolve and call")
                        }
                        ExpressionKind::ArgumentList(a) => {
                            Err(Error::new("namespace cannot be called"))?
                        }
                        ExpressionKind::Subscript(s) => {
                            Err(Error::new("namespace cannot be subscript"))?
                        }
                        _ => unreachable!()
                    }
                }
                _ => unreachable!()
            }
        }
    }
}

