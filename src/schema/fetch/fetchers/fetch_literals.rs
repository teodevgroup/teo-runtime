use indexmap::indexmap;
use maplit::btreemap;
use teo_parser::ast::literals::{TupleLiteral, ArrayLiteral, DictionaryLiteral, EnumVariantLiteral};
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::r#type::synthesized_enum::SynthesizedEnum;
use teo_parser::r#type::synthesized_interface_enum::SynthesizedInterfaceEnum;
use teo_parser::r#type::Type;
use teo_parser::traits::named_identifiable::NamedIdentifiable;
use teo_parser::traits::resolved::Resolve;
use teo_result::Result;
use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::types::option_variant::OptionVariant;
use teo_teon::Value;
use crate::arguments::Arguments;
use crate::coder::json_to_teon::{fetch_synthesized_enum, fetch_synthesized_interface_enum};
use crate::interface_enum_variant::InterfaceEnumVariant;
use crate::namespace::Namespace;
use crate::object::Object;
use crate::schema::fetch::fetch_argument_list::fetch_argument_list;
use crate::schema::fetch::fetch_expression::{fetch_dictionary_key_expression, fetch_expression};
use crate::utils::ContainsStr;

pub fn fetch_tuple_literal<I>(tuple_literal: &TupleLiteral, schema: &Schema, info_provider: &I, expect: &Type, namespace: &Namespace) -> Result<Object> where I: InfoProvider {
    let mut result = vec![];
    for (index, expression) in tuple_literal.expressions().enumerate() {
        result.push(fetch_expression(expression, schema, info_provider, expect.unwrap_optional().unwrap_tuple_index(index).unwrap(), namespace)?.as_teon().unwrap().clone());
    }
    Ok(Object::from(Value::Tuple(result)))
}

pub fn fetch_array_literal<I>(array_literal: &ArrayLiteral, schema: &Schema, info_provider: &I, expect: &Type, namespace: &Namespace) -> Result<Object> where I: InfoProvider {
    let mut teon_result = vec![];
    let mut array_result = vec![];
    for expression in array_literal.expressions() {
        let expression_result = fetch_expression(expression, schema, info_provider, expect.unwrap_optional().unwrap_array(), namespace)?;
        if expression_result.is_interface_enum_variant() {
            array_result.push(expression_result);
        } else {
            teon_result.push(expression_result.as_teon().unwrap().clone());
        }
    }
    if !array_result.is_empty() {
        Ok(Object::from(array_result))
    } else {
        Ok(Object::from(Value::Array(teon_result)))
    }
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
    match expect {
        Type::EnumVariant(reference) => {
            let r#enum = schema.find_top_by_path(reference.path()).unwrap().as_enum().unwrap();
            if let Some(member) = r#enum.members().find(|m| m.identifier().name() == e.identifier().name()) {
                let mut args = None;
                if let Some(argument_list) = e.argument_list() {
                    args = Some(fetch_argument_list(argument_list, schema, info_provider, namespace)?);
                } else if member.argument_list_declaration().is_some() {
                    args = Some(Arguments::new(btreemap! {}))
                }
                if r#enum.option {
                    Ok(Object::from(Value::OptionVariant(OptionVariant {
                        value: member.resolved().try_into()?,
                        display: format!(".{}", member.identifier().name()),
                    })))
                } else if r#enum.interface {
                    Ok(Object::from(InterfaceEnumVariant {
                        value: member.identifier().name().to_owned(),
                        args,
                    }))
                } else {
                    Ok(Object::from(Value::EnumVariant(EnumVariant {
                        value: member.identifier().name().to_owned(),
                        args: None,
                    })))
                }
            } else {
                unreachable!()
            }
        },
        Type::SynthesizedEnum(synthesized_enum) => {
            fetch_enum_variant_literal_from_synthesized_enum(e, schema, info_provider, synthesized_enum, namespace)
        },
        Type::SynthesizedEnumReference(synthesized_enum_reference) => {
            let synthesized_enum = fetch_synthesized_enum(synthesized_enum_reference, namespace);
            fetch_enum_variant_literal_from_synthesized_enum(e, schema, info_provider, synthesized_enum, namespace)
        },
        Type::SynthesizedInterfaceEnum(synthesized_interface_enum) => {
            fetch_enum_variant_literal_from_synthesized_interface_enum(e, schema, info_provider, synthesized_interface_enum, namespace)
        },
        Type::SynthesizedInterfaceEnumReference(synthesized_interface_enum_reference) => {
            let synthesized_interface_enum = fetch_synthesized_interface_enum(synthesized_interface_enum_reference, schema);
            fetch_enum_variant_literal_from_synthesized_interface_enum(e, schema, info_provider, synthesized_interface_enum, namespace)
        },
        Type::FieldName(name) => {
            Ok(Object::from(Value::EnumVariant(EnumVariant {
                value: name.clone(),
                args: None,
            })))
        }
        _ => unreachable!(),
    }
}

fn fetch_enum_variant_literal_from_synthesized_interface_enum<I>(e: &EnumVariantLiteral, schema: &Schema, info_provider: &I, synthesized_enum: &SynthesizedInterfaceEnum, namespace: &Namespace) -> Result<Object> where I: InfoProvider {
    if synthesized_enum.keys.contains_str(e.identifier().name()) {
        let args = if let Some(argument_list) = e.argument_list() {
            let mut map = btreemap! {};
            for argument in argument_list.arguments() {
                map.insert(argument.name().unwrap().name().to_owned(), Object::from(argument.value().resolved().value().unwrap().to_owned()));
            }
            let arguments = Arguments::new(map);
            Some(arguments)
        } else {
            None
        };
        Ok(Object::from(InterfaceEnumVariant {
            value: e.identifier().name().to_owned(),
            args,
        }))
    } else {
        unreachable!()
    }
}

fn fetch_enum_variant_literal_from_synthesized_enum<I>(e: &EnumVariantLiteral, schema: &Schema, info_provider: &I, synthesized_enum: &SynthesizedEnum, namespace: &Namespace) -> Result<Object> where I: InfoProvider {
    if synthesized_enum.keys.contains_str(e.identifier().name()) {
        Ok(Object::from(Value::EnumVariant(EnumVariant {
            value: e.identifier().name().to_owned(),
            args: None,
        })))
    } else {
        unreachable!()
    }
}