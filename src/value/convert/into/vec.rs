use std::fmt::Display;
use teo_result::Error;
use crate::value::Value;

impl<T, E> TryFrom<Value> for Vec<T> where T: TryFrom<Value, Error=E>, Error: From<E> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(vec) => {
                let mut result = vec![];
                for v in vec {
                    result.push(v.try_into()?);
                }
                Ok(result)
            },
            _ => Err(Error::new(format!("Cannot convert into array: {}", value))),
        }
    }
}

impl<'a, T, E> TryFrom<&'a Value> for Vec<T> where T: TryFrom<&'a Value, Error=E>, Error: From<E> {
    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(vec) => {
                let mut result = vec![];
                for v in vec {
                    result.push(v.try_into()?);
                }
                Ok(result)
            },
            _ => Err(Error::new(format!("Cannot convert into Vec<T>: {}", value))),
        }
    }
}

impl<T, E> TryFrom<Value> for Option<Vec<T>> where T: TryFrom<Value, Error=E> + Clone, Error: From<E> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Array(vec) => {
                let mut result = vec![];
                for v in vec {
                    result.push(v.try_into()?);
                }
                Ok(Some(result))
            }
            _ => Err(Error::new(format!("Cannot convert into Option<Vec<T>>: {}", value))),
        }
    }
}

impl<T, E> TryFrom<&Value> for Option<Vec<T>> where T: TryFrom<Value, Error=E> + Clone, Error: From<E> {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Array(vec) => {
                let mut result = vec![];
                for v in vec {
                    result.push(v.clone().try_into()?);
                }
                Ok(Some(result))
            }
            _ => Err(Error::new(format!("Cannot convert into Option<Vec<T>>: {}", value))),
        }
    }
}

impl TryFrom<&Value> for &Vec<Value> {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(inner) => Ok(inner),
            _ => Err(Error::new(format!("cannot convert to &Vec<Value>: {}", value)))
        }
    }
}