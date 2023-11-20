use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;
use bigdecimal::{BigDecimal, FromPrimitive};
use bson::oid::ObjectId;
use chrono::{DateTime, NaiveDate, Utc};
use indexmap::{indexmap, IndexMap};
use itertools::Itertools;
use key_path::KeyPath;
use maplit::btreemap;
use teo_parser::r#type::synthesized_shape_reference::{SynthesizedShapeReference, SynthesizedShapeReferenceKind};
use teo_parser::r#type::Type;
use teo_teon::Value;
use teo_teon::types::file::File;
use crate::interface::Interface;
use crate::namespace::Namespace;
use crate::path::Error;
use crate::utils::ContainsStr;

pub fn fetch_input<'a>(reference: &SynthesizedShapeReference, main_namespace: &'a Namespace) -> &'a Type {
    let model = main_namespace.model_at_path(&reference.owner.as_model_object().unwrap().str_path()).unwrap();
    if reference.kind.requires_without() {
        model.cache.shape.get_without(reference.kind, reference.without.as_ref().unwrap()).unwrap()
    } else {
        model.cache.shape.get(reference.kind).unwrap()
    }
}

pub fn json_to_teon_with_type(json: &serde_json::Value, path: &KeyPath, t: &Type, main_namespace: &Namespace) -> crate::path::Result<Value> {
    match t {
        Type::Undetermined => Ok(Value::from(json)),
        Type::Ignored => Ok(Value::from(json)),
        Type::Any => Ok(Value::from(json)),
        Type::Null => if json.is_null() { Ok(Value::Null) } else { Err(Error::value_error(path.clone(), "expect null")) },
        Type::Bool => if json.is_boolean() { Ok(Value::from(json)) } else { Err(Error::value_error(path.clone(), "expect bool")) },
        Type::Int => if json.is_i64() { Ok(Value::Int(json.as_i64().unwrap() as i32)) } else { Err(Error::value_error(path.clone(), "expect int")) },
        Type::Int64 => if json.is_i64() { Ok(Value::Int64(json.as_i64().unwrap())) } else { Err(Error::value_error(path.clone(), "expect int 64")) },
        Type::Float32 => if json.is_f64() { Ok(Value::Float32(json.as_f64().unwrap() as f32)) } else { Err(Error::value_error(path.clone(), "expect float 32")) },
        Type::Float => if json.is_f64() { Ok(Value::Float(json.as_f64().unwrap())) } else { Err(Error::value_error(path.clone(), "expect float")) },
        Type::Decimal => if json.is_string() {
            Ok(Value::Decimal(match BigDecimal::from_str(json.as_str().unwrap()) {
                Ok(s) => s,
                Err(_) => Err(Error::value_error(path.clone(), "string is not valid decimal"))?,
            }))
        } else if json.is_number() {
            if let Some(f) = json.as_f64() {
                Ok(Value::Decimal(match BigDecimal::from_f64(f) {
                    Some(s) => s,
                    None => Err(Error::value_error(path.clone(), "number is not valid decimal"))?,
                }))
            } else if let Some(i) = json.as_i64() {
                Ok(Value::Decimal(match BigDecimal::from_i64(i) {
                    Some(s) => s,
                    None => Err(Error::value_error(path.clone(), "number is not valid decimal"))?,
                }))
            } else {
                unreachable!()
            }
        } else {
            Err(Error::value_error(path.clone(), "expect string or number which represents decimal"))
        }
        Type::String => if json.is_string() { Ok(Value::String(json.as_str().unwrap().to_owned())) } else { Err(Error::value_error(path.clone(), "expect string")) },
        Type::ObjectId => if json.is_string() {
            Ok(Value::ObjectId(match ObjectId::parse_str(json.as_str().unwrap()) {
                Ok(s) => s,
                Err(_) => Err(Error::value_error(path.clone(), "string is not valid object id"))?,
            }))
        } else {
            Err(Error::value_error(path.clone(), "expect string represents object id"))
        }
        Type::Date => if json.is_string() {
            Ok(Value::Date(match NaiveDate::parse_from_str(json.as_str().unwrap(), "%Y-%m-%d") {
                Ok(s) => s,
                Err(_) => Err(Error::value_error(path.clone(), "string is not valid date"))?,
            }))
        } else {
            Err(Error::value_error(path.clone(), "expect string represents date"))
        }
        Type::DateTime => if json.is_string() {
            Ok(Value::DateTime(match DateTime::parse_from_rfc3339(json.as_str().unwrap()) {
                Ok(d) => d.with_timezone(&Utc),
                Err(_) => Err(Error::value_error(path.clone(), "string is not valid datetime"))?,
            }))
        } else {
            Err(Error::value_error(path.clone(), "expect string represents datetime"))
        }
        Type::File => Ok(Value::File(match File::try_from(json) {
            Ok(f) => f,
            Err(_) => Err(Error::value_error(path.clone(), "invalid file input"))?,
        })),
        Type::Enumerable(inner) => {
            if let Some(json_array) = json.as_array() {
                let values: Vec<Value> = json_array.iter().enumerate().map(|(i, j)| Ok(json_to_teon_with_type(j, &(path + i), inner.as_ref(), main_namespace)?)).collect::<crate::path::Result<Vec<Value>>>()?;
                Ok(Value::Array(values))
            } else {
                Ok(Value::Array(vec![json_to_teon_with_type(json, path, inner.as_ref(), main_namespace)?]))
            }
        }
        Type::Array(inner) => {
            if let Some(json_array) = json.as_array() {
                let values: Vec<Value> = json_array.iter().enumerate().map(|(i, j)| json_to_teon_with_type(j, &(path + i), inner.as_ref(), main_namespace)).collect::<crate::path::Result<Vec<Value>>>()?;
                Ok(Value::Array(values))
            } else {
                Err(Error::value_error(path.clone(), "expect array"))
            }
        }
        Type::Dictionary(inner) => {
            if let Some(json_object) = json.as_object() {
                let values: IndexMap<String, Value> = json_object.iter().map(|(k, j)| Ok((k.clone(), json_to_teon_with_type(j, &(path + k), inner.as_ref(), main_namespace)?))).collect::<crate::path::Result<IndexMap<String, Value>>>()?;
                Ok(Value::Dictionary(values))
            } else {
                Err(Error::value_error(path.clone(), "expect dictionary"))
            }
        }
        Type::Tuple(_) => Err(Error::value_error(path.clone(), "unexpected type"))?,
        Type::Range(_) => Err(Error::value_error(path.clone(), "unexpected type"))?,
        Type::Union(inners) => {
            for inner in inners {
                if let Ok(result) = json_to_teon_with_type(json, path, inner, main_namespace) {
                    return Ok(result);
                }
            }
            Err(Error::value_error(path.clone(), "unexpected value"))
        }
        Type::EnumVariant(reference) => if json.is_string() {
            let e = main_namespace.enum_at_path(&reference.str_path()).unwrap();
            if e.cache.member_names.contains_str(json.as_str().unwrap()) {
                Ok(Value::String(json.as_str().unwrap().to_owned()))
            } else {
                Err(Error::value_error(path.clone(), "expect enum member"))
            }
        } else {
            Err(Error::value_error(path.clone(), "expect string represents enum member"))
        }
        Type::InterfaceObject(reference, gens) => {
            let i = main_namespace.interface_at_path(&reference.str_path()).unwrap();
            let shapes = collect_interface_shapes(i, gens, main_namespace);
            json_to_teon_with_shapes(json, path, shapes, main_namespace)
        }
        Type::Optional(inner) => {
            if json.is_null() {
                Ok(Value::Null)
            } else {
                json_to_teon_with_type(json, path, inner.as_ref(), main_namespace)
            }
        },
        Type::SynthesizedShapeReference(shape_reference) => {
            let input = fetch_input(shape_reference, main_namespace).unwrap();
            json_to_teon(json, path, input.as_ref(), main_namespace)
        },
        _ => Err(Error::value_error(path.clone(), "unexpected type"))?,
    }
}

