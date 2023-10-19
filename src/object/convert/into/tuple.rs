use teo_teon::Value;
use crate::error::Error;
use crate::object::Object;

impl<T0, T1> TryFrom<Object> for (T0, T1) where (T0, T1): TryFrom<Value> {

    type Error = Error;

    fn try_from(ref value: Object) -> std::result::Result<Self, Self::Error> {
        let teon: Value = value.try_into()?;
        match teon.try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(format!("object is not Tuple or in wrong type: {:?}", value)))
        }
    }
}