use indexmap::IndexMap;
use teo_parser::r#type::synthesized_shape::SynthesizedShape;
use teo_parser::r#type::Type;
use crate::value::range::Range;
use crate::value::Value;
use crate::namespace::extensions::SynthesizedShapeReferenceExtension;
use crate::namespace::Namespace;

pub trait TeonCast {
    fn cast(&self, target: Option<&Type>, namespace: &Namespace) -> Self;
}

impl TeonCast for Value {
    fn cast(&self, target: Option<&Type>, namespace: &Namespace) -> Self {
        if let Some(target) = target {
            do_cast(self, target, namespace)
        } else {
            self.clone()
        }
    }
}

fn do_cast(value: &Value, target: &Type, namespace: &Namespace) -> Value {
    match target {
        Type::Int => do_cast_to_int(value),
        Type::Int64 => do_cast_to_int64(value),
        Type::Float32 => do_cast_to_float32(value),
        Type::Float => do_cast_to_float(value),
        Type::Union(types) => {
            let mut result = value.clone();
            for t in types {
                result = do_cast(&result, t, namespace);
            }
            result
        }
        Type::Enumerable(enumerable) => {
            if let Some(array) = value.as_array() {
                Value::Array(array.iter().map(|v| do_cast(v, enumerable.as_ref(), namespace)).collect())
            } else {
                do_cast(value, enumerable.as_ref(), namespace)
            }
        }
        Type::Optional(inner) => do_cast(value, inner.as_ref(), namespace),
        Type::Array(inner) => {
            if let Some(array) = value.as_array() {
                Value::Array(array.iter().map(|v| do_cast(v, inner.as_ref(), namespace)).collect())
            } else {
                value.clone()
            }
        }
        Type::Dictionary(inner) => {
            if let Some(dictionary) = value.as_dictionary() {
                Value::Dictionary(dictionary.iter().map(|(k, v)| (k.clone(), do_cast(v, inner.as_ref(), namespace))).collect())
            } else {
                value.clone()
            }
        }
        Type::Tuple(types) => {
            let undetermined = Type::Undetermined;
            if let Some(array) = value.as_array() {
                Value::Tuple(array.iter().enumerate().map(|(i, v)| do_cast(v, types.get(i).unwrap_or(&undetermined), namespace)).collect())
            } else if let Some(array) = value.as_tuple() {
                Value::Tuple(array.iter().enumerate().map(|(i, v)| do_cast(v, types.get(i).unwrap_or(&undetermined), namespace)).collect())
            } else {
                value.clone()
            }
        }
        Type::Range(inner) => {
            if let Some(range) = value.as_range() {
                Value::Range(Range {
                    start: Box::new(do_cast(range.start.as_ref(), inner.as_ref(), namespace)),
                    end: Box::new(do_cast(range.end.as_ref(), inner.as_ref(), namespace)),
                    closed: range.closed
                })
            } else {
                value.clone()
            }
        }
        Type::SynthesizedShape(shape) => {
            if let Some(dictionary) = value.as_dictionary() {
                Value::Dictionary(do_cast_shape(dictionary, shape, namespace))
            } else {
                value.clone()
            }
        }
        Type::SynthesizedShapeReference(reference) => {
            if let Some(definition) = reference.fetch_synthesized_definition_for_namespace(namespace) {
                do_cast(value, definition, namespace)
            } else {
                value.clone()
            }
        }
        Type::InterfaceObject(reference, gens) => {
            if let Some(dictionary) = value.as_dictionary() {
                let interface = namespace.interface_at_path(&reference.str_path()).unwrap();
                let shape = interface.shape_from_generics(gens);
                Value::Dictionary(do_cast_shape(dictionary, &shape, namespace))
            } else {
                value.clone()
            }
        }
        _ => value.clone(),
    }
}

fn do_cast_to_int(value: &Value) -> Value {
    match value {
        Value::Float(f) => Value::Int(*f as i32),
        Value::Float32(f) => Value::Int(*f as i32),
        Value::Int64(i) => Value::Int(*i as i32),
        _ => value.clone()
    }
}

fn do_cast_to_int64(value: &Value) -> Value {
    match value {
        Value::Float(f) => Value::Int64(*f as i64),
        Value::Float32(f) => Value::Int64(*f as i64),
        Value::Int(i) => Value::Int64(*i as i64),
        _ => value.clone(),
    }
}

fn do_cast_to_float32(value: &Value) -> Value {
    match value {
        Value::Float(f) => Value::Float32(*f as f32),
        Value::Int(i) => Value::Float32(*i as f32),
        Value::Int64(i) => Value::Float32(*i as f32),
        _ => value.clone(),
    }
}

fn do_cast_to_float(value: &Value) -> Value {
    match value {
        Value::Float32(f) => Value::Float(*f as f64),
        Value::Int(i) => Value::Float(*i as f64),
        Value::Int64(i) => Value::Float(*i as f64),
        _ => value.clone(),
    }
}

fn do_cast_shape(dictionary: &IndexMap<String, Value>, shape: &SynthesizedShape, namespace: &Namespace) -> IndexMap<String, Value> {
    let undetermined = Type::Undetermined;
    dictionary.iter().map(|(k, v)| (k.clone(), do_cast(v, shape.get(k).unwrap_or(&undetermined), namespace))).collect()
}