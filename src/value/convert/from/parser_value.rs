use teo_parser::value::Value as ParserValue;
use crate::value::interface_enum_variant::InterfaceEnumVariant;
use crate::value::option_variant::OptionVariant;
use crate::value::range::Range;
use crate::value::Value;

impl From<ParserValue> for Value {
    fn from(value: ParserValue) -> Self {
        match value {
            ParserValue::Null => Value::Null,
            ParserValue::Bool(v) => Value::Bool(v),
            ParserValue::Int(v) => Value::Int(v),
            ParserValue::Int64(v) => Value::Int64(v),
            ParserValue::Float32(v) => Value::Float32(v),
            ParserValue::Float(v) => Value::Float(v),
            ParserValue::Decimal(v) => Value::Decimal(v),
            ParserValue::ObjectId(v) => Value::ObjectId(v),
            ParserValue::String(v) => Value::String(v),
            ParserValue::Date(v) => Value::Date(v),
            ParserValue::DateTime(v) => Value::DateTime(v),
            ParserValue::Array(v) => Value::Array(v.iter().map(|v| v.into()).collect()),
            ParserValue::Dictionary(v) => Value::Dictionary(v.iter().map(|(k, v)| (k.to_owned(), v.into())).collect()),
            ParserValue::Range(v) => Value::Range(Range::from(v)),
            ParserValue::Tuple(v) => Value::Tuple(v.iter().map(|v| v.into()).collect()),
            ParserValue::OptionVariant(v) => Value::OptionVariant(OptionVariant::from(v)),
            ParserValue::InterfaceEnumVariant(v) => Value::InterfaceEnumVariant(InterfaceEnumVariant::from(v)),
            ParserValue::Regex(v) => Value::Regex(v),
        }
    }
}