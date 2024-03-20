use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;
use bigdecimal::{BigDecimal, FromPrimitive};
use bson::oid::ObjectId;
use chrono::{DateTime, NaiveDate, Utc};
use indexmap::IndexMap;
use key_path::KeyPath;
use maplit::btreemap;
use teo_parser::ast::schema::Schema;
use teo_parser::r#type::synthesized_enum::SynthesizedEnum;
use teo_parser::r#type::synthesized_enum_reference::SynthesizedEnumReference;
use teo_parser::r#type::synthesized_interface_enum::SynthesizedInterfaceEnum;
use teo_parser::r#type::synthesized_interface_enum_reference::SynthesizedInterfaceEnumReference;
use teo_parser::r#type::synthesized_shape::SynthesizedShape;
use teo_parser::r#type::synthesized_shape_reference::SynthesizedShapeReference;
use teo_parser::r#type::Type;
use teo_parser::traits::resolved::Resolve;
use crate::value::Value;
use crate::value::file::File;
use crate::interface::Interface;
use crate::namespace::Namespace;
use teo_result::Error;
use crate::utils::ContainsStr;


pub fn fetch_synthesized_interface_enum<'a>(reference: &SynthesizedInterfaceEnumReference, schema: &'a Schema) -> &'a SynthesizedInterfaceEnum {
    let model = schema.find_top_by_path(reference.owner.as_model_object().unwrap().path()).unwrap().as_model().unwrap();
    model.resolved().interface_enums.get(&reference.kind).unwrap()
}

pub fn fetch_synthesized_enum<'a>(reference: &SynthesizedEnumReference, main_namespace: &'a Namespace) -> &'a SynthesizedEnum {
    let model = main_namespace.model_at_path(&reference.owner.as_model_object().unwrap().str_path()).unwrap();
    model.cache.shape.enums.get(&reference.kind).unwrap()
}

pub fn fetch_input<'a>(reference: &SynthesizedShapeReference, main_namespace: &'a Namespace) -> &'a Type {
    let model = main_namespace.model_at_path(&reference.owner.as_model_object().unwrap().str_path()).unwrap();
    if reference.kind.requires_without() {
        model.cache.shape.get_without(reference.kind, reference.without.as_ref().unwrap()).unwrap()
    } else {
        model.cache.shape.get(reference.kind).unwrap()
    }
}

