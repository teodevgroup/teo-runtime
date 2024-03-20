use std::fmt::Display;
use crate::value::Value;
use teo_result::Error;
use crate::object::Object;

impl<T0, T1> TryFrom<Object> for (T0, T1) where (T0, T1): TryFrom<Value>, <(T0, T1) as TryFrom<Value>>::Error: Display {

    type Error = Error;

    fn try_from(ref value: Object) -> std::result::Result<Self, Self::Error> {
        let teon: Value = value.try_into()?;
        match teon.try_into() {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::new(format!("{}", e))),
        }
    }
}