use teo_parser::ast::expression::{Expression, ExpressionKind};
use teo_parser::ast::unit::Unit;
use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::r#type::Type;
use teo_result::{Error, Result};
use crate::namespace::Namespace;
use crate::object::Object;
use crate::schema::fetch::fetch_argument_list::fetch_argument_list;
use crate::schema::fetch::fetch_decorator_arguments::fetch_decorator_arguments;
use crate::schema::fetch::fetch_expression::fetch_expression;
use crate::schema::fetch::fetchers::fetch_identifier::{fetch_identifier, fetch_identifier_path};

#[derive(Debug)]
pub(super) enum UnitFetchResult {
    Reference(Vec<usize>),
    Object(Object),
}

impl UnitFetchResult {

    pub(super) fn into_object(self, schema: &Schema) -> Result<Object> {
        unreachable!()
    }
}

pub fn fetch_unit<I>(unit: &Unit, schema: &Schema, info_provider: &I, expect: &Type, namespace: &Namespace) -> Result<Object> where I: InfoProvider {
    if unit.expressions.len() == 1 {
        fetch_expression(unit.expressions.as_ref().get(0).unwrap(), schema, info_provider, expect, namespace)
    } else {
        let first_expression = unit.expressions.get(0).unwrap();
        let expected = Type::Undetermined;
        let mut current = if let Some(identifier) = first_expression.kind.as_identifier() {
            let reference = fetch_identifier_path(identifier, schema, info_provider, &expected, namespace)?;
            let top = schema.find_top_by_path(&reference).unwrap();
            if let Some(constant) = top.as_constant() {
                UnitFetchResult::Object(fetch_expression(&constant.expression, schema, info_provider, &expected, namespace)?)
            } else {
                UnitFetchResult::Reference(reference)
            }
        } else {
            UnitFetchResult::Object(fetch_expression(first_expression, schema, info_provider, &expected, namespace)?)
        };
        for (index, item) in unit.expressions.iter().enumerate() {
            if index == 0 { continue }
            current = fetch_current_item_for_unit(&current, unit.expressions.get(index - 1).unwrap(), schema, info_provider, &expected, namespace)?;
        }
        current.into_resolved(schema)
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
                ExpressionKind::Call(call) => {
                    let r#struct = namespace.struct_at_path(&path).unwrap();
                    let function = r#struct.function(call.identifier.name()).unwrap();
                    let arguments = fetch_argument_list(&call.argument_list, schema, info_provider, namespace)?;
                    let result = function.body.call(current_value.clone(), arguments)?;
                    return Ok(UnitFetchResult::Object(result));
                }
                ExpressionKind::Subscript(subscript) => {
                    let r#struct = namespace.struct_at_path(&path).unwrap();
                    let function = r#struct.function("subscript").unwrap();
                    let arguments = fetch_argument_list(&call.argument_list, schema, info_provider, namespace)?;
                    let result = function.body.call(current_value.clone(), arguments)?;
                    return Ok(UnitFetchResult::Object(result));

                }
                _ => unreachable!(),
            }
        }
        UnitResolveResult::Reference(path) => {
            match context.schema.find_top_by_path(path).unwrap() {
                Top::StructDeclaration(struct_declaration) => {
                    let struct_object = Type::StructObject(struct_declaration.path.clone(), struct_declaration.string_path.clone());
                    match &item.kind {
                        ExpressionKind::ArgumentList(argument_list) => {
                            if let Some(new) = struct_declaration.function_declarations.iter().find(|f| f.r#static && f.identifier.name() == "new") {
                                resolve_argument_list(last_span, Some(argument_list), new.callable_variants(struct_declaration), &btreemap!{
                                    Keyword::SelfIdentifier => &struct_object,
                                },  context, None);
                                UnitResolveResult::Result(ExpressionResolved::type_only(new.return_type.resolved().clone()))
                            } else {
                                context.insert_diagnostics_error(last_span, "Constructor is not found");
                                return UnitResolveResult::Result(ExpressionResolved::undetermined())
                            }
                        }
                        ExpressionKind::Call(call) => {
                            if let Some(new) = struct_declaration.function_declarations.iter().find(|f| f.r#static && f.identifier.name() == call.identifier.name()) {
                                resolve_argument_list(last_span, Some(&call.argument_list), new.callable_variants(struct_declaration),  &btreemap!{
                                    Keyword::SelfIdentifier => &struct_object,
                                }, context, None);
                                UnitResolveResult::Result(ExpressionResolved::type_only(new.return_type.resolved().clone()))
                            } else {
                                context.insert_diagnostics_error(last_span, "static struct function is not found");
                                return UnitResolveResult::Result(ExpressionResolved::undetermined())
                            }
                        }
                        ExpressionKind::Subscript(s) => {
                            context.insert_diagnostics_error(s.span, "Struct cannot be subscript");
                            return UnitResolveResult::Result(ExpressionResolved::undetermined())
                        }
                        ExpressionKind::Identifier(i) => {
                            context.insert_diagnostics_error(i.span, "Struct fields are not accessible");
                            return UnitResolveResult::Result(ExpressionResolved::undetermined())
                        }
                        _ => unreachable!()
                    }
                },
                Top::Config(config) => {
                    match &item.kind {
                        ExpressionKind::Identifier(identifier) => {
                            if let Some(item) = config.items.iter().find(|i| i.identifier.name() == identifier.name()) {
                                return UnitResolveResult::Result(item.expression.resolved().clone());
                            } else {
                                context.insert_diagnostics_error(item.span(), "Undefined field");
                                return UnitResolveResult::Result(ExpressionResolved::undetermined())
                            }
                        },
                        ExpressionKind::ArgumentList(a) => {
                            context.insert_diagnostics_error(a.span, "Config cannot be called");
                            return UnitResolveResult::Result(ExpressionResolved::undetermined())
                        }
                        ExpressionKind::Call(c) => {
                            context.insert_diagnostics_error(c.span, "Config cannot be called");
                            return UnitResolveResult::Result(ExpressionResolved::undetermined())
                        }
                        ExpressionKind::Subscript(s) => {
                            context.insert_diagnostics_error(s.span, "Config cannot be subscript");
                            return UnitResolveResult::Result(ExpressionResolved::undetermined())
                        }
                        _ => unreachable!()
                    }
                }
                Top::Constant(constant) => {
                    if !constant.is_resolved() {
                        resolve_constant(constant, context);
                    }
                    UnitResolveResult::Result(constant.resolved().expression_resolved.clone())
                }
                Top::Enum(r#enum) => {
                    match &item.kind {
                        ExpressionKind::Identifier(i) => {
                            if let Some(member_declaration) = r#enum.members.iter().find(|m| m.identifier.name() == i.name()) {
                                if member_declaration.argument_list_declaration.is_some() {
                                    context.insert_diagnostics_error(i.span, "expect argument list");
                                }
                            } else {
                                context.insert_diagnostics_error(i.span, "enum member not found");
                            }
                            return UnitResolveResult::Result(ExpressionResolved {
                                r#type: Type::EnumVariant(r#enum.path.clone(), r#enum.string_path.clone()),
                                value: Some(Value::EnumVariant(EnumVariant {
                                    value: Box::new(Value::String(i.name().to_owned())),
                                    display: format!(".{}", i.name()),
                                    path: r#enum.string_path.clone(),
                                    args: None,
                                })),
                            });
                        }
                        ExpressionKind::Call(c) => {
                            if let Some(member_declaration) = r#enum.members.iter().find(|m| m.identifier.name() == c.identifier.name()) {
                                if member_declaration.argument_list_declaration.is_none() {
                                    context.insert_diagnostics_error(c.argument_list.span, "unexpected argument list");
                                } else {
                                    resolve_argument_list(
                                        c.identifier.span,
                                        Some(&c.argument_list),
                                        vec![CallableVariant {
                                            generics_declarations: vec![],
                                            argument_list_declaration: Some(member_declaration.argument_list_declaration.as_ref().unwrap()),
                                            generics_constraints: vec![],
                                            pipeline_input: None,
                                            pipeline_output: None,
                                        }],
                                        &btreemap! {},
                                        context,
                                        None,
                                    );
                                }
                            } else {
                                context.insert_diagnostics_error(c.identifier.span, "enum member not found");
                            }
                            return UnitResolveResult::Result(ExpressionResolved {
                                r#type: Type::EnumVariant(r#enum.path.clone(), r#enum.string_path.clone()),
                                value: Some(Value::EnumVariant(EnumVariant {
                                    value: Box::new(Value::String(c.identifier.name().to_owned())),
                                    display: format!(".{}", c.identifier.name()),
                                    path: r#enum.string_path.clone(),
                                    args: None,
                                })),
                            });
                        }
                        ExpressionKind::ArgumentList(a) => {
                            context.insert_diagnostics_error(a.span, "Enum cannot be called");
                            return UnitResolveResult::Result(ExpressionResolved::undetermined())
                        }
                        ExpressionKind::Subscript(s) => {
                            context.insert_diagnostics_error(s.span, "Enum cannot be subscript");
                            return UnitResolveResult::Result(ExpressionResolved::undetermined())
                        }
                        _ => unreachable!()
                    }
                }
                Top::Model(_) => {
                    match &item.kind {
                        ExpressionKind::Identifier(_) => todo!("return model field enum here"),
                        ExpressionKind::ArgumentList(a) => {
                            context.insert_diagnostics_error(a.span, "Model cannot be called");
                            return UnitResolveResult::Result(ExpressionResolved::undetermined())
                        }
                        ExpressionKind::Call(c) => {
                            context.insert_diagnostics_error(c.span, "Model cannot be called");
                            return UnitResolveResult::Result(ExpressionResolved::undetermined())
                        }
                        ExpressionKind::Subscript(s) => {
                            context.insert_diagnostics_error(s.span, "Model cannot be subscript");
                            return UnitResolveResult::Result(ExpressionResolved::undetermined())
                        }
                        _ => unreachable!()
                    }
                }
                Top::Interface(_) => {
                    match &item.kind {
                        ExpressionKind::Identifier(_) => todo!("return interface field enum here"),
                        ExpressionKind::ArgumentList(a) => {
                            context.insert_diagnostics_error(a.span, "Interface cannot be called");
                            return UnitResolveResult::Result(ExpressionResolved::undetermined())
                        }
                        ExpressionKind::Call(c) => {
                            context.insert_diagnostics_error(c.span, "Interface cannot be called");
                            return UnitResolveResult::Result(ExpressionResolved::undetermined())
                        }
                        ExpressionKind::Subscript(s) => {
                            context.insert_diagnostics_error(s.span, "Interface cannot be subscript");
                            return UnitResolveResult::Result(ExpressionResolved::undetermined())
                        }
                        _ => unreachable!()
                    }
                }
                Top::Namespace(namespace) => {
                    match &item.kind {
                        ExpressionKind::Identifier(identifier) => {
                            if let Some(top) = namespace.find_top_by_name(identifier.name(), &top_filter_for_reference_type(ReferenceType::Default), context.current_availability()) {
                                return UnitResolveResult::Reference(top.path().clone())
                            } else {
                                context.insert_diagnostics_error(identifier.span, "Invalid reference");
                                return UnitResolveResult::Result(ExpressionResolved::undetermined())
                            }
                        },
                        ExpressionKind::Call(c) => {
                            todo!("resolve and call")
                        }
                        ExpressionKind::ArgumentList(a) => {
                            context.insert_diagnostics_error(a.span, "Namespace cannot be called");
                            return UnitResolveResult::Result(ExpressionResolved::undetermined())
                        }
                        ExpressionKind::Subscript(s) => {
                            context.insert_diagnostics_error(s.span, "Namespace cannot be subscript");
                            return UnitResolveResult::Result(ExpressionResolved::undetermined())
                        }
                        _ => unreachable!()
                    }
                }
                _ => unreachable!()
            }
        }
    }
}