pub fn json_to_teon_with_shapes(json: &serde_json::Value, path: &KeyPath, shapes: Vec<&Shape>, main_namespace: &Namespace) -> crate::path::Result<Value> {
    if let Some(object) = json.as_object() {
        let combined = if shapes.len() == 1 {
            Cow::Borrowed(*shapes.first().unwrap())
        } else {
            Cow::Owned(shapes.iter().fold(Shape::new(indexmap! {}), |mut item1, item2| {
                item1.extend((*item2).clone().into_iter());
                item1
            }))
        };
        let required_keys: BTreeSet<&str> = combined.as_ref().iter().filter_map(|(k, v)| if !v.is_optional() {
            Some(k.as_str())
        } else {
            None
        }).collect();
        let all_keys: BTreeSet<&str> = combined.as_ref().keys().map(AsRef::as_ref).collect();
        let passed_in_keys: BTreeSet<&str> = object.keys().map(AsRef::as_ref).collect();
        let unallowed_keys: Vec<&str> = passed_in_keys.difference(&all_keys).map(|s| *s).collect();
        if let Some(unallowed) = unallowed_keys.first() {
            return Err(Error::value_error(path + *unallowed, "unexpected key"));
        }
        let not_provided_keys: Vec<&str> = required_keys.difference(&passed_in_keys).map(|s| *s).collect();
        if let Some(not_provided) = not_provided_keys.first() {
            return Err(Error::value_error(path + *not_provided, "expect value"));
        }
        let map: IndexMap<String, Value> = object.iter().map(|(k, v)| Ok((k.to_owned(), json_to_teon(v, &(path + k), combined.as_ref().get(k).unwrap(), main_namespace)?))).collect::<crate::path::Result<IndexMap<String, Value>>>()?;
        Ok(Value::Dictionary(map))
    } else {
        Err(Error::value_error(path.clone(), "unexpected value"))
    }

}