pub fn json_to_teon_with_type(json: &serde_json::Value, path: &KeyPath, t: &Type, main_namespace: &Namespace) -> teo_result::Result<Value> {
    match t {
        Type::Undetermined => Ok(Value::from(json)),
        Type::Ignored => Ok(Value::from(json)),
        Type::Any => Ok(Value::from(json)),
        Type::Null => if json.is_null() { Ok(Value::Null) } else { Err(Error::invalid_request_pathed(path.clone(), "expect null")) },
        Type::Bool => if json.is_boolean() { Ok(Value::from(json)) } else { Err(Error::invalid_request_pathed(path.clone(), "expect bool")) },
        Type::Int => if json.is_i64() { Ok(Value::Int(json.as_i64().unwrap() as i32)) } else { Err(Error::invalid_request_pathed(path.clone(), "expect int")) },
        Type::Int64 => if json.is_i64() { Ok(Value::Int64(json.as_i64().unwrap())) } else { Err(Error::invalid_request_pathed(path.clone(), "expect int 64")) },
        Type::Float32 => if json.is_f64() { Ok(Value::Float32(json.as_f64().unwrap() as f32)) } else if json.is_i64() { Ok(Value::Float32(json.as_i64().unwrap() as f32)) } else { Err(Error::invalid_request_pathed(path.clone(), "expect float 32")) },
        Type::Float => if json.is_f64() { Ok(Value::Float(json.as_f64().unwrap())) } else if json.is_i64() { Ok(Value::Float(json.as_i64().unwrap() as f64)) } else { Err(Error::invalid_request_pathed(path.clone(), "expect float")) },
        Type::Decimal => if json.is_string() {
            Ok(Value::Decimal(match BigDecimal::from_str(json.as_str().unwrap()) {
                Ok(s) => s,
                Err(_) => Err(Error::invalid_request_pathed(path.clone(), "string is not valid decimal"))?,
            }))
        } else if json.is_number() {
            if let Some(f) = json.as_f64() {
                Ok(Value::Decimal(match BigDecimal::from_f64(f) {
                    Some(s) => s,
                    None => Err(Error::invalid_request_pathed(path.clone(), "number is not valid decimal"))?,
                }))
            } else if let Some(i) = json.as_i64() {
                Ok(Value::Decimal(match BigDecimal::from_i64(i) {
                    Some(s) => s,
                    None => Err(Error::invalid_request_pathed(path.clone(), "number is not valid decimal"))?,
                }))
            } else {
                unreachable!()
            }
        } else {
            Err(Error::invalid_request_pathed(path.clone(), "expect string or number which represents decimal"))
        }
        Type::String => if json.is_string() { Ok(Value::String(json.as_str().unwrap().to_owned())) } else { Err(Error::invalid_request_pathed(path.clone(), "expect string")) },
        Type::ObjectId => if json.is_string() {
            Ok(Value::ObjectId(match ObjectId::parse_str(json.as_str().unwrap()) {
                Ok(s) => s,
                Err(_) => Err(Error::invalid_request_pathed(path.clone(), "string is not valid object id"))?,
            }))
        } else {
            Err(Error::invalid_request_pathed(path.clone(), "expect string represents object id"))
        }
        Type::Date => if json.is_string() {
            Ok(Value::Date(match NaiveDate::parse_from_str(json.as_str().unwrap(), "%Y-%m-%d") {
                Ok(s) => s,
                Err(_) => Err(Error::invalid_request_pathed(path.clone(), "string is not valid date"))?,
            }))
        } else {
            Err(Error::invalid_request_pathed(path.clone(), "expect string represents date"))
        }
        Type::DateTime => if json.is_string() {
            Ok(Value::DateTime(match DateTime::parse_from_rfc3339(json.as_str().unwrap()) {
                Ok(d) => d.with_timezone(&Utc),
                Err(_) => Err(Error::invalid_request_pathed(path.clone(), "string is not valid datetime"))?,
            }))
        } else {
            Err(Error::invalid_request_pathed(path.clone(), "expect string represents datetime"))
        }
        Type::File => {
            Ok(Value::File(match File::try_from(json) {
                Ok(f) => f,
                Err(err) => Err(Error::invalid_request_pathed(path.clone(), err.message()))?,
            }))
        },
        Type::Enumerable(inner) => {
            if let Some(json_array) = json.as_array() {
                let values: Vec<Value> = json_array.iter().enumerate().map(|(i, j)| Ok(json_to_teon_with_type(j, &(path + i), inner.as_ref(), main_namespace)?)).collect::<teo_result::Result<Vec<Value>>>()?;
                Ok(Value::Array(values))
            } else {
                Ok(Value::Array(vec![json_to_teon_with_type(json, path, inner.as_ref(), main_namespace)?]))
            }
        }
        Type::Array(inner) => {
            if let Some(json_array) = json.as_array() {
                let values: Vec<Value> = json_array.iter().enumerate().map(|(i, j)| json_to_teon_with_type(j, &(path + i), inner.as_ref(), main_namespace)).collect::<teo_result::Result<Vec<Value>>>()?;
                Ok(Value::Array(values))
            } else {
                Err(Error::invalid_request_pathed(path.clone(), "expect array"))
            }
        }
        Type::Dictionary(inner) => {
            if let Some(json_object) = json.as_object() {
                let values: IndexMap<String, Value> = json_object.iter().map(|(k, j)| Ok((k.clone(), json_to_teon_with_type(j, &(path + k), inner.as_ref(), main_namespace)?))).collect::<teo_result::Result<IndexMap<String, Value>>>()?;
                Ok(Value::Dictionary(values))
            } else {
                Err(Error::invalid_request_pathed(path.clone(), "expect dictionary"))
            }
        }
        Type::Tuple(_) => Err(Error::invalid_request_pathed(path.clone(), "unexpected type"))?,
        Type::Range(_) => Err(Error::invalid_request_pathed(path.clone(), "unexpected type"))?,
        Type::Union(inners) => {
            for inner in inners {
                if let Ok(result) = json_to_teon_with_type(json, path, inner, main_namespace) {
                    return Ok(result);
                }
            }
            Err(Error::invalid_request_pathed(path.clone(), "unexpected value"))
        }
        Type::EnumVariant(reference) => if json.is_string() {
            let e = main_namespace.enum_at_path(&reference.str_path()).unwrap();
            if e.cache.member_names.contains_str(json.as_str().unwrap()) {
                Ok(Value::String(json.as_str().unwrap().to_owned()))
            } else {
                Err(Error::invalid_request_pathed(path.clone(), "expect enum member"))
            }
        } else {
            Err(Error::invalid_request_pathed(path.clone(), "expect string represents enum member"))
        }
        Type::InterfaceObject(reference, gens) => {
            let i = main_namespace.interface_at_path(&reference.str_path()).unwrap();
            let shape = collect_interface_shape(i, gens);
            json_to_teon_with_shape(json, path, &shape, main_namespace)
        }
        Type::Optional(inner) => {
            if json.is_null() {
                Ok(Value::Null)
            } else {
                json_to_teon_with_type(json, path, inner.as_ref(), main_namespace)
            }
        },
        Type::SynthesizedShapeReference(shape_reference) => {
            let input = fetch_input(shape_reference, main_namespace);
            json_to_teon(json, path, input, main_namespace)
        },
        Type::SynthesizedEnumReference(enum_reference) => {
            let synthesized_enum = fetch_synthesized_enum(enum_reference, main_namespace);
            json_to_teon_with_synthesized_enum(json, path, synthesized_enum)
        },
        Type::SynthesizedEnum(synthesized_enum) => json_to_teon_with_synthesized_enum(json, path, synthesized_enum),
        Type::SynthesizedShape(synthesized_shape) => json_to_teon_with_shape(json, path, synthesized_shape, main_namespace),
        Type::DeclaredSynthesizedShape(synthesized_shape_reference, model_type) => {
            if let Some(model_reference) = model_type.as_model_object() {
                let m = main_namespace.model_at_path(&model_reference.str_path()).unwrap();
                if let Some(shape) = m.cache.shape.get_declared(synthesized_shape_reference.string_path()) {
                    json_to_teon_with_shape(json, path, shape, main_namespace)
                } else {
                    Err(Error::invalid_request_pathed(path.clone(), "unexpected type"))?
                }
            } else {
                Err(Error::invalid_request_pathed(path.clone(), "unexpected type"))?
            }
        },
        _ => Err(Error::invalid_request_pathed(path.clone(), "unexpected type"))?,
    }
}

