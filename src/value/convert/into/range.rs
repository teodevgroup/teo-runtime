use teo_result::Error;
use crate::value::range::Range;
use crate::value::Value;

impl TryFrom<Value> for Range {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Range(s) => Ok(s),
            _ => Err(Error::new(format!("Cannot convert {} into Range", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for Range {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Range(s) => Ok(s.clone()),
            _ => Err(Error::new(format!("Cannot convert {} into Range", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for &'a Range {
    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Range(s) => Ok(s),
            _ => Err(Error::new(format!("Cannot convert {} into &Range", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for Option<Range> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Range(s) => Ok(Some(s)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<Range>", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for Option<Range> {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        value.clone().try_into()
    }
}

impl<'a> TryFrom<&'a Value> for Option<&'a Range> {
    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Range(s) => Ok(Some(s)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<&Range>", value.type_hint()))),
        }
    }
}
