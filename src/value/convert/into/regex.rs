use regex::Regex;
use teo_result::Error;
use crate::value::Value;

impl TryFrom<Value> for Regex {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Regex(s) => Ok(s),
            _ => Err(Error::new(format!("cannot convert to Regex: {}", value))),
        }
    }
}

impl TryFrom<&Value> for Regex {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Regex(s) => Ok(s.clone()),
            _ => Err(Error::new(format!("cannot convert to Regex: {}", value))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for &'a Regex {
    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Regex(s) => Ok(s),
            _ => Err(Error::new(format!("cannot convert to &Regex: {}", value))),
        }
    }
}

impl TryInto<Option<Regex>> for Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<Regex>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Regex(s) => Ok(Some(s)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<Regex>", self.type_hint()))),
        }
    }
}

impl TryInto<Option<Regex>> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<Regex>, Self::Error> {
        self.clone().try_into()
    }
}

impl<'a> TryInto<Option<&'a Regex>> for &'a Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<&'a Regex>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Regex(s) => Ok(Some(s)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<&Regex>", self.type_hint()))),
        }
    }
}