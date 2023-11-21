use maplit::btreemap;
use teo_parser::ast::expression::{Expression, ExpressionKind};
use teo_parser::ast::reference_space::ReferenceSpace;
use teo_parser::ast::unit::Unit;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::expr::{ExprInfo, ReferenceInfo, ReferenceType};
use teo_parser::r#type::reference::Reference;
use teo_parser::r#type::Type;
use teo_parser::traits::identifiable::Identifiable;
use teo_parser::traits::named_identifiable::NamedIdentifiable;
use teo_parser::traits::resolved::Resolve;
use teo_parser::utils::top_filter::top_filter_for_reference_type;
use teo_result::{Error, Result};
use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::Value;
use crate::arguments::Arguments;
use crate::namespace::Namespace;
use crate::object::Object;
use crate::object::traits::PrimitiveStruct;
use crate::schema::fetch::fetch_argument_list::fetch_argument_list;
use crate::schema::fetch::fetch_expression::fetch_expression;
use crate::schema::fetch::fetchers::fetch_identifier::{fetch_identifier_to_expr_info, fetch_identifier_to_node};

#[derive(Debug)]
pub(super) enum UnitFetchResult {
    Reference(ReferenceInfo, Option<Object>),
    Object(Object),
}

impl UnitFetchResult {

    pub(super) fn into_object(self, schema: &Schema, expect: &Type) -> Result<Object> {
        match self {
            UnitFetchResult::Object(o) => Ok(o),
            UnitFetchResult::Reference(reference_info, _) => {
                match reference_info.r#type() {
                    ReferenceType::Model => Ok(Object::from(Value::from(reference_info.reference().string_path().clone()))),
                    ReferenceType::DataSet => Ok(Object::from(Value::from(reference_info.reference().string_path().clone()))),
                    ReferenceType::ModelField => Ok(Object::from(Value::EnumVariant(EnumVariant {
                        value: reference_info.reference.string_path().last().unwrap().clone(),
                        args: None,
                    }))),
                    ReferenceType::EnumMember => Ok(Object::from(Value::EnumVariant(EnumVariant {
                        value: reference_info.reference.string_path().last().unwrap().clone(),
                        args: None,
                    }))),
                    _ => Err(Error::new("cannot convert reference into object"))?,
                }
            }
        }
    }
}

pub fn fetch_unit<I>(unit: &Unit, schema: &Schema, info_provider: &I, expect: &Type, namespace: &Namespace) -> Result<Object> where I: InfoProvider {
    if unit.expressions().count() == 1 {
        fetch_expression(unit.expression_at(0).unwrap(), schema, info_provider, expect, namespace)
    } else {
        let mut current = None;
        for expression in unit.expressions() {
            current = Some(fetch_current_item_for_unit(current, expression, schema, info_provider, &Type::Undetermined, namespace)?);
        }
        // here should add coerce
        Ok(current.into_object(schema)?)
    }
}

fn fetch_current_item_for_unit<I>(current: Option<UnitFetchResult>, expression: &Expression, schema: &Schema, info_provider: &I, expect: &Type, namespace: &Namespace) -> Result<UnitFetchResult> where I: InfoProvider {
    let Some(current) = current else {
        let expected = Type::Undetermined;
        return Ok(if let Some(identifier) = expression.kind.as_identifier() {
            let top = fetch_identifier_to_node(identifier, schema, info_provider, &expected, &top_filter_for_reference_type(ReferenceSpace::Default))?;
            let expr_info = fetch_identifier_to_expr_info(identifier, schema, info_provider, &expected,  &top_filter_for_reference_type(ReferenceSpace::Default))?;
            if let Some(constant) = top.as_constant_declaration() {
                UnitFetchResult::Object(fetch_expression(constant.expression(), schema, info_provider, &expected, namespace)?)
            } else {
                UnitFetchResult::Reference(expr_info.reference_info().unwrap().clone(), None)
            }
        } else {
            UnitFetchResult::Object(fetch_expression(expression, schema, info_provider, &expected, namespace)?)
        });
    };
    match current {
        UnitFetchResult::Object(current_value) => {
            let path = if current_value.is_teon() {
                current_value.as_teon().unwrap().default_struct_path()?
            } else if current_value.is_struct_object() {
                current_value.as_struct_object().unwrap().struct_path()
            } else {
                Err(Error::new("not supported"))?
            };
            let r#struct = namespace.struct_at_path(&path).unwrap();
            let struct_definition = schema.std_source().find_node_by_string_path(&path).unwrap().as_struct_declaration().unwrap();
            match &expression.kind {
                ExpressionKind::Identifier(identifier) => {
                    let instance_function = r#struct.function(identifier.name()).unwrap();
                    let instance_function_definition = struct_definition.instance_function(identifier.name()).unwrap();
                    UnitFetchResult::Reference(ReferenceInfo::new(
                        ReferenceType::StructInstanceFunction,
                        Reference::new(instance_function_definition.path().clone(), instance_function_definition.string_path().clone()),
                        None,
                    ), Some(current_value))
                }
                ExpressionKind::Subscript(subscript) => {
                    let instance_function = r#struct.function("subscript").unwrap();
                    let instance_function_definition = struct_definition.instance_function("subscript").unwrap();
                    let only_argument_declaration = instance_function_definition.argument_list_declaration().argument_declarations().next().unwrap();
                    let expected = only_argument_declaration.type_expr().resolved();
                    let only_argument = fetch_expression(subscript.expression(), schema, info_provider, expected, namespace)?;
                    let only_argument_name = only_argument_declaration.name().name();
                    let arguments = Arguments::new(btreemap! {only_argument_name.to_owned() => only_argument});
                    return Ok(UnitFetchResult::Object(instance_function.body.call(current_value, arguments)?));
                }
                _ => unreachable!(),
            }
        }
        UnitFetchResult::Reference(reference_info, this) => {
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

