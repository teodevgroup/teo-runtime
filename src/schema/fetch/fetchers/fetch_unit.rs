use maplit::btreemap;
use teo_parser::ast::expression::{Expression, ExpressionKind};
use teo_parser::ast::reference_space::ReferenceSpace;
use teo_parser::ast::unit::Unit;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::expr::{ExprInfo, ReferenceInfo, ReferenceType};
use teo_parser::r#type::reference::Reference;
use teo_parser::r#type::Type;
use teo_parser::search::search_identifier_path::{search_identifier_path_names_with_filter_to_expr_info, search_identifier_path_names_with_filter_to_top_multiple};
use teo_parser::traits::identifiable::Identifiable;
use teo_parser::traits::named_identifiable::NamedIdentifiable;
use teo_parser::traits::node_trait::NodeTrait;
use teo_parser::traits::resolved::Resolve;
use teo_parser::utils::top_filter::top_filter_for_reference_type;
use teo_result::{Error, Result};
use crate::value::Value;
use crate::arguments::Arguments;
use crate::value::interface_enum_variant::InterfaceEnumVariant;
use crate::namespace::Namespace;
use crate::schema::fetch::fetch_argument_list::{fetch_argument_list, fetch_argument_list_or_empty};
use crate::schema::fetch::fetch_expression::fetch_expression;
use crate::schema::fetch::fetchers::fetch_identifier::{fetch_identifier_to_expr_info, fetch_identifier_to_node};
use crate::value::option_variant::OptionVariant;

#[derive(Debug)]
pub(super) enum UnitFetchResult {
    Reference(ReferenceInfo, Option<Value>),
    Value(Value),
}

impl UnitFetchResult {

    pub(super) fn into_object(self) -> Result<Value> {
        match self {
            UnitFetchResult::Value(o) => Ok(o),
            UnitFetchResult::Reference(reference_info, _) => {
                match reference_info.r#type() {
                    ReferenceType::Model => Ok(Value::from(reference_info.reference().string_path().clone())),
                    ReferenceType::DataSet => Ok(Value::from(reference_info.reference().string_path().clone())),
                    ReferenceType::ModelField => Ok(Value::String(reference_info.reference.string_path().last().unwrap().clone())),
                    ReferenceType::EnumMember => Ok(Value::String(reference_info.reference.string_path().last().unwrap().clone())),
                    _ => Err(Error::new("cannot convert reference into value"))?,
                }
            }
        }
    }
}

pub fn fetch_unit<I>(unit: &Unit, schema: &Schema, info_provider: &I, expect: &Type, namespace: &Namespace) -> Result<Value> where I: InfoProvider {
    if unit.expressions().count() == 1 {
        fetch_expression(unit.expression_at(0).unwrap(), schema, info_provider, expect, namespace)
    } else {
        let mut current = None;
        for expression in unit.expressions() {
            current = Some(fetch_current_item_for_unit(current, expression, schema, info_provider, &Type::Undetermined, namespace)?);
        }
        // here should add coerce
        Ok(current.unwrap().into_object()?)
    }
}

