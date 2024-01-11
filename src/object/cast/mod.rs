use teo_parser::r#type::Type;
use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::Value;

pub trait TeonCast {
    fn cast(&self, target: Option<&Type>) -> Self;
}

impl TeonCast for Value {
    fn cast(&self, target: Option<&Type>) -> Self {
        if let Some(target) = target {
            do_cast(self, target)
        } else {
            self.clone()
        }
    }
}

fn do_cast(value: &Value, target: &Type) -> Value {
    match target {
        Type::Int => do_cast_to_int(value),
        Type::Int64 => do_cast_to_int64(value),
        Type::Float32 => do_cast_to_float32(value),
        Type::Float => do_cast_to_float(value),
        Type::EnumVariant(_) => do_cast_to_enum_variant(value),
        Type::Union(types) => {

        }
        Type::Enumerable(enumerable) => {

        }
        Type::Optional(inner) => do_cast(value, inner.as_ref()),
        Type::Array(_) => {}
        Type::Dictionary(_) => {}
        Type::Tuple(_) => {}
        Type::Range(_) => {}
        Type::SynthesizedShape(_) => {}
        Type::SynthesizedShapeReference(_) => {}
        Type::SynthesizedEnum(_) => {}
        Type::SynthesizedEnumReference(_) => {}
        Type::SynthesizedInterfaceEnum(_) => {}
        Type::SynthesizedInterfaceEnumReference(_) => {}
        Type::InterfaceObject(_, _) => {}
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

fn do_cast_to_enum_variant(value: &Value) -> Value {
    match value {
        Value::String(s) => Value::EnumVariant(EnumVariant {
            value: s.clone(),
            args: None,
        }),
        _ => value.clone(),
    }
}