fn json_to_teon_with_synthesized_enum(json: &serde_json::Value, path: &KeyPath, synthesized_enum: &SynthesizedEnum) -> teo_result::Result<Value> {
    if json.is_string() {
        let name = json.as_str().unwrap();
        if synthesized_enum.keys.contains_str(name) {
            return Ok(Value::String(name.to_owned()));
        }
    }
    Err(Error::invalid_request_pathed(path.clone(), "expect string enum variant"))
}

pub fn json_to_teon_with_shape(json: &serde_json::Value, path: &KeyPath, shape: &SynthesizedShape, main_namespace: &Namespace) -> teo_result::Result<Value> {
    if let Some(object) = json.as_object() {
        let required_keys: BTreeSet<&str> = shape.iter().filter_map(|(k, v)| if !v.is_optional() {
            Some(k.as_str())
        } else {
            None
        }).collect();
        let all_keys: BTreeSet<&str> = shape.keys().map(AsRef::as_ref).collect();
        let passed_in_keys: BTreeSet<&str> = object.keys().map(AsRef::as_ref).collect();
        let unallowed_keys: Vec<&str> = passed_in_keys.difference(&all_keys).map(|s| *s).collect();
        if let Some(unallowed) = unallowed_keys.first() {
            return Err(Error::invalid_request_pathed(path + *unallowed, "unexpected key"));
        }
        let not_provided_keys: Vec<&str> = required_keys.difference(&passed_in_keys).map(|s| *s).collect();
        if let Some(not_provided) = not_provided_keys.first() {
            return Err(Error::invalid_request_pathed(path + *not_provided, "expect value"));
        }
        let map: IndexMap<String, Value> = object.iter().map(|(k, v)| Ok((k.to_owned(), json_to_teon(v, &(path + k), shape.get(k).unwrap(), main_namespace)?))).collect::<teo_result::Result<IndexMap<String, Value>>>()?;
        Ok(Value::Dictionary(map))
    } else {
        Err(Error::invalid_request_pathed(path.clone(), "unexpected value"))
    }

}

pub fn json_to_teon(json: &serde_json::Value, path: &KeyPath, input: &Type, main_namespace: &Namespace) -> teo_result::Result<Value> {
    json_to_teon_with_type(json, path, input, main_namespace)
}

fn collect_interface_shape(interface: &Interface, gens: &Vec<Type>) -> SynthesizedShape {
    interface.shape_from_generics(gens)
}

fn calculate_generics_map(
    generics_names: &Vec<String>,
    types: &Vec<Type>,
) -> BTreeMap<String, Type> {
    if generics_names.len() == types.len() {
        return generics_names.iter().enumerate().map(|(index, identifier)| (identifier.to_owned(), types.get(index).unwrap().clone())).collect();
    }
    btreemap!{}
}