fn fetch_current_item_for_unit<I>(current: Option<UnitFetchResult>, expression: &Expression, schema: &Schema, info_provider: &I, expect: &Type, namespace: &Namespace) -> Result<UnitFetchResult> where I: InfoProvider {
    let Some(current) = current else {
        let expected = Type::Undetermined;
        return Ok(if let Some(identifier) = expression.kind.as_identifier() {
            let top = fetch_identifier_to_node(identifier, schema, info_provider, &expected, &top_filter_for_reference_type(ReferenceSpace::Default))?;
            let expr_info = fetch_identifier_to_expr_info(identifier, schema, info_provider, &expected,  &top_filter_for_reference_type(ReferenceSpace::Default))?;
            if let Some(constant) = top.as_constant_declaration() {
                UnitFetchResult::Value(fetch_expression(constant.expression(), schema, info_provider, &expected, namespace)?)
            } else {
                UnitFetchResult::Reference(expr_info.reference_info().unwrap().clone(), None)
            }
        } else {
            UnitFetchResult::Value(fetch_expression(expression, schema, info_provider, &expected, namespace)?)
        });
    };
    match current {
        UnitFetchResult::Value(current_value) => {
            if current_value.is_interface_enum_variant() {
                todo!()
            } else {
                let path = if current_value.is_struct_object() {
                    current_value.as_struct_object().unwrap().struct_path()
                } else {
                    current_value.default_struct_path()?
                };
                let r#struct = namespace.struct_at_path(&path).unwrap();
                let struct_definition = schema.std_source().find_node_by_string_path(&path, &top_filter_for_reference_type(ReferenceSpace::Default), info_provider.availability()).unwrap().as_struct_declaration().unwrap();
                match &expression.kind {
                    ExpressionKind::Identifier(identifier) => {
                        let instance_function = r#struct.function(identifier.name()).unwrap();
                        let instance_function_definition = struct_definition.instance_function(identifier.name()).unwrap();
                        Ok(UnitFetchResult::Reference(ReferenceInfo::new(
                            ReferenceType::StructInstanceFunction,
                            Reference::new(instance_function_definition.path().clone(), instance_function_definition.string_path().clone()),
                            None,
                        ), Some(current_value)))
                    }
                    ExpressionKind::Subscript(subscript) => {
                        let instance_function = r#struct.function("subscript").unwrap();
                        let instance_function_definition = struct_definition.instance_function("subscript").unwrap();
                        let only_argument_declaration = instance_function_definition.argument_list_declaration().argument_declarations().next().unwrap();
                        let expected = only_argument_declaration.type_expr().resolved();
                        let only_argument = fetch_expression(subscript.expression(), schema, info_provider, expected, namespace)?;
                        let only_argument_name = only_argument_declaration.name().name();
                        let arguments = Arguments::new(btreemap! {only_argument_name.to_owned() => only_argument});
                        return Ok(UnitFetchResult::Value(instance_function.body.call(current_value, arguments)?));
                    }
                    _ => unreachable!(),
                }
            }
        }
        UnitFetchResult::Reference(reference_info, this) => {
            match reference_info.r#type {
                ReferenceType::Config => {
                    let config = schema.find_top_by_path(reference_info.reference().path()).unwrap().as_config().unwrap();
                    match &expression.kind {
                        ExpressionKind::Identifier(identifier) => {
                            if let Some((_, v)) = config.items().iter().find(|(k, v)| k.named_key_without_resolving().unwrap() == identifier.name()) {
                                return Ok(UnitFetchResult::Value(fetch_expression(v, schema, info_provider, expect, namespace)?));
                            } else {
                                Err(Error::new("config item not found"))?
                            }
                        }
                        _ => unreachable!()
                    }
                }
                ReferenceType::DictionaryField => unreachable!(),
                ReferenceType::Constant => unreachable!(),
                ReferenceType::Enum => {
                    let r#enum = schema.find_top_by_path(reference_info.reference().path()).unwrap().as_enum().unwrap();
                    match &expression.kind {
                        ExpressionKind::Identifier(i) => {
                            if r#enum.option {
                                Ok(UnitFetchResult::Value(Value::OptionVariant(Value::from(OptionVariant {
                                    value: r#enum.members().find(|m| m.name() == i.name()).unwrap().resolved().to_int().unwrap(),
                                    display: format!(".{}", i.name()),
                                }))))
                            } else if r#enum.interface {
                                let member_definition = r#enum.members().find(|m| m.identifier().name() == i.name()).unwrap();
                                if member_definition.argument_list_declaration().is_some() {
                                    Ok(UnitFetchResult::Reference(ReferenceInfo::new(
                                        ReferenceType::EnumMember,
                                        Reference::new(member_definition.path().clone(), member_definition.string_path().clone()),
                                        None,
                                    ), None))
                                } else {
                                    Ok(UnitFetchResult::Value(Value::from(InterfaceEnumVariant {
                                        value: i.name().to_owned(),
                                        args: None
                                    })))
                                }
                            } else {
                                Ok(UnitFetchResult::Value(Value::from(Value::String(i.name().to_owned()))))
                            }
                        }
                        _ => unreachable!()
                    }
                }
                ReferenceType::EnumMember => {
                    let r#enum = schema.find_top_by_path(&reference_info.reference().path_without_last(1)).unwrap().as_enum().unwrap();
                    let member = r#enum.child(*reference_info.reference().path().last().unwrap()).unwrap().as_enum_member().unwrap();
                    match &expression.kind {
                        ExpressionKind::ArgumentList(argument_list) => {
                            let args = fetch_argument_list_or_empty(Some(argument_list), schema, info_provider, namespace)?;
                            Ok(UnitFetchResult::Value(Value::from(InterfaceEnumVariant {
                                value: member.identifier().name().to_owned(),
                                args: Some(args)
                            })))
                        },
                        _ => unreachable!()
                    }
                }
                ReferenceType::Model => {
                    let model = schema.find_top_by_path(reference_info.reference().path()).unwrap().as_model().unwrap();
                    match &expression.kind {
                        ExpressionKind::Identifier(identifier) => {
                            if let Some(field) = model.fields().find(|f| f.identifier().name() == identifier.name()) {
                                Ok(UnitFetchResult::Reference(ReferenceInfo::new(
                                    ReferenceType::ModelField,
                                    Reference::new(field.path().clone(), field.string_path().clone()),
                                    None,
                                ), None))
                            } else {
                                Err(Error::new("model field not found"))
                            }
                        },
                        _ => unreachable!()
                    }
                }
                ReferenceType::Middleware => todo!("accept argument list into middleware"),
                ReferenceType::StructDeclaration => {
                    let r#struct = namespace.struct_at_path(&reference_info.reference().str_path()).unwrap();
                    let struct_definition = schema.std_source().find_node_by_string_path(&reference_info.reference().str_path(), &top_filter_for_reference_type(ReferenceSpace::Default), info_provider.availability()).unwrap().as_struct_declaration().unwrap();
                    match &expression.kind {
                        ExpressionKind::ArgumentList(argument_list) => {
                            let function = r#struct.static_function("new").unwrap();
                            let arguments = fetch_argument_list(argument_list, schema, info_provider, namespace)?;
                            let result = function.body.call(arguments)?;
                            return Ok(UnitFetchResult::Value(result));
                        }
                        ExpressionKind::Identifier(i) => {
                            let static_function_definition = struct_definition.static_function(i.name()).unwrap();
                            return Ok(UnitFetchResult::Reference(ReferenceInfo::new(
                                ReferenceType::StructStaticFunction,
                                Reference::new(static_function_definition.path().clone(), static_function_definition.string_path().clone()),
                                None,
                            ), None));
                        }
                        _ => unreachable!()
                    }
                }
                ReferenceType::StructInstanceFunction => {
                    let r#struct = namespace.struct_at_path(&reference_info.reference().str_path_without_last(1)).unwrap();
                    match &expression.kind {
                        ExpressionKind::ArgumentList(argument_list) => {
                            let function = r#struct.function(reference_info.reference().str_path().last().unwrap()).unwrap();
                            let arguments = fetch_argument_list(argument_list, schema, info_provider, namespace)?;
                            let result = function.body.call(this.unwrap(), arguments)?;
                            return Ok(UnitFetchResult::Value(result));
                        }
                        _ => unreachable!()
                    }
                }
                ReferenceType::StructStaticFunction => {
                    let r#struct = namespace.struct_at_path(&reference_info.reference().str_path_without_last(1)).unwrap();
                    match &expression.kind {
                        ExpressionKind::ArgumentList(argument_list) => {
                            let function = r#struct.static_function(reference_info.reference().str_path().last().unwrap()).unwrap();
                            let arguments = fetch_argument_list(argument_list, schema, info_provider, namespace)?;
                            let result = function.body.call(arguments)?;
                            return Ok(UnitFetchResult::Value(result));
                        }
                        _ => unreachable!()
                    }
                }
                ReferenceType::FunctionDeclaration => unreachable!(),
                ReferenceType::Namespace => {
                    match &expression.kind {
                        ExpressionKind::Identifier(identifier) => {
                            let mut str_path = reference_info.reference().str_path();
                            str_path.push(identifier.name());
                            let expr_info = search_identifier_path_names_with_filter_to_expr_info(
                                &str_path,
                                schema,
                                schema.source(*info_provider.path().first().unwrap()).unwrap(),
                                &info_provider.namespace_str_path(),
                                &top_filter_for_reference_type(ReferenceSpace::Default),
                                info_provider.availability(),
                            ).unwrap();
                            return Ok(UnitFetchResult::Reference(expr_info.reference_info().unwrap().clone(), None))
                        },
                        _ => unreachable!()
                    }
                },
                _ => unreachable!(),
            }
        }
    }
}

