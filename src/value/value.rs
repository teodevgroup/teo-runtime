use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::mem;
use std::ops::{Add, Div, Mul, Sub, Rem, Neg, BitAnd, BitXor, BitOr, Not, Shl, Shr};
use std::str::FromStr;
use chrono::prelude::{DateTime, Utc};
use indexmap::IndexMap;
use bson::oid::ObjectId;
use chrono::{NaiveDate, SecondsFormat};
use regex::Regex;
use bigdecimal::{BigDecimal, Zero};
use itertools::Itertools;
use teo_parser::r#type::synthesized_shape::SynthesizedShape;
use teo_parser::r#type::Type;
use super::file::File;
use super::range::Range;
use super::index::Index;
use teo_result::{Error, Result};
use crate::{model, r#struct};
use crate::namespace::extensions::SynthesizedShapeReferenceExtension;
use crate::namespace::Namespace;
use crate::pipeline::Pipeline;
use super::interface_enum_variant::InterfaceEnumVariant;
use super::option_variant::OptionVariant;

// Code from this file is inspired from serde json
// https://github.com/serde-rs/json/blob/master/src/value/mod.rs

/// Represents any valid Teon value.
///
#[derive(Debug, Clone)]
pub enum Value {

    /// Represents a Teon null value.
    ///
    /// ```
    /// # use teo_runtime::teon;
    /// #
    /// let v = teon!(null);
    /// ```
    Null,

    /// Represents a Teon Bool.
    ///
    /// ```
    /// # use teo_runtime::teon;
    /// #
    /// let v = teon!(true);
    /// ```
    Bool(bool),

    /// Represents a Teon Int.
    ///
    /// ```
    /// # use teo_runtime::teon;
    /// #
    /// let v = teon!(12_i32);
    /// ```
    Int(i32),

    /// Represents a Teon Int64.
    ///
    /// ```
    /// # use teo_runtime::teon;
    /// #
    /// let v = teon!(12_i64);
    /// ```
    Int64(i64),

    /// Represents a Teon Float32.
    ///
    /// ```
    /// # use teo_runtime::teon;
    /// #
    /// let v = teon!(12.5_f32);
    /// ```
    Float32(f32),

    /// Represents a Teon Float.
    ///
    /// ```
    /// # use teo_runtime::teon;
    /// #
    /// let v = teon!(12.5_f64);
    /// ```
    Float(f64),

    /// Represents a Teon Decimal.
    ///
    Decimal(BigDecimal),

    /// Represents a Teon ObjectId.
    ///
    ObjectId(ObjectId),

    /// Represents a Teon String.
    ///
    String(String),

    /// Represents a Teon Date.
    ///
    Date(NaiveDate),

    /// Represents a Teon DateTime.
    ///
    DateTime(DateTime<Utc>),

    /// Represents a Teon Array.
    ///
    Array(Vec<Value>),

    /// Represents a Teon btree_dictionary.
    ///
    Dictionary(IndexMap<String, Value>),

    /// Represents a Teon Range.
    ///
    Range(Range),

    /// Represents a Teon Tuple.
    ///
    Tuple(Vec<Value>),

    /// Represents a Teon option variant.
    ///
    InterfaceEnumVariant(InterfaceEnumVariant),

    /// Represents a Teon option variant.
    ///
    OptionVariant(OptionVariant),

    /// Represents a Teon Regex.
    ///
    Regex(Regex),

    /// Represents a Teon File.
    ///
    File(File),

    /// Represents a model object.
    ///
    ModelObject(model::Object),

    /// Represents a struct object.
    ///
    StructObject(r#struct::Object),

    /// Represents a pipeline.
    ///
    Pipeline(Pipeline),

    /// Represents a type as value.
    ///
    Type(Type),
}

impl Value {

    // Access

    pub fn get<I: Index>(&self, index: I) -> Option<&Value> {
        index.index_into(self)
    }

    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Value> {
        index.index_into_mut(self)
    }

    // Value

    pub fn is_null(&self) -> bool {
        match self {
            Value::Null => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        self.as_bool().is_some()
    }

    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }

    pub fn is_int(&self) -> bool {
        self.as_int().is_some()
    }

    pub fn as_int(&self) -> Option<i32> {
        match *self {
            Value::Int(v) => Some(v),
            _ => None
        }
    }

    pub fn to_int(&self) -> Option<i32> {
        match *self {
            Value::Int(i) => Some(i),
            Value::Int64(i) => if i >= (i32::MAX as i64) {
                None
            } else {
                Some(i as i32)
            }
            _ => None
        }
    }

    pub fn is_int64(&self) -> bool {
        self.as_int64().is_some()
    }

    pub fn as_int64(&self) -> Option<i64> {
        match *self {
            Value::Int64(v) => Some(v),
            _ => None
        }
    }

    pub fn to_int64(&self) -> Option<i64> {
        match *self {
            Value::Int64(v) => Some(v),
            Value::Int(v) => Some(v as i64),
            _ => None,
        }
    }

    pub fn is_float32(&self) -> bool {
        self.as_float32().is_some()
    }

    pub fn as_float32(&self) -> Option<f32> {
        match *self {
            Value::Float32(v) => Some(v),
            _ => None
        }
    }

    pub fn to_float32(&self) -> Option<f32> {
        match *self {
            Value::Float32(v) => Some(v),
            Value::Float(v) => Some(v as f32),
            Value::Int(i) => Some(i as f32),
            Value::Int64(i) => Some(i as f32),
            _ => None,
        }
    }

    pub fn is_float(&self) -> bool {
        self.as_float().is_some()
    }

    pub fn as_float(&self) -> Option<f64> {
        match *self {
            Value::Float(v) => Some(v),
            _ => None
        }
    }

    pub fn to_float(&self) -> Option<f64> {
        match *self {
            Value::Int(v) => Some(v as f64),
            Value::Int64(v) => Some(v as f64),
            Value::Float32(v) => Some(v as f64),
            Value::Float(v) => Some(v),
            _ => None
        }
    }

    pub fn is_decimal(&self) -> bool {
        match *self {
            Value::Decimal(_) => true,
            _ => false,
        }
    }

    pub fn as_decimal(&self) -> Option<&BigDecimal> {
        match self {
            Value::Decimal(v) => Some(v),
            _ => None
        }
    }

    pub fn is_object_id(&self) -> bool {
        self.as_object_id().is_some()
    }

    pub fn as_object_id(&self) -> Option<&ObjectId> {
        match self {
            Value::ObjectId(o) => Some(o),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_date(&self) -> bool {
        self.as_date().is_some()
    }

    pub fn as_date(&self) -> Option<&NaiveDate> {
        match self {
            Value::Date(d) => Some(d),
            _ => None,
        }
    }

    pub fn is_datetime(&self) -> bool {
        self.as_datetime().is_some()
    }

    pub fn as_datetime(&self) -> Option<&DateTime<Utc>> {
        match self {
            Value::DateTime(d) => Some(d),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(vec) => Some(vec),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::Array(vec) => Some(vec),
            _ => None,
        }
    }

    pub fn into_array(self) -> Option<Vec<Value>> {
        match self {
            Value::Array(vec) => Some(vec),
            _ => None,
        }
    }

    pub fn is_dictionary(&self) -> bool {
        self.as_dictionary().is_some()
    }

    pub fn as_dictionary(&self) -> Option<&IndexMap<String, Value>> {
        match self {
            Value::Dictionary(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_dictionary_mut(&mut self) -> Option<&mut IndexMap<String, Value>> {
        match self {
            Value::Dictionary(map) => Some(map),
            _ => None,
        }
    }

    pub fn is_range(&self) -> bool {
        self.as_range().is_some()
    }

    pub fn as_range(&self) -> Option<&Range> {
        match self {
            Value::Range(r) => Some(r),
            _ => None,
        }
    }

    pub fn is_tuple(&self) -> bool {
        self.as_range().is_some()
    }

    pub fn as_tuple(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Tuple(t) => Some(t),
            _ => None,
        }
    }

    pub fn is_option_variant(&self) -> bool {
        self.as_option_variant().is_some()
    }

    pub fn as_option_variant(&self) -> Option<&OptionVariant> {
        match self {
            Value::OptionVariant(e) => Some(e),
            _ => None,
        }
    }

    pub fn is_interface_enum_variant(&self) -> bool {
        self.as_interface_enum_variant().is_some()
    }

    pub fn as_interface_enum_variant(&self) -> Option<&InterfaceEnumVariant> {
        match self {
            Value::InterfaceEnumVariant(e) => Some(e),
            _ => None,
        }
    }

    pub fn is_regexp(&self) -> bool {
        self.as_regexp().is_some()
    }

    pub fn as_regexp(&self) -> Option<&Regex> {
        match self {
            Value::Regex(r) => Some(r),
            _ => None,
        }
    }

    pub fn is_file(&self) -> bool {
        self.as_file().is_some()
    }

    pub fn as_file(&self) -> Option<&File> {
        match self {
            Value::File(f) => Some(f),
            _ => None,
        }
    }

    pub fn is_model_object(&self) -> bool {
        self.as_model_object().is_some()
    }

    pub fn as_model_object(&self) -> Option<&model::Object> {
        match self {
            Value::ModelObject(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_struct_object(&self) -> bool {
        self.as_struct_object().is_some()
    }

    pub fn as_struct_object(&self) -> Option<&r#struct::Object> {
        match self {
            Value::StructObject(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_pipeline(&self) -> bool {
        self.as_pipeline().is_some()
    }

    pub fn as_pipeline(&self) -> Option<&Pipeline> {
        match self {
            Value::Pipeline(p) => Some(p),
            _ => None,
        }
    }

    pub fn is_type(&self) -> bool {
        self.as_type().is_some()
    }

    pub fn as_type(&self) -> Option<&Type> {
        match self {
            Value::Type(p) => Some(p),
            _ => None,
        }
    }

    // Compound queries

    pub fn is_any_int(&self) -> bool {
        match *self {
            Value::Int(_) | Value::Int64(_) => true,
            _ => false,
        }
    }

    pub fn is_any_float(&self) -> bool {
        match *self {
            Value::Float32(_) | Value::Float(_) => true,
            _ => false,
        }
    }

    pub fn is_any_int_or_float(&self) -> bool {
        self.is_any_int() || self.is_any_float()
    }

    pub fn is_any_number(&self) -> bool {
        self.is_any_int() || self.is_any_float() || self.is_decimal()
    }

    pub fn to_usize(&self) -> Option<usize> {
        match *self {
            Value::Int(n) => Some(n as usize),
            Value::Int64(n) => Some(n as usize),
            _ => None
        }
    }

    pub fn wrap_into_vec<T>(self) -> Result<Vec<T>> where T: TryFrom<Value>, T::Error: Display {
        match self {
            Value::Array(array) => {
                let mut retval = vec![];
                for v in array {
                    match T::try_from(v) {
                        Ok(v) => retval.push(v),
                        Err(e) => Err(Error::new(format!("{}", e)))?,
                    }
                }
                Ok(retval)
            },
            _ => match T::try_from(self) {
                Ok(v) => Ok(vec![v]),
                Err(e) => Err(Error::new(format!("{}", e))),
            }
        }
    }

    /// Takes the value out of the `Value`, leaving a `Null` in its place.
    ///
    pub fn take(&mut self) -> Value {
        mem::replace(self, Value::Null)
    }

    // Type hint

    pub fn type_hint(&self) -> &str {
        match self {
            Value::Null => "Null",
            Value::Bool(_) => "Bool",
            Value::Int(_) => "Int",
            Value::Int64(_) => "Int64",
            Value::Float32(_) => "Float32",
            Value::Float(_) => "Float",
            Value::Decimal(_) => "Decimal",
            Value::ObjectId(_) => "ObjectId",
            Value::String(_) => "String",
            Value::Date(_) => "Date",
            Value::DateTime(_) => "DateTime",
            Value::Array(_) => "Array",
            Value::Dictionary(_) => "Dictionary",
            Value::Range(_) => "Range",
            Value::Tuple(_) => "Tuple",
            Value::InterfaceEnumVariant(_) => "EnumVariant",
            Value::OptionVariant(_) => "OptionVariant",
            Value::Regex(_) => "RegExp",
            Value::File(_) => "File",
            Value::ModelObject(_) => "ModelObject",
            Value::StructObject(_) => "StructObject",
            Value::Pipeline(_) => "Pipeline",
            Value::Type(_) => "Type",
        }
    }

    pub fn recip(&self) -> Result<Value> {
        Ok(match self {
            Value::Int(n) => Value::Float((*n as f64).recip()),
            Value::Int64(n) => Value::Float((*n as f64).recip()),
            Value::Float32(n) => Value::Float32((*n).recip()),
            Value::Float(n) => Value::Float((*n).recip()),
            Value::Decimal(n) => Value::Decimal(BigDecimal::from_str("1").unwrap() / n),
            _ => Err(Error::new("recip: value is not number"))?
        })
    }

    pub fn normal_not(&self) -> Value {
        Value::Bool(match self {
            Value::Null => true,
            Value::Bool(b) => !b,
            Value::Int(i) => i.is_zero(),
            Value::Int64(i) => i.is_zero(),
            Value::Float32(f) => f.is_zero(),
            Value::Float(f) => f.is_zero(),
            Value::Decimal(d) => d.is_zero(),
            Value::ObjectId(_) => false,
            Value::String(s) => s.is_empty(),
            Value::Date(_) => false,
            Value::DateTime(_) => false,
            Value::Array(a) => a.is_empty(),
            Value::Dictionary(d) => d.is_empty(),
            Value::Range(_) => false,
            Value::Tuple(_) => false,
            Value::InterfaceEnumVariant(e) => e.normal_not(),
            Value::OptionVariant(o) => o.normal_not(),
            Value::Regex(_) => false,
            Value::File(_) => false,
            Value::Pipeline(_) => false,
            Value::ModelObject(_) => false,
            Value::StructObject(_) => false,
            Value::Type(_) => false,
        })
    }

    pub fn and<'a>(&'a self, rhs: &'a Value) -> &'a Value {
        if self.normal_not().is_false() {
            rhs
        } else {
            self
        }
    }

    pub fn or<'a>(&'a self, rhs: &'a Value) -> &'a Value {
        if self.normal_not().is_false() {
            self
        } else {
            rhs
        }
    }

    pub fn is_false(&self) -> bool {
        self.is_bool() && self.as_bool().unwrap() == false
    }

    pub fn is_true(&self) -> bool {
        self.is_bool() && self.as_bool().unwrap() == true
    }

    pub fn try_into_err_prefix<T, E>(self, prefix: impl AsRef<str>) -> Result<T> where Error: From<E>, T: TryFrom<Value, Error = E> {
        let result: std::result::Result<T, E> = self.try_into();
        match result {
            Ok(t) => Ok(t),
            Err(e) => Err(Error::new(format!("{}: {}", prefix.as_ref(), Error::from(e)))),
        }
    }

    fn try_into_err_message_inner<T, E>(self) -> Result<T> where Error: From<E>, T: TryFrom<Value, Error = E> {
        Ok(self.try_into()?)
    }

    pub fn try_into_err_message<T, E>(self, message: impl AsRef<str>) -> Result<T> where Error: From<E>, T: TryFrom<Value, Error = E> {
        let result: Result<T> = self.try_into_err_message_inner();
        match result {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::new(message.as_ref())),
        }
    }

    pub fn try_ref_into_err_prefix<'a, T: 'a, E>(&'a self, prefix: impl AsRef<str>) -> Result<T> where Error: From<E>, T: TryFrom<&'a Value, Error = E> {
        let result: std::result::Result<T, E> = self.try_into();
        match result {
            Ok(t) => Ok(t),
            Err(e) => Err(Error::new(format!("{}: {}", prefix.as_ref(), Error::from(e)))),
        }
    }

    pub fn try_ref_into_err_message<'a, T: 'a, E>(&'a self, message: impl AsRef<str>) -> Result<T> where Error: From<E>, T: TryFrom<&'a Value, Error = E> {
        let result: std::result::Result<T, E> = self.try_into();
        match result {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::new(message.as_ref())),
        }
    }

    pub(crate) fn cast(&self, target: Option<&Type>, namespace: &Namespace) -> Self {
        if let Some(target) = target {
            do_cast(self, target, namespace)
        } else {
            self.clone()
        }
    }

    pub(crate) fn default_struct_path(&self) -> Result<Vec<&'static str>> {
        Ok(match self {
            Value::Null => vec!["std", "Null"],
            Value::Bool(_) => vec!["std", "Bool"],
            Value::Int(_) => vec!["std", "Int"],
            Value::Int64(_) => vec!["std", "Int64"],
            Value::Float32(_) => vec!["std", "Float32"],
            Value::Float(_) => vec!["std", "Float"],
            Value::Decimal(_) => vec!["std", "Decimal"],
            Value::ObjectId(_) => vec!["std", "ObjectId"],
            Value::String(_) => vec!["std", "String"],
            Value::Date(_) => vec!["std", "Date"],
            Value::DateTime(_) => vec!["std", "DateTime"],
            Value::Array(_) => vec!["std", "Array"],
            Value::Dictionary(_) => vec!["std", "Dictionary"],
            Value::Range(_) => vec!["std", "Range"],
            Value::Tuple(_) => Err(Error::new("tuple struct is not supported"))?,
            Value::Regex(_) => vec!["std", "Regex"],
            Value::File(_) => vec!["std", "File"],
            Value::OptionVariant(_) => Err(Error::new("option variant struct is not supported"))?,
            _ => Err(Error::new("primitive struct is not supported for this type"))?,
        })
    }


}

impl Default for Value {
    fn default() -> Value {
        Value::Null
    }
}

fn check_enum_operands(name: &str, lhs: &Value, rhs: &Value) -> Result<()> {
    if let (Some(_), Some(_)) = (lhs.as_option_variant(), rhs.as_option_variant()) {
        Ok(())
    } else {
        Err(operands_error_message(lhs, rhs, name))
    }
}

fn operand_error_message(operand: &Value, name: &str) -> Error {
    Error::new(format!("cannot {name} {}", operand.type_hint()))
}

fn check_operands<F>(lhs: &Value, rhs: &Value, name: &str, matcher: F) -> Result<()> where F: Fn(&Value) -> bool {
    let matcher_wrapper = |value: &Value| {
        (&matcher)(value)
    };
    if !matcher_wrapper(lhs) || !matcher_wrapper(rhs) {
        return Err(operands_error_message(lhs, rhs, name));
    }
    Ok(())
}

fn operands_error_message(lhs: &Value, rhs: &Value, name: &str) -> Error {
    Error::new(format!("cannot {name} {} with {}", lhs.type_hint(), rhs.type_hint()))
}

impl Add for &Value {

    type Output = Result<Value>;

    fn add(self, rhs: Self) -> Self::Output {
        Ok(match self {
            Value::Int(v) => {
                check_operands(&self, &rhs, "add", |v| v.is_any_int())?;
                Value::Int(v + rhs.to_int().unwrap())
            },
            Value::Int64(v) => {
                check_operands(&self, &rhs, "add", |v| v.is_any_int())?;
                Value::Int64(v + rhs.to_int64().unwrap())
            },
            Value::Float32(v) => {
                check_operands(&self, &rhs, "add", |v| v.is_any_int_or_float())?;
                Value::Float32(v + rhs.to_float32().unwrap())
            },
            Value::Float(v) => {
                check_operands(&self, &rhs, "add", |v| v.is_any_int_or_float())?;
                Value::Float(v + rhs.to_float().unwrap())
            },
            Value::Decimal(d) => {
                check_operands(&self, &rhs, "add", |v| v.is_decimal())?;
                Value::Decimal(d + rhs.as_decimal().unwrap())
            },
            Value::String(s) => {
                check_operands(&self, &rhs, "add", |v| v.is_string())?;
                Value::String(s.to_owned() + rhs.as_str().unwrap())
            }
            _ => Err(operands_error_message(self, rhs, "add"))?,
        })
    }
}

impl Sub for &Value {

    type Output = Result<Value>;

    fn sub(self, rhs: Self) -> Self::Output {
        Ok(match self {
            Value::Int(v) => {
                check_operands(&self, &rhs, "sub", |v| v.is_any_int())?;
                Value::Int(v - rhs.to_int().unwrap())
            },
            Value::Int64(v) => {
                check_operands(&self, &rhs, "sub", |v| v.is_any_int())?;
                Value::Int64(v - rhs.to_int64().unwrap())
            },
            Value::Float32(v) => {
                check_operands(&self, &rhs, "sub", |v| v.is_any_int_or_float())?;
                Value::Float32(v - rhs.to_float32().unwrap())
            },
            Value::Float(v) => {
                check_operands(&self, &rhs, "sub", |v| v.is_any_int_or_float())?;
                Value::Float(v - rhs.to_float().unwrap())
            },
            Value::Decimal(d) => {
                check_operands(&self, &rhs, "sub", |v| v.is_decimal())?;
                Value::Decimal(d - rhs.as_decimal().unwrap())
            },
            _ => Err(operands_error_message(self, rhs, "sub"))?,
        })
    }
}

impl Mul for &Value {

    type Output = Result<Value>;

    fn mul(self, rhs: Self) -> Self::Output {
        Ok(match self {
            Value::Int(v) => {
                check_operands(&self, &rhs, "mul", |v| v.is_any_int())?;
                Value::Int(v * rhs.to_int().unwrap())
            },
            Value::Int64(v) => {
                check_operands(&self, &rhs, "mul", |v| v.is_any_int())?;
                Value::Int64(v * rhs.to_int64().unwrap())
            },
            Value::Float32(v) => {
                check_operands(&self, &rhs, "mul", |v| v.is_any_int_or_float())?;
                Value::Float32(v * rhs.to_float32().unwrap())
            },
            Value::Float(v) => {
                check_operands(&self, &rhs, "mul", |v| v.is_any_int_or_float())?;
                Value::Float(v * rhs.to_float().unwrap())
            },
            Value::Decimal(d) => {
                check_operands(&self, &rhs, "mul", |v| v.is_decimal())?;
                Value::Decimal(d * rhs.as_decimal().unwrap())
            },
            _ => Err(operands_error_message(self, rhs, "mul"))?,
        })
    }
}

impl Div for &Value {

    type Output = Result<Value>;

    fn div(self, rhs: Self) -> Self::Output {
        Ok(match self {
            Value::Int(v) => {
                check_operands(&self, &rhs, "div", |v| v.is_any_int())?;
                Value::Int(v / rhs.to_int().unwrap())
            },
            Value::Int64(v) => {
                check_operands(&self, &rhs, "div", |v| v.is_any_int())?;
                Value::Int64(v / rhs.to_int64().unwrap())
            },
            Value::Float32(v) => {
                check_operands(&self, &rhs, "div", |v| v.is_any_int_or_float())?;
                Value::Float32(v / rhs.to_float32().unwrap())
            },
            Value::Float(v) => {
                check_operands(&self, &rhs, "div", |v| v.is_any_int_or_float())?;
                Value::Float(v / rhs.to_float().unwrap())
            },
            Value::Decimal(d) => {
                check_operands(&self, &rhs, "div", |v| v.is_decimal())?;
                Value::Decimal(d / rhs.as_decimal().unwrap())
            },
            _ => Err(operands_error_message(self, rhs, "div"))?,
        })
    }
}

impl Rem for &Value {

    type Output = Result<Value>;

    fn rem(self, rhs: Self) -> Self::Output {
        Ok(match self {
            Value::Int(v) => {
                check_operands(&self, &rhs, "rem", |v| v.is_any_int())?;
                Value::Int(v % rhs.to_int().unwrap())
            },
            Value::Int64(v) => {
                check_operands(&self, &rhs, "rem", |v| v.is_any_int())?;
                Value::Int64(v % rhs.to_int64().unwrap())
            },
            Value::Float32(v) => {
                check_operands(&self, &rhs, "rem", |v| v.is_any_int_or_float())?;
                Value::Float32(v % rhs.to_float32().unwrap())
            },
            Value::Float(v) => {
                check_operands(&self, &rhs, "rem", |v| v.is_any_int_or_float())?;
                Value::Float(v % rhs.to_float().unwrap())
            },
            Value::Decimal(d) => {
                check_operands(&self, &rhs, "rem", |v| v.is_decimal())?;
                Value::Decimal(d % rhs.as_decimal().unwrap())
            },
            _ => Err(operands_error_message(self, rhs, "rem"))?,
        })
    }
}

impl Neg for &Value {

    type Output = Result<Value>;

    fn neg(self) -> Self::Output {
        Ok(match self {
            Value::Int(val) => Value::Int(-*val),
            Value::Int64(val) => Value::Int64(-*val),
            Value::Float32(val) => Value::Float32(-*val),
            Value::Float(val) => Value::Float(-*val),
            Value::Decimal(val) => Value::Decimal(val.neg()),
            _ => Err(operand_error_message(self, "neg"))?,
        })
    }
}

impl Shl for &Value {

    type Output = Result<Value>;

    fn shl(self, rhs: Self) -> Self::Output {
        Ok(match self {
            Value::Int(v) => {
                check_operands(&self, rhs, "shift left", |v| v.is_any_int())?;
                Value::Int(v << rhs.as_int().unwrap())
            },
            Value::Int64(v) => {
                check_operands(&self, rhs, "shift left", |v| v.is_any_int())?;
                Value::Int64(v << rhs.as_int64().unwrap())
            },
            _ => Err(operand_error_message(self, "shift left"))?,
        })
    }
}

impl Shr for &Value {

    type Output = Result<Value>;

    fn shr(self, rhs: Self) -> Self::Output {
        Ok(match self {
            Value::Int(v) => {
                check_operands(&self, rhs, "shift right", |v| v.is_any_int())?;
                Value::Int(v >> rhs.as_int().unwrap())
            },
            Value::Int64(v) => {
                check_operands(&self, rhs, "shift right", |v| v.is_any_int())?;
                Value::Int64(v >> rhs.as_int64().unwrap())
            },
            _ => Err(operand_error_message(self, "shift right"))?,
        })
    }
}

impl BitAnd for &Value {

    type Output = Result<Value>;

    fn bitand(self, rhs: Self) -> Self::Output {
        Ok(match self {
            Value::Int(v) => {
                check_operands(&self, rhs, "bitand", |v| v.is_any_int())?;
                Value::Int(v & rhs.as_int().unwrap())
            },
            Value::Int64(v) => {
                check_operands(&self, rhs, "bitand", |v| v.is_any_int())?;
                Value::Int64(v & rhs.as_int64().unwrap())
            },
            Value::OptionVariant(e) => {
                check_enum_operands("bitand", self, rhs)?;
                Value::OptionVariant((e & rhs.as_option_variant().unwrap())?)
            }
            _ => Err(operand_error_message(self, "bitand"))?,
        })
    }
}

impl BitXor for &Value {

    type Output = Result<Value>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Ok(match self {
            Value::Int(v) => {
                check_operands(&self, rhs, "bitxor", |v| v.is_any_int())?;
                Value::Int(v ^ rhs.as_int().unwrap())
            },
            Value::Int64(v) => {
                check_operands(&self, rhs, "bitxor", |v| v.is_any_int())?;
                Value::Int64(v ^ rhs.as_int64().unwrap())
            },
            Value::OptionVariant(e) => {
                check_enum_operands("bitxor", self, rhs)?;
                Value::OptionVariant((e ^ rhs.as_option_variant().unwrap())?)
            }
            _ => Err(operand_error_message(self, "bitxor"))?,
        })
    }
}

impl BitOr for &Value {

    type Output = Result<Value>;

    fn bitor(self, rhs: Self) -> Self::Output {
        Ok(match self {
            Value::Int(v) => {
                check_operands(&self, rhs, "bitor", |v| v.is_any_int())?;
                Value::Int(v | rhs.as_int().unwrap())
            },
            Value::Int64(v) => {
                check_operands(&self, rhs, "bitor", |v| v.is_any_int())?;
                Value::Int64(v | rhs.as_int64().unwrap())
            },
            Value::OptionVariant(e) => {
                check_enum_operands("bitor", self, rhs)?;
                Value::OptionVariant((e | rhs.as_option_variant().unwrap())?)
            }
            _ => Err(operand_error_message(self, "bitor"))?,
        })
    }
}

// This is bit neg
impl Not for &Value {

    type Output = Result<Value>;

    fn not(self) -> Self::Output {
        Ok(match self {
            Value::Int(val) => Value::Int(-*val),
            Value::Int64(val) => Value::Int64(-*val),
            Value::Float32(val) => Value::Float32(-*val),
            Value::Float(val) => Value::Float(-*val),
            Value::Decimal(val) => Value::Decimal(val.neg()),
            Value::OptionVariant(e) => Value::OptionVariant(e.not()),
            _ => Err(operand_error_message(self, "bitneg"))?,
        })
    }
}

impl PartialEq for Value {

    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        if self.is_any_int() && other.is_any_int() {
            return self.to_int64().unwrap() == other.to_int64().unwrap();
        }
        if self.is_any_int_or_float() && other.is_any_int_or_float() {
            return self.to_float().unwrap() == other.to_float().unwrap();
        }
        match (self, other) {
            (Null, Null) => true,
            (Bool(s), Bool(o)) => s == o,
            (Decimal(s), Decimal(o)) => s == o,
            (ObjectId(s), ObjectId(o)) => s == o,
            (String(s), String(o)) => s == o,
            (Date(s), Date(o)) => s == o,
            (DateTime(s), DateTime(o)) => s == o,
            (Array(s), Array(o)) => s == o,
            (Dictionary(s), Dictionary(o)) => s == o,
            (Range(s), Range(o)) => s == o,
            (Tuple(s), Tuple(o)) => s == o,
            (InterfaceEnumVariant(s), InterfaceEnumVariant(o)) => s == o,
            (OptionVariant(s), OptionVariant(o)) => s.value == o.value,
            (Regex(s), Regex(o)) => s.as_str() == o.as_str(),
            (File(s), File(o)) => s == o,
            _ => false,
        }
    }
}

impl PartialOrd for Value {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Value::*;
        if self.is_any_int() && other.is_any_int() {
            return self.to_int64().unwrap().partial_cmp(&other.to_int64().unwrap());
        }
        if self.is_any_int_or_float() && other.is_any_int_or_float() {
            return self.to_float().unwrap().partial_cmp(&other.to_float().unwrap());
        }
        match (self, other) {
            (Null, Null) => Some(Ordering::Equal),
            (Bool(s), Bool(o)) => s.partial_cmp(o),
            (Decimal(s), Decimal(o)) => s.partial_cmp(o),
            (ObjectId(s), ObjectId(o)) => s.partial_cmp(o),
            (String(s), String(o)) => s.partial_cmp(o),
            (Date(s), Date(o)) => s.partial_cmp(o),
            (DateTime(s), DateTime(o)) => s.partial_cmp(o),
            (Array(s), Array(o)) => s.partial_cmp(o),
            (Tuple(s), Tuple(o)) => s.partial_cmp(o),
            (InterfaceEnumVariant(s), InterfaceEnumVariant(o)) => s.value.partial_cmp(&o.value),
            (OptionVariant(s), OptionVariant(o)) => s.value.partial_cmp(&o.value),
            _ => None,
        }
    }
}

impl AsRef<Value> for Value {

    fn as_ref(&self) -> &Value {
        &self
    }
}

impl Display for Value {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => f.write_str("null"),
            Value::Bool(b) => Display::fmt(b, f),
            Value::Int(i) => Display::fmt(i, f),
            Value::Int64(i) => Display::fmt(i, f),
            Value::Float32(n) => Display::fmt(n, f),
            Value::Float(n) => Display::fmt(n, f),
            Value::Decimal(d) => {
                f.write_str("Decimal(\"")?;
                Display::fmt(d, f)?;
                f.write_str("\"")
            },
            Value::ObjectId(o) => {
                f.write_str("ObjectId(\"")?;
                Display::fmt(o, f)?;
                f.write_str("\"")
            },
            Value::String(s) => {
                f.write_str(&format!("\"{}\"", s.replace("\"", "\\\"")))
            }
            Value::Date(d) => f.write_str(&format!("Date(\"{}\")", d.to_string())),
            Value::DateTime(d) => f.write_str(&format!("DateTime(\"{}\")", d.to_rfc3339_opts(SecondsFormat::Millis, true))),
            Value::Array(a) => {
                f.write_str(&("[".to_string() + a.iter().map(|v| format!("{v}")).join(", ").as_str() + "]"))
            }
            Value::Dictionary(m) => {
                f.write_str(&("{".to_string() + m.iter().map(|(k, v)| format!("\"{k}\": {}", format!("{v}"))).join(", ").as_str() + "}"))
            }
            Value::Range(r) => Display::fmt(r, f),
            Value::Tuple(t) => {
                f.write_str("(")?;
                for (i, v) in t.iter().enumerate() {
                    Display::fmt(v, f)?;
                    if i != t.len() - 1 {
                        f.write_str(", ")?;
                    }
                }
                if t.len() == 1 {
                    f.write_str(",")?;
                }
                f.write_str(")")
            }
            Value::InterfaceEnumVariant(e) => {
                Display::fmt(e, f)
            }
            Value::OptionVariant(o) => {
                f.write_str(&o.display)
            }
            Value::Regex(r) => {
                f.write_str("/")?;
                f.write_str(&format!("{}", r.as_str().replace("/", "\\/")))?;
                f.write_str("/")
            }
            Value::File(file) => Display::fmt(file, f),
            Value::ModelObject(model_object) => Display::fmt(model_object, f),
            Value::StructObject(struct_object) => Display::fmt(struct_object, f),
            Value::Pipeline(pipeline) => Display::fmt(pipeline, f),
            Value::Type(t) => Display::fmt(t, f),
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