pub fn json_to_teon(json: &serde_json::Value, path: &KeyPath, input: &Input, main_namespace: &Namespace) -> crate::path::Result<Value> {
    match input {
        Input::Undetermined => Ok(Value::from(json)),
        Input::Or(inputs) => {
            for i in inputs {
                if let Ok(result) = json_to_teon(json, path, i, main_namespace) {
                    return Ok(result);
                }
            }
            Err(Error::value_error(path.clone(), "unexpected value"))
        }
        Input::Shape(shape) => {
            json_to_teon_with_shapes(json, path, vec![shape], main_namespace)
        }
        Input::Type(t) => {
            json_to_teon_with_type(json, path, t, main_namespace)
        }
        Input::SynthesizedEnum(e) => {
            if let Some(str) = json.as_str() {
                if e.members.keys().contains(&str.to_owned()) {
                    Ok(Value::String(str.to_owned()))
                } else {
                    Err(Error::value_error(path.clone(), "unexpected value"))
                }
            } else {
                Err(Error::value_error(path.clone(), "unexpected value"))
            }
        }
    }
}

fn collect_interface_shapes<'a>(interface: &'a Interface, gens: &Vec<Type>, namespace: &'a Namespace) -> Vec<&'a Shape> {
    let mut result = vec![];
    let shape = interface.cache.shape.map.get(gens).unwrap().as_shape().unwrap();
    result.push(shape);
    let map = calculate_generics_map(&interface.generic_names, gens);
    for extend in &interface.extends {
        if let Some((reference, inner_gens)) = extend.as_interface_object() {
            let inner_interface = namespace.interface_at_path(&reference.string_path().iter().map(AsRef::as_ref).collect()).unwrap();
            result.extend(collect_interface_shapes(inner_interface, &inner_gens.iter().map(|t| t.replace_generics(&map)).collect(), namespace));
        }
    }
    result
